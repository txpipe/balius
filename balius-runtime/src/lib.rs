use kv::KvHost;
use ledgers::LedgerHost;
use logging::LoggerHost;
use router::Router;
use sign::SignerHost;
use std::{collections::HashMap, io::Read, path::Path, sync::Arc};
use thiserror::Error;
use tokio::sync::Mutex;
use tracing::{debug, info, warn};
use utxorpc::spec::sync::BlockRef;
use wasmtime::component::HasSelf;

pub mod wit {
    wasmtime::component::bindgen!({
        path: "./wit",
        async: true,
        tracing: true,
    });
}

mod metrics;
mod router;

// implementations
pub mod drivers;
pub mod http;
pub mod kv;
pub mod ledgers;
pub mod logging;
pub mod sign;
pub mod store;
pub mod submit;

pub use store::{AtomicUpdateTrait, Store, StoreTrait};
pub use wit::Response;

pub type WorkerId = String;

#[derive(Error, Debug)]
pub enum Error {
    #[error("wasm error {0}")]
    Wasm(wasmtime::Error),

    #[error("store error {0}")]
    Store(String),

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

    #[error("failed to interact with WASM object: {0}")]
    ObjectStoreError(object_store::Error),

    #[error("failed to interact with local WASM file: {0}")]
    IoError(std::io::Error),

    #[error("kv error {0}")]
    KvError(String),
}

impl From<wasmtime::Error> for Error {
    fn from(value: wasmtime::Error) -> Self {
        Self::Wasm(value)
    }
}

impl From<redb::Error> for Error {
    fn from(value: redb::Error) -> Self {
        Self::Store(value.to_string())
    }
}

impl From<redb::DatabaseError> for Error {
    fn from(value: redb::DatabaseError) -> Self {
        Self::Store(value.to_string())
    }
}

impl From<redb::TransactionError> for Error {
    fn from(value: redb::TransactionError) -> Self {
        Self::Store(value.to_string())
    }
}

impl From<redb::TableError> for Error {
    fn from(value: redb::TableError) -> Self {
        Self::Store(value.to_string())
    }
}

impl From<redb::CommitError> for Error {
    fn from(value: redb::CommitError) -> Self {
        Self::Store(value.to_string())
    }
}

impl From<redb::StorageError> for Error {
    fn from(value: redb::StorageError) -> Self {
        Self::Store(value.to_string())
    }
}

