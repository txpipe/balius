pub mod bindings {
    wit_bindgen::generate!({
        pub_export_macro: true,
        default_bindings_module: "balius::bindings",
    });
}

use bindings::{balius::odk, Response};
use serde::Serialize;
use std::{
    collections::HashMap,
    sync::{LazyLock, RwLock},
};

pub trait Handler: Send + Sync {
    fn handle(
        &self,
        evt: odk::driver::Event,
    ) -> Result<odk::driver::Response, odk::driver::HandleError>;
}

impl<F, R> Handler for F
where
    Response: From<R>,
    Self: Fn(odk::driver::Event) -> Result<R, odk::driver::HandleError> + Send + Sync,
{
    fn handle(
        &self,
        evt: odk::driver::Event,
    ) -> Result<odk::driver::Response, odk::driver::HandleError> {
        let temp = self(evt)?;
        Ok(temp.into())
    }
}

pub struct Json<T>(pub T);

impl<T> From<Json<T>> for odk::driver::Response
where
    T: Serialize,
{
    fn from(value: Json<T>) -> Self {
        Self::Json(serde_json::to_vec(&value.0).unwrap())
    }
}

struct Channel {
    handler: Box<dyn Handler>,
    pattern: odk::driver::EventPattern,
}

type ChannelId = u32;
type ChannelRegistry = HashMap<ChannelId, Channel>;

static WORKER: LazyLock<RwLock<Worker>> = LazyLock::new(|| RwLock::new(Worker::new()));

#[derive(Default)]
pub struct Worker {
    channels: ChannelRegistry,
}

impl Worker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle_request<F>(mut self, method: &str, handler: F) -> Self
    where
        F: Handler + 'static,
    {
        self.channels.insert(
            self.channels.len() as u32,
            Channel {
                handler: Box::new(handler),
                pattern: odk::driver::EventPattern::Request(method.to_owned()),
            },
        );

        self
    }
}

pub trait Main {
    fn main() -> Worker;
}

impl<T: Main> self::bindings::Guest for T {
    fn handle(
        id: u32,
        evt: odk::driver::Event,
    ) -> Result<odk::driver::Response, odk::driver::HandleError> {
        let worker = WORKER.read().unwrap();
        let channel = worker.channels.get(&id).ok_or(1u32)?;

        channel.handler.handle(evt)
    }

    fn init(config: odk::driver::Cbor) {
        let worker = Self::main();

        let mut singelton = WORKER.write().unwrap();
        *singelton = worker;

        for (id, handler) in singelton.channels.iter() {
            odk::driver::register_channel(*id, &handler.pattern);
        }
    }
}

#[macro_export]
macro_rules! entrypoint {
    ($main:ident) => {
        struct _Main;

        impl balius::Main for _Main {
            fn main() -> balius::Worker {
                $main(Default::default())
            }
        }

        balius::bindings::export!(_Main);
    };
}
