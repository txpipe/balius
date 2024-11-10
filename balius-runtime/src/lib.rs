use router::Router;
use std::{collections::HashMap, path::Path, sync::Arc};
use thiserror::Error;
use tokio::sync::Mutex;
use tracing::{debug, info, warn};
use utxorpc::spec::sync::BlockRef;

mod wit {
    wasmtime::component::bindgen!({
        path:"../wit",
        async: true,
        tracing: true,
    });
}

mod router;
mod store;

// implementations
pub mod drivers;
pub mod kv;
pub mod ledgers;
pub mod submit;

pub use store::Store;

pub type WorkerId = String;

#[derive(Error, Debug)]
pub enum Error {
    #[error("wasm error {0}")]
    Wasm(wasmtime::Error),

    #[error("store error {0}")]
    Store(redb::Error),

    #[error("worker not found '{0}'")]
    WorkerNotFound(WorkerId),

    #[error("worker failed to handle event (code: '{0}', message: '{1}')")]
    Handle(u32, String),

    #[error("no target available to solve request")]
    NoTarget,

    #[error("more than one target available to solve request")]
    AmbiguousTarget,

    #[error("address in block failed to parse")]
    BadAddress(pallas::ledger::addresses::Error),

    #[error("ledger error: {0}")]
    Ledger(String),

    #[error("config error: {0}")]
    Config(String),

    #[error("driver error: {0}")]
    Driver(String),
}

impl From<wasmtime::Error> for Error {
    fn from(value: wasmtime::Error) -> Self {
        Self::Wasm(value)
    }
}

impl From<redb::Error> for Error {
    fn from(value: redb::Error) -> Self {
        Self::Store(value)
    }
}

impl From<redb::DatabaseError> for Error {
    fn from(value: redb::DatabaseError) -> Self {
        Self::Store(value.into())
    }
}

impl From<redb::TransactionError> for Error {
    fn from(value: redb::TransactionError) -> Self {
        Self::Store(value.into())
    }
}

impl From<redb::TableError> for Error {
    fn from(value: redb::TableError) -> Self {
        Self::Store(value.into())
    }
}

impl From<redb::CommitError> for Error {
    fn from(value: redb::CommitError) -> Self {
        Self::Store(value.into())
    }
}

impl From<redb::StorageError> for Error {
    fn from(value: redb::StorageError) -> Self {
        Self::Store(value.into())
    }
}

impl From<pallas::ledger::addresses::Error> for Error {
    fn from(value: pallas::ledger::addresses::Error) -> Self {
        Self::BadAddress(value.into())
    }
}

pub type BlockSlot = u64;
pub type BlockHash = pallas::crypto::hash::Hash<32>;

pub enum ChainPoint {
    Cardano(utxorpc::spec::sync::BlockRef),
}

pub type LogSeq = u64;

pub enum Utxo {
    Cardano(utxorpc::spec::cardano::TxOutput),
}

impl Utxo {
    pub fn to_bytes(&self) -> Vec<u8> {
        use prost::Message;

        match self {
            Self::Cardano(utxo) => utxo.encode_to_vec(),
        }
    }
}

pub enum Tx {
    Cardano(utxorpc::spec::cardano::Tx),
}

