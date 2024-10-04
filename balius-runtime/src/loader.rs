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
    ) -> wasmtime::Result<()> {
        let component = Component::from_file(&self.engine, wasm_path)?;

        let mut store = Store::new(
            &self.engine,
            Adapter::new(id.to_owned(), self.router.clone()),
        );

        let instance = wit::Worker::instantiate(&mut store, &component, &self.linker)?;
        //instance.call_init(&mut store, &vec![])?;
        instance.call_init(&mut store, &hex::decode("7b22667265655f76616c696461746f72223a7b227265665f74786f223a7b2268617368223a2266376433383337373135363830663361313730653939636432303262373236383432643937663832633035616638666364313830353363363465333365633466222c22696e646578223a307d2c2268617368223a226566376131636562623264633764653838346464663832663866636263393166653937353064636438633132656337363433613939626265222c2261646472657373223a22616464723171783266787632756d796874746b78797870387830646c706474336b3663776e673570786a336a687379647a6572336e306433766c6c6d797177737835776b7463643863633373713833356c75376472763278776c32777977666773653335613378227d7d").unwrap())?;

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

        let response = result.map_err(|code| super::Error::Handle(code))?;

        Ok(response)
    }
}
