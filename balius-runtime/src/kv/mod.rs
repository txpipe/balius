use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

use crate::{metrics::Metrics, wit::balius::app::kv as wit, Error};

pub use wit::{Host as CustomKv, KvError, Payload};

#[derive(Clone)]
pub enum Kv {
    Mock,
    Memory(Arc<RwLock<memory::MemoryKv>>),
    Redb(Arc<RwLock<redb::RedbKv>>),
    Custom(Arc<Mutex<dyn KvProvider + Send + Sync>>),
}

impl Kv {
    pub async fn into_ephemeral(self) -> Result<Self, Error> {
        match self {
            Kv::Mock => Ok(self),
            Kv::Memory(x) => Ok(Kv::Memory(x)),
            Kv::Redb(x) => Ok(Kv::Redb(Arc::new(RwLock::new(
                x.write().await.into_ephemeral()?,
            )))),
            Kv::Custom(_) => Err(Error::KvError(
                "Cannot convert custom kv into ephemeral".to_string(),
            )),
        }
    }
}

pub struct KvHost {
    worker_id: String,
    provider: Kv,
    metrics: Arc<Metrics>,
}
impl KvHost {
    pub fn new(worker_id: &str, provider: &Kv, metrics: &Arc<Metrics>) -> Self {
        Self {
            worker_id: worker_id.to_string(),
            provider: provider.clone(),
            metrics: metrics.clone(),
        }
    }
}

pub mod memory;
pub mod redb;

#[async_trait::async_trait]
pub trait KvProvider {
    async fn get_value(&mut self, worker_id: &str, key: String) -> Result<Payload, KvError>;
    async fn set_value(
        &mut self,
        worker_id: &str,
        key: String,
        value: Payload,
    ) -> Result<(), KvError>;
    async fn list_values(
        &mut self,
        worker_id: &str,
        prefix: String,
    ) -> Result<Vec<String>, KvError>;
}

impl wit::Host for KvHost {
    async fn get_value(&mut self, key: String) -> Result<Payload, KvError> {
        self.metrics.kv_get(&self.worker_id);
        match &mut self.provider {
            Kv::Mock => todo!(),
            Kv::Memory(kv) => {
                kv.read()
                    .await
                    .clone()
                    .get_value(&self.worker_id, key)
                    .await
            }
            Kv::Redb(kv) => {
                kv.read()
                    .await
                    .clone()
                    .get_value(&self.worker_id, key)
                    .await
            }
            Kv::Custom(kv) => {
                let mut lock = kv.lock().await;
                lock.get_value(&self.worker_id, key).await
            }
        }
    }

    async fn set_value(&mut self, key: String, value: Payload) -> Result<(), KvError> {
        self.metrics.kv_set(&self.worker_id);
        match &mut self.provider {
            Kv::Mock => todo!(),
            Kv::Memory(kv) => {
                kv.write()
                    .await
                    .set_value(&self.worker_id, key, value)
                    .await
            }
            Kv::Redb(kv) => {
                kv.read()
                    .await
                    .clone()
                    .set_value(&self.worker_id, key, value)
                    .await
            }

            Kv::Custom(kv) => {
                let mut lock = kv.lock().await;
                lock.set_value(&self.worker_id, key, value).await
            }
        }
    }

    async fn list_values(&mut self, prefix: String) -> Result<Vec<String>, KvError> {
        self.metrics.kv_list(&self.worker_id);
        match &mut self.provider {
            Kv::Mock => todo!(),
            Kv::Memory(kv) => {
                kv.read()
                    .await
                    .clone()
                    .list_values(&self.worker_id, prefix)
                    .await
            }
            Kv::Redb(kv) => {
                kv.read()
                    .await
                    .clone()
                    .list_values(&self.worker_id, prefix)
                    .await
            }

            Kv::Custom(kv) => {
                let mut lock = kv.lock().await;
                lock.list_values(&self.worker_id, prefix).await
            }
        }
    }
}
