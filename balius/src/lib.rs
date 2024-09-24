pub mod bindings {
    wit_bindgen::generate!({
        pub_export_macro: true,
        default_bindings_module: "balius::bindings",
    });
}

//mod macros;
mod qol;

pub use qol::*;

use bindings::balius::odk;
use serde::Serialize;
use std::{
    collections::HashMap,
    ops::Deref,
    sync::{LazyLock, RwLock},
};

pub trait Handler: Send + Sync {
    fn handle(
        &self,
        config: odk::driver::Cbor,
        evt: odk::driver::Event,
    ) -> std::result::Result<odk::driver::Response, odk::driver::HandleError>;
}

// Add the IntoHandler trait definition
pub trait IntoHandler<O> {
    fn into_handler(self) -> Box<dyn Handler>;
}

macro_rules! define_fn_handler {
    ($id:ident, $($P:ident),*) => {
        pub struct $id<F, C, $($P,)* O> {
            f: F,
            _phantom: std::marker::PhantomData<(C, $($P,)* O)>,
        }

        impl<F, C, $($P,)* O> $id<F, C, $($P,)* O> {
            pub fn new(f: F) -> Self {
                Self {
                    f,
                    _phantom: std::marker::PhantomData,
                }
            }
        }

        impl<F, C, $($P,)* O> Handler for $id<F, C, $($P,)* O>
        where
            F: Fn(C, $($P,)*) -> Result<O> + Send + Sync,
            C: From<bindings::Env> + Send + Sync,
            $($P: From<odk::driver::Event> + Send + Sync,)*
            O: Into<odk::driver::Response> + Send + Sync,
        {
            fn handle(
                &self,
                env: bindings::Env,
                evt: odk::driver::Event,
            ) -> std::result::Result<odk::driver::Response, odk::driver::HandleError> {
                #[allow(unused_parens)]
                let output = (self.f)(C::from(env), $($P::from(evt.clone()),)*)?;
                Ok(output.into())
            }
        }


    };
}

// Use the macro to define FnHandler for different arities
define_fn_handler!(FnHandler, P1);
define_fn_handler!(FnHandler2, P1, P2);
define_fn_handler!(FnHandler3, P1, P2, P3);
define_fn_handler!(FnHandler4, P1, P2, P3, P4);

pub struct Json<T>(pub T);

impl<T> Deref for Json<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<odk::driver::Event> for Json<T>
where
    T: serde::de::DeserializeOwned,
{
    fn from(value: odk::driver::Event) -> Self {
        let bytes = match value {
            odk::driver::Event::Request(x) => x,
            _ => todo!(),
        };

        let t = serde_json::from_slice(bytes.as_slice()).unwrap();
        Json(t)
    }
}

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
    env: Option<bindings::Env>,
}

impl Worker {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn init(mut self, env: bindings::Env) -> Self {
        self.env = Some(env);

        self
    }

    pub fn handle_request<C, P, O, F>(mut self, method: &str, handler: F) -> Self
    where
        F: Fn(Env<C>, P) -> qol::Result<O> + Send + Sync + 'static,
        Env<C>: From<bindings::Env> + Send + Sync + 'static,
        P: From<odk::driver::Event> + Send + Sync + 'static,
        O: Into<odk::driver::Response> + Send + Sync + 'static,
    {
        let handler = FnHandler::new(handler);

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
    ) -> std::result::Result<odk::driver::Response, odk::driver::HandleError> {
        let worker = WORKER.read().unwrap();
        let channel = worker.channels.get(&id).ok_or(1u32)?;
        let env = match &worker.env {
            Some(e) => e.clone(),
            None => return Err(0),
        };

        channel.handler.handle(env, evt)
    }

    fn init(env: bindings::Env) {
        let worker = Self::main().init(env);

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
                $main()
            }
        }

        balius::bindings::export!(_Main);
    };
}
