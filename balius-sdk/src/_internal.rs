use std::{
    collections::{BTreeMap, HashMap},
    sync::{LazyLock, RwLock},
};

use crate::wit;

type ChannelId = u32;

pub trait Handler: Send + Sync + 'static {
    fn handle(
        &self,
        config: wit::Config,
        event: wit::Event,
    ) -> Result<wit::Response, wit::HandleError>;
}

pub(crate) struct Channel {
    pub(crate) handler: Box<dyn Handler>,
    pub(crate) pattern: wit::balius::app::driver::EventPattern,
}

type ChannelRegistry = HashMap<ChannelId, Channel>;

#[derive(Default)]
pub struct Worker {
    pub(crate) channels: ChannelRegistry,
    pub(crate) config: Option<wit::Config>,
    pub(crate) requested_signers: BTreeMap<String, String>,
    pub(crate) signers: BTreeMap<String, Vec<u8>>,
}

static WORKER: LazyLock<RwLock<Worker>> = LazyLock::new(|| RwLock::new(Worker::default()));

pub fn global_init_worker(env: wit::Config, mut worker: Worker) {
    worker.init(env);

    for (id, handler) in worker.channels.iter() {
        wit::balius::app::driver::register_channel(*id, &handler.pattern);
    }

    for (name, algorithm) in worker.requested_signers.iter() {
        worker.signers.insert(
            name.clone(),
            wit::balius::app::driver::register_signer(name, algorithm),
        );
    }

    let mut singelton = WORKER.write().unwrap();
    *singelton = worker;
}

pub fn global_handle_request(id: u32, evt: wit::Event) -> Result<wit::Response, wit::HandleError> {
    let worker = WORKER.read().unwrap();

    let channel = worker.channels.get(&id).ok_or(wit::HandleError {
        code: 1,
        message: "no channel".to_owned(),
    })?;

    let config = match &worker.config {
        Some(e) => e.clone(),
        None => {
            return Err(wit::HandleError {
                code: 0,
                message: "no config".to_owned(),
            })
        }
    };

    channel.handler.handle(config, evt)
}

pub fn global_get_public_keys() -> BTreeMap<String, Vec<u8>> {
    let worker = WORKER.read().unwrap();
    worker.signers.clone()
}