impl From<pallas::ledger::addresses::Error> for Error {
    fn from(value: pallas::ledger::addresses::Error) -> Self {
        Self::BadAddress(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<object_store::Error> for Error {
    fn from(value: object_store::Error) -> Self {
        match value {
            object_store::Error::Generic { store, source } => {
                Self::Config(format!("Failed to parse url: {store}, {source}"))
            }
            object_store::Error::NotFound { path: _, source } => {
                Self::WorkerNotFound(source.to_string())
            }
            x => Self::ObjectStoreError(x),
        }
    }
}

pub type BlockSlot = u64;
pub type BlockHash = pallas::crypto::hash::Hash<32>;

pub enum ChainPoint {
    Cardano(utxorpc::spec::sync::BlockRef),
}

pub type LogSeq = u64;

pub enum TxInput {
    Cardano(utxorpc::spec::cardano::TxInput),
}

impl TxInput {
    pub fn to_bytes(&self) -> Vec<u8> {
        use prost::Message;

        match self {
            Self::Cardano(tx_input) => tx_input.encode_to_vec(),
        }
    }

    pub fn address(&self) -> Option<Vec<u8>> {
        match self {
            Self::Cardano(tx_input) => tx_input.as_output.as_ref().map(|o| o.address.to_vec()),
        }
    }
}

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

    pub fn address(&self) -> Option<Vec<u8>> {
        match self {
            Self::Cardano(utxo) => Some(utxo.address.to_vec()),
        }
    }
}

pub enum Tx {
    Cardano(utxorpc::spec::cardano::Tx),
}

impl Tx {
    pub fn hash(&self) -> Vec<u8> {
        match self {
            Self::Cardano(tx) => tx.hash.clone().into(),
        }
    }
    pub fn inputs(&self) -> Vec<TxInput> {
        match self {
            Self::Cardano(tx) => tx
                .inputs
                .iter()
                .map(|i| TxInput::Cardano(i.clone()))
                .collect(),
        }
    }
    pub fn outputs(&self) -> Vec<Utxo> {
        match self {
            Self::Cardano(tx) => tx
                .outputs
                .iter()
                .map(|o| Utxo::Cardano(o.clone()))
                .collect(),
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        use prost::Message;

        match self {
            Self::Cardano(tx) => tx.encode_to_vec(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Block {
    Cardano(utxorpc::spec::cardano::Block),
}

impl Block {
    pub fn hash(&self) -> Vec<u8> {
        match self {
            Self::Cardano(block) => block.header.as_ref().unwrap().hash.clone().into(),
        }
    }
    pub fn height(&self) -> u64 {
        match self {
            Self::Cardano(block) => block.header.as_ref().unwrap().height,
        }
    }
    pub fn slot(&self) -> u64 {
        match self {
            Self::Cardano(block) => block.header.as_ref().unwrap().slot,
        }
    }
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
    pub ledger: Option<ledgers::LedgerHost>,
    pub logging: Option<logging::LoggerHost>,
    pub kv: Option<kv::KvHost>,
    pub sign: Option<sign::SignerHost>,
    pub submit: Option<submit::Submit>,
    pub http: Option<http::Http>,
}

impl wit::balius::app::driver::Host for WorkerState {
    async fn register_channel(
        &mut self,
        id: u32,
        pattern: wit::balius::app::driver::EventPattern,
    ) -> () {
        self.router.register_channel(id, &pattern);
    }

    async fn register_signer(&mut self, name: String, algorithm: String) -> Vec<u8> {
        let signer = self.sign.as_mut().expect("No sign interface defined.");
        signer.add_key(name, algorithm).await
    }
}

struct LoadedWorker {
    wasm_store: wasmtime::Store<WorkerState>,
    instance: wit::Worker,
    cursor: Option<LogSeq>,
    metrics: Arc<metrics::Metrics>,
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
        let worker_id = self.wasm_store.data().worker_id.clone();
        let block_hash = block.hash();
        let block_height = block.height();
        let block_slot = block.slot();
        for tx in block.txs() {
            let tx_hash = tx.hash();
            let channels = self.wasm_store.data().router.find_tx_targets(&tx);
            if !channels.is_empty() {
                let event = wit::Event::Tx(wit::balius::app::driver::Tx {
                    block: wit::balius::app::driver::BlockRef {
                        block_hash: block_hash.clone(),
                        block_height,
                        block_slot,
                    },
                    body: tx.to_bytes(),
                    hash: tx_hash.clone(),
                });
                for channel in channels {
                    self.metrics.tx_handled(&worker_id);
                    self.acknowledge_event(channel, &event).await?;
                }
            }

            for (index, utxo) in tx.outputs().into_iter().enumerate() {
                let channels = self.wasm_store.data().router.find_utxo_targets(&utxo);
                if channels.is_empty() {
                    continue;
                }

                let event = wit::Event::Utxo(wit::balius::app::driver::Utxo {
                    block: wit::balius::app::driver::BlockRef {
                        block_hash: block_hash.clone(),
                        block_height,
                        block_slot,
                    },
                    body: utxo.to_bytes(),
                    ref_: wit::balius::app::driver::TxoRef {
                        tx_hash: tx_hash.clone(),
                        txo_index: index as u32,
                    },
                });

                for channel in channels {
                    self.metrics.utxo_handled(&worker_id);
                    self.acknowledge_event(channel, &event).await?;
                }
            }
        }

        Ok(())
    }

    async fn undo_block(&mut self, block: &Block) -> Result<(), Error> {
        let worker_id = self.wasm_store.data().worker_id.clone();
        let block_hash = block.hash();
        let block_height = block.height();
        let block_slot = block.slot();
        for tx in block.txs() {
            let tx_hash = tx.hash();
            for (index, utxo) in tx.outputs().into_iter().enumerate().rev() {
                let channels = self.wasm_store.data().router.find_utxo_targets(&utxo);
                if channels.is_empty() {
                    continue;
                }

                let event = wit::Event::UtxoUndo(wit::balius::app::driver::Utxo {
                    block: wit::balius::app::driver::BlockRef {
                        block_hash: block_hash.clone(),
                        block_height,
                        block_slot,
                    },
                    body: utxo.to_bytes(),
                    ref_: wit::balius::app::driver::TxoRef {
                        tx_hash: tx_hash.clone(),
                        txo_index: index as u32,
                    },
                });

                for channel in channels {
                    self.metrics.undo_utxo_handled(&worker_id);
                    self.acknowledge_event(channel, &event).await?;
                }
            }

            let channels = self.wasm_store.data().router.find_tx_targets(&tx);
            if !channels.is_empty() {
                let event = wit::Event::TxUndo(wit::balius::app::driver::Tx {
                    block: wit::balius::app::driver::BlockRef {
                        block_hash: block_hash.clone(),
                        block_height,
                        block_slot,
                    },
                    body: tx.to_bytes(),
                    hash: tx_hash.clone(),
                });
                for channel in channels {
                    self.metrics.undo_tx_handled(&worker_id);
                    self.acknowledge_event(channel, &event).await?;
                }
            }
        }

        Ok(())
    }

    async fn apply_chain(
        &mut self,
        undo_blocks: &Vec<Block>,
        next_block: &Block,
    ) -> Result<(), Error> {
        for block in undo_blocks {
            self.undo_block(block).await?;
        }

        self.apply_block(next_block).await
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
    logging: Option<logging::Logger>,
    kv: Option<kv::Kv>,
    sign: Option<sign::Signer>,
    submit: Option<submit::Submit>,
    http: Option<http::Http>,

    metrics: Arc<metrics::Metrics>,
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
            .flat_map(|w| w.cursor)
            .min();

        if let Some(seq) = lowest_seq {
            debug!(lowest_seq, "found lowest seq");
            return self.store.find_chain_point(seq).await;
        }

        Ok(None)
    }

    pub async fn register_worker(
        &self,
        id: &str,
        wasm: &[u8],
        config: serde_json::Value,
    ) -> Result<(), Error> {
        let component = wasmtime::component::Component::new(&self.engine, wasm)?;

        let mut wasm_store = wasmtime::Store::new(
            &self.engine,
            WorkerState {
                worker_id: id.to_owned(),
                router: Router::new(),
                ledger: self
                    .ledger
                    .as_ref()
                    .map(|l| LedgerHost::new(id, l, &self.metrics)),
                logging: self
                    .logging
                    .as_ref()
                    .map(|kv| LoggerHost::new(id, kv, &self.metrics)),
                kv: self
                    .kv
                    .as_ref()
                    .map(|kv| KvHost::new(id, kv, &self.metrics)),
                sign: self
                    .sign
                    .as_ref()
                    .map(|s| SignerHost::new(id, s, &self.metrics)),
                submit: self.submit.clone(),
                http: self.http.clone(),
            },
        );

        let instance =
            wit::Worker::instantiate_async(&mut wasm_store, &component, &self.linker).await?;

        let config = serde_json::to_vec(&config).unwrap();
        instance.call_init(&mut wasm_store, &config).await?;

        let cursor = self.store.get_worker_cursor(id).await?;
        debug!(cursor, id, "found cursor for worker");

        self.loaded.lock().await.insert(
            id.to_owned(),
            LoadedWorker {
                wasm_store,
                instance,
                cursor,
                metrics: self.metrics.clone(),
            },
        );

        Ok(())
    }

    /// Register worker into runtime using URL.
    ///
    /// Will download bytes from URL and interpret it as WASM. URL support is
    /// determined by build features passed on to the [object_store](https://docs.rs/crate/object_store/latest) crate.
    pub async fn register_worker_from_url(
        &self,
        id: &str,
        url: &url::Url,
        config: serde_json::Value,
    ) -> Result<(), Error> {
        let (store, path) = object_store::parse_url(url)?;
        let bytes = store.get(&path).await?.bytes().await?;
        self.register_worker(id, &bytes, config).await
    }

    pub async fn register_worker_from_file(
        &self,
        id: &str,
        wasm_path: impl AsRef<Path>,
        config: serde_json::Value,
    ) -> Result<(), Error> {
        let mut file = std::fs::File::open(wasm_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        self.register_worker(id, &buffer, config).await
    }

    pub async fn remove_worker(&self, id: &str) -> Result<(), Error> {
        match self.loaded.lock().await.remove(id) {
            Some(_) => {
                info!(worker = id, "Successfully removed worker from runtime.")
            }
            None => {
                warn!(worker = id, "Worker not found, skipping remove.")
            }
        }

        Ok(())
    }

    pub async fn handle_chain(
        &mut self,
        undo_blocks: &Vec<Block>,
        next_block: &Block,
    ) -> Result<(), Error> {
        info!("applying block");

        let log_seq = self.store.write_ahead(undo_blocks, next_block).await?;

        let mut workers = self.loaded.lock().await;

        let mut store_update = self.store.start_atomic_update(log_seq).await?;

        for (_, worker) in workers.iter_mut() {
            worker.apply_chain(undo_blocks, next_block).await?;
            store_update
                .update_worker_cursor(&worker.wasm_store.data().worker_id)
                .await?;
        }

        store_update.commit().await?;

        Ok(())
    }

    pub async fn handle_request(
        &self,
        worker_id: &str,
        method: &str,
        params: Vec<u8>,
    ) -> Result<wit::Response, Error> {
        let mut lock = self.loaded.lock().await;

        let worker = lock
            .get_mut(worker_id)
            .ok_or(Error::WorkerNotFound(worker_id.to_string()))?;

        let channel = worker
            .wasm_store
            .data()
            .router
            .find_request_target(method)?;

        let evt = wit::Event::Request(params);

        let result = worker.dispatch_event(channel, &evt).await;
        self.metrics.request(worker_id, method, result.is_ok());
        result
    }
}

struct HasField<T>(std::marker::PhantomData<T>);
impl<T: 'static> wasmtime::component::HasData for HasField<T> {
    type Data<'a> = &'a mut T;
}

pub struct RuntimeBuilder {
    store: store::Store,
    engine: wasmtime::Engine,
    linker: wasmtime::component::Linker<WorkerState>,
    ledger: Option<ledgers::Ledger>,
    logging: Option<logging::Logger>,
    kv: Option<kv::Kv>,
    sign: Option<sign::Signer>,
    submit: Option<submit::Submit>,
    http: Option<http::Http>,
}

impl RuntimeBuilder {
    pub fn new(store: store::Store) -> Self {
        let mut config = wasmtime::Config::new();
        config.async_support(true);
        let engine = wasmtime::Engine::new(&config).unwrap();
        let mut linker = wasmtime::component::Linker::new(&engine);

        wit::balius::app::driver::add_to_linker::<_, HasSelf<_>>(
            &mut linker,
            |state: &mut WorkerState| state,
        )
        .unwrap();

        Self {
            store,
            engine,
            linker,
            ledger: None,
            logging: None,
            kv: None,
            sign: None,
            submit: None,
            http: None,
        }
    }

    pub fn with_ledger(mut self, ledger: ledgers::Ledger) -> Self {
        self.ledger = Some(ledger);

        wit::balius::app::ledger::add_to_linker::<_, HasField<_>>(
            &mut self.linker,
            |state: &mut WorkerState| state.ledger.as_mut().unwrap(),
        )
        .unwrap();

        self
    }

    pub fn with_kv(mut self, kv: kv::Kv) -> Self {
        wit::balius::app::kv::add_to_linker::<_, HasField<_>>(
            &mut self.linker,
            |state: &mut WorkerState| state.kv.as_mut().unwrap(),
        )
        .unwrap();
        self.kv = Some(kv);

        self
    }

    pub fn with_logger(mut self, logging: logging::Logger) -> Self {
        self.logging = Some(logging);

        wit::balius::app::logging::add_to_linker::<_, HasField<_>>(
            &mut self.linker,
            |state: &mut WorkerState| state.logging.as_mut().unwrap(),
        )
        .unwrap();

        self
    }

    pub fn with_signer(mut self, sign: sign::Signer) -> Self {
        self.sign = Some(sign);

        wit::balius::app::sign::add_to_linker::<_, HasField<_>>(
            &mut self.linker,
            |state: &mut WorkerState| state.sign.as_mut().unwrap(),
        )
        .unwrap();

        self
    }

    pub fn with_submit(mut self, submit: submit::Submit) -> Self {
        self.submit = Some(submit);

        wit::balius::app::submit::add_to_linker::<_, HasField<_>>(
            &mut self.linker,
            |state: &mut WorkerState| state.submit.as_mut().unwrap(),
        )
        .unwrap();

        self
    }

    pub fn with_http(mut self, http: http::Http) -> Self {
        self.http = Some(http);
        wit::balius::app::http::add_to_linker::<_, HasField<_>>(
            &mut self.linker,
            |state: &mut WorkerState| state.http.as_mut().unwrap(),
        )
        .unwrap();

        self
    }

    pub fn build(self) -> Result<Runtime, Error> {
        let mut this = self;
        if this.logging.is_none() {
            this = this.with_logger(logging::Logger::Silent);
        }

        let RuntimeBuilder {
            store,
            engine,
            linker,
            ledger,
            logging,
            kv,
            sign,
            submit,
            http,
        } = this;

        Ok(Runtime {
            metrics: Default::default(),
            loaded: Default::default(),
            engine,
            linker,
            store,
            ledger,
            logging,
            kv,
            sign,
            submit,
            http,
        })
    }
}
