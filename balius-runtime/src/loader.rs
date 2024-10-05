use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, Mutex},
};

use wasmtime::{component::Component, component::Linker, Engine, Store};

use crate::{adapter::Adapter, router::Router, wit};

struct LoadedWorker {
    store: Store<Adapter>,
    instance: wit::Worker,
}

type WorkerMap = HashMap<String, LoadedWorker>;

#[derive(Clone)]
pub struct Loader {
    engine: Engine,
    linker: Linker<Adapter>,
    router: Router,
    loaded: Arc<Mutex<WorkerMap>>,
}

impl Loader {
    pub fn new(router: Router) -> Result<Self, super::Error> {
        let engine = Default::default();

        let mut linker = Linker::new(&engine);
        wit::balius::app::driver::add_to_linker(&mut linker, |state: &mut Adapter| state)?;
        wit::balius::app::kv::add_to_linker(&mut linker, |state: &mut Adapter| state)?;
        wit::balius::app::submit::add_to_linker(&mut linker, |state: &mut Adapter| state)?;
        wit::balius::app::ledger::add_to_linker(&mut linker, |state: &mut Adapter| state)?;

        Ok(Self {
            engine,
            loaded: Default::default(),
            linker,
            router,
        })
    }

    pub fn register_worker(
        &mut self,
        id: &str,
        wasm_path: impl AsRef<Path>,
        config: serde_json::Value,
    ) -> wasmtime::Result<()> {
        let component = Component::from_file(&self.engine, wasm_path)?;

        let mut store = Store::new(
            &self.engine,
            Adapter::new(id.to_owned(), self.router.clone()),
        );

        let instance = wit::Worker::instantiate(&mut store, &component, &self.linker)?;
        let config = serde_json::to_vec(&config).unwrap();
        instance.call_init(&mut store, &config)?;

        self.loaded
            .lock()
            .unwrap()
            .insert(id.to_owned(), LoadedWorker { store, instance });

        Ok(())
    }

    pub fn dispatch_event(
        &self,
        worker: &str,
        channel: u32,
        event: &wit::Event,
    ) -> Result<wit::Response, super::Error> {
        let mut lock = self.loaded.lock().unwrap();

        let worker = lock
            .get_mut(worker)
            .ok_or(super::Error::WorkerNotFound(worker.to_string()))?;

        let result = worker
            .instance
            .call_handle(&mut worker.store, channel, event)?;

        let response = result.map_err(|err| super::Error::Handle(err.code, err.message))?;

        Ok(response)
    }
}
