use pallas::ledger::traverse::MultiEraBlock;
use serde_json::json;
use std::{
    collections::{HashMap, HashSet},
    path::Path,
    sync::Arc,
};
use thiserror::Error;
use tokio::sync::Mutex;

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

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct ChainPoint(pub BlockSlot, pub BlockHash);

pub type LogSeq = u64;

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
        self.router.register_channel(&self.worker_id, id, &pattern);
    }
}

struct LoadedWorker {
    store: wasmtime::Store<WorkerState>,
    instance: wit::Worker,
}

type WorkerMap = HashMap<String, LoadedWorker>;

#[derive(Clone)]
pub struct Runtime {
    engine: wasmtime::Engine,
    linker: wasmtime::component::Linker<WorkerState>,
    loaded: Arc<Mutex<WorkerMap>>,

    router: router::Router,
    store: store::Store,
    ledger: Option<ledgers::Ledger>,
    kv: Option<kv::Kv>,
    submit: Option<submit::Submit>,
}

impl Runtime {
    pub fn builder(store: store::Store) -> RuntimeBuilder {
        RuntimeBuilder::new(store)
    }

    pub fn cursor(&self) -> Result<Option<LogSeq>, Error> {
        let cursor = self.store.lowest_cursor()?;

        Ok(cursor)
    }

    pub async fn register_worker(
        &mut self,
        id: &str,
        wasm_path: impl AsRef<Path>,
        config: serde_json::Value,
    ) -> Result<(), Error> {
        let component = wasmtime::component::Component::from_file(&self.engine, wasm_path)?;

        let mut store = wasmtime::Store::new(
            &self.engine,
            WorkerState {
                worker_id: id.to_owned(),
                router: self.router.clone(),
                ledger: self.ledger.clone(),
                kv: self.kv.clone(),
                submit: self.submit.clone(),
            },
        );

        let instance = wit::Worker::instantiate_async(&mut store, &component, &self.linker).await?;

        let config = serde_json::to_vec(&config).unwrap();
        instance.call_init(&mut store, &config).await?;

        self.loaded
            .lock()
            .await
            .insert(id.to_owned(), LoadedWorker { store, instance });

        Ok(())
    }

    pub async fn dispatch_event(
        &self,
        worker: &str,
        channel: u32,
        event: &wit::Event,
    ) -> Result<wit::Response, Error> {
        let mut lock = self.loaded.lock().await;

        let worker = lock
            .get_mut(worker)
            .ok_or(Error::WorkerNotFound(worker.to_string()))?;

        let result = worker
            .instance
            .call_handle(&mut worker.store, channel, event)
            .await?;

        let response = result.map_err(|err| Error::Handle(err.code, err.message))?;

        Ok(response)
    }

    async fn fire_and_forget(
        &self,
        event: &wit::Event,
        targets: HashSet<router::Target>,
    ) -> Result<(), Error> {
        for target in targets {
            let result = self
                .dispatch_event(&target.worker, target.channel, event)
                .await;

            match result {
                Ok(wit::Response::Acknowledge) => {
                    tracing::debug!(worker = target.worker, "worker acknowledge");
                }
                Ok(_) => {
                    tracing::warn!(worker = target.worker, "worker returned unexpected data");
                }
                Err(Error::Handle(code, message)) => {
                    tracing::warn!(code, message);
                }
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }

    pub async fn apply_block(
        &self,
        block: &MultiEraBlock<'_>,
        wal_seq: LogSeq,
    ) -> Result<(), Error> {
        for tx in block.txs() {
            for utxo in tx.outputs() {
                let targets = self.router.find_utxo_targets(&utxo)?;
                let event = wit::Event::Utxo(utxo.encode());

                self.fire_and_forget(&event, targets).await?;
            }
        }

        Ok(())
    }

    pub fn undo_block(&self, block: &MultiEraBlock, wal_seq: LogSeq) -> Result<(), Error> {
        Ok(())
    }

    pub async fn handle_request(
        &self,
        worker: &str,
        method: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, Error> {
        let target = self.router.find_request_target(worker, method)?;

        let evt = wit::Event::Request(serde_json::to_vec(&params).unwrap());

        let reply = self
            .dispatch_event(&target.worker, target.channel, &evt)
            .await?;

        let json = match reply {
            wit::Response::Acknowledge => json!({}),
            wit::Response::Json(x) => serde_json::from_slice(&x).unwrap(),
            wit::Response::Cbor(x) => json!({ "cbor": x }),
            wit::Response::PartialTx(x) => json!({ "tx": x }),
        };

        Ok(json)
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
            router: router::Router::new(),
            engine,
            linker,
            store,
            ledger,
            kv,
            submit,
        })
    }
}