impl Tx {
    pub fn outputs(&self) -> Vec<Utxo> {
        match self {
            Self::Cardano(tx) => tx
                .outputs
                .iter()
                .map(|o| Utxo::Cardano(o.clone()))
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Block {
    Cardano(utxorpc::spec::cardano::Block),
}

impl Block {
    pub fn txs(&self) -> Vec<Tx> {
        match self {
            Self::Cardano(block) => block
                .body
                .iter()
                .flat_map(|b| b.tx.iter())
                .map(|t| Tx::Cardano(t.clone()))
                .collect(),
        }
    }

    pub fn chain_point(&self) -> ChainPoint {
        match self {
            Self::Cardano(block) => ChainPoint::Cardano(BlockRef {
                index: block.header.as_ref().unwrap().slot,
                hash: block.header.as_ref().unwrap().hash.clone(),
            }),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        use prost::Message;

        match self {
            Self::Cardano(block) => block.encode_to_vec(),
        }
    }

    pub fn from_bytes(data: &[u8]) -> Self {
        use prost::Message;

        Self::Cardano(utxorpc::spec::cardano::Block::decode(data).unwrap())
    }
}

struct WorkerState {
    pub worker_id: String,
    pub router: router::Router,
    pub ledger: Option<ledgers::Ledger>,
    pub kv: Option<kv::Kv>,
    pub submit: Option<submit::Submit>,
}

#[async_trait::async_trait]
impl wit::balius::app::driver::Host for WorkerState {
    async fn register_channel(
        &mut self,
        id: u32,
        pattern: wit::balius::app::driver::EventPattern,
    ) -> () {
        self.router.register_channel(id, &pattern);
    }
}

struct LoadedWorker {
    wasm_store: wasmtime::Store<WorkerState>,
    instance: wit::Worker,
    cursor: Option<LogSeq>,
}

impl LoadedWorker {
    pub async fn dispatch_event(
        &mut self,
        channel: u32,
        event: &wit::Event,
    ) -> Result<wit::Response, Error> {
        self.instance
            .call_handle(&mut self.wasm_store, channel, event)
            .await?
            .map_err(|err| Error::Handle(err.code, err.message))
    }

    async fn acknowledge_event(&mut self, channel: u32, event: &wit::Event) -> Result<(), Error> {
        let result = self.dispatch_event(channel, event).await;

        match result {
            Ok(wit::Response::Acknowledge) => {
                tracing::debug!("worker acknowledge");
            }
            Ok(_) => {
                tracing::warn!("worker returned unexpected data");
            }
            Err(Error::Handle(code, message)) => {
                tracing::warn!(code, message);
            }
            Err(e) => return Err(e),
        }

        Ok(())
    }

    async fn apply_block(&mut self, block: &Block) -> Result<(), Error> {
        for tx in block.txs() {
            for utxo in tx.outputs() {
                let channels = self.wasm_store.data().router.find_utxo_targets(&utxo)?;

                let event = wit::Event::Utxo(utxo.to_bytes());

                for channel in channels {
                    self.acknowledge_event(channel, &event).await?;
                }
            }
        }

        Ok(())
    }

    async fn undo_block(&mut self, block: &Block) -> Result<(), Error> {
        for tx in block.txs() {
            for utxo in tx.outputs() {
                let channels = self.wasm_store.data().router.find_utxo_targets(&utxo)?;

                let event = wit::Event::UtxoUndo(utxo.to_bytes());

                for channel in channels {
                    self.acknowledge_event(channel, &event).await?;
                }
            }
        }

        Ok(())
    }
}

type WorkerMap = HashMap<String, LoadedWorker>;

#[derive(Clone)]
pub struct Runtime {
    engine: wasmtime::Engine,
    linker: wasmtime::component::Linker<WorkerState>,
    loaded: Arc<Mutex<WorkerMap>>,

    store: store::Store,
    ledger: Option<ledgers::Ledger>,
    kv: Option<kv::Kv>,
    submit: Option<submit::Submit>,
}

impl Runtime {
    pub fn builder(store: store::Store) -> RuntimeBuilder {
        RuntimeBuilder::new(store)
    }

    pub async fn chain_cursor(&self) -> Result<Option<ChainPoint>, Error> {
        let lowest_seq = self
            .loaded
            .lock()
            .await
            .values()
            .map(|w| w.cursor)
            .flatten()
            .min();

        if let Some(seq) = lowest_seq {
            debug!(lowest_seq, "found lowest seq");

            let entry = self.store.get_entry(seq)?;
            let block = Block::from_bytes(&entry.unwrap().next_block);
            return Ok(Some(block.chain_point()));
        }

        Ok(None)
    }

    pub async fn register_worker(
        &mut self,
        id: &str,
        wasm_path: impl AsRef<Path>,
        config: serde_json::Value,
    ) -> Result<(), Error> {
        let component = wasmtime::component::Component::from_file(&self.engine, wasm_path)?;

        let mut wasm_store = wasmtime::Store::new(
            &self.engine,
            WorkerState {
                worker_id: id.to_owned(),
                router: Router::new(),
                ledger: self.ledger.clone(),
                kv: self.kv.clone(),
                submit: self.submit.clone(),
            },
        );

        let instance =
            wit::Worker::instantiate_async(&mut wasm_store, &component, &self.linker).await?;

        let config = serde_json::to_vec(&config).unwrap();
        instance.call_init(&mut wasm_store, &config).await?;

        let cursor = self.store.get_worker_cursor(id)?;
        debug!(cursor, id, "found cursor for worker");

        self.loaded.lock().await.insert(
            id.to_owned(),
            LoadedWorker {
                wasm_store,
                instance,
                cursor,
            },
        );

        Ok(())
    }

    pub async fn handle_chain(
        &mut self,
        undo_blocks: &Vec<Block>,
        next_block: &Block,
    ) -> Result<(), Error> {
        info!("applying block");

        let log_seq = self.store.write_ahead(undo_blocks, next_block)?;

        let mut lock = self.loaded.lock().await;

        let mut atomic_update = self.store.start_atomic_update(log_seq)?;

        for (_, worker) in lock.iter_mut() {
            for undo_block in undo_blocks {
                worker.undo_block(undo_block).await?;
            }

            worker.apply_block(next_block).await?;

            atomic_update.update_worker_cursor(&worker.wasm_store.data().worker_id)?;
        }

        atomic_update.commit()?;

        Ok(())
    }

    pub async fn handle_request(
        &self,
        worker: &str,
        method: &str,
        params: Vec<u8>,
    ) -> Result<wit::Response, Error> {
        let mut lock = self.loaded.lock().await;

        let worker = lock
            .get_mut(worker)
            .ok_or(Error::WorkerNotFound(worker.to_string()))?;

        let channel = worker
            .wasm_store
            .data()
            .router
            .find_request_target(method)?;

        let evt = wit::Event::Request(params);

        worker.dispatch_event(channel, &evt).await
    }
}

pub struct RuntimeBuilder {
    store: store::Store,
    engine: wasmtime::Engine,
    linker: wasmtime::component::Linker<WorkerState>,
    ledger: Option<ledgers::Ledger>,
    kv: Option<kv::Kv>,
    submit: Option<submit::Submit>,
}

impl RuntimeBuilder {
    pub fn new(store: store::Store) -> Self {
        let mut config = wasmtime::Config::new();
        config.async_support(true);
        let engine = wasmtime::Engine::new(&config).unwrap();
        let mut linker = wasmtime::component::Linker::new(&engine);

        wit::balius::app::driver::add_to_linker(&mut linker, |state: &mut WorkerState| state)
            .unwrap();

        Self {
            store,
            engine,
            linker,
            ledger: None,
            kv: None,
            submit: None,
        }
    }

    pub fn with_ledger(mut self, ledger: ledgers::Ledger) -> Self {
        self.ledger = Some(ledger);

        wit::balius::app::ledger::add_to_linker(&mut self.linker, |state: &mut WorkerState| {
            state.ledger.as_mut().unwrap()
        })
        .unwrap();

        self
    }

    pub fn with_kv(mut self, kv: kv::Kv) -> Self {
        self.kv = Some(kv);

        wit::balius::app::kv::add_to_linker(&mut self.linker, |state: &mut WorkerState| {
            state.kv.as_mut().unwrap()
        })
        .unwrap();

        self
    }

    pub fn with_submit(mut self, submit: submit::Submit) -> Self {
        self.submit = Some(submit);

        wit::balius::app::submit::add_to_linker(&mut self.linker, |state: &mut WorkerState| {
            state.submit.as_mut().unwrap()
        })
        .unwrap();

        self
    }

    pub fn build(self) -> Result<Runtime, Error> {
        let RuntimeBuilder {
            store,
            engine,
            linker,
            ledger,
            kv,
            submit,
        } = self;

        Ok(Runtime {
            loaded: Default::default(),
            engine,
            linker,
            store,
            ledger,
            kv,
            submit,
        })
    }
}
