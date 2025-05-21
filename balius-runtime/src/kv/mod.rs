use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

use crate::wit::balius::app::kv as wit;

pub use wit::{Host as CustomKv, KvError, Payload};

#[derive(Clone)]
pub enum Kv {
    Mock,
    Memory(Arc<RwLock<memory::MemoryKv>>),
    Custom(Arc<Mutex<dyn KvProvider + Send + Sync>>),
}

pub struct KvHost {
    worker_id: String,
    provider: Kv,
}
impl KvHost {
    pub fn new(worker_id: &str, provider: &Kv) -> Self {
        Self {
            worker_id: worker_id.to_string(),
            provider: provider.clone(),
        }
    }
}

pub mod memory;

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

#[async_trait::async_trait]
impl wit::Host for KvHost {
    async fn get_value(&mut self, key: String) -> Result<Payload, KvError> {
        match &mut self.provider {
            Kv::Mock => todo!(),
            Kv::Memory(kv) => {
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
        match &mut self.provider {
            Kv::Mock => todo!(),
            Kv::Memory(kv) => {
                kv.write()
                    .await
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
        match &mut self.provider {
            Kv::Mock => todo!(),
            Kv::Memory(kv) => {
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
