use std::sync::Arc;
use tokio::sync::Mutex;

use crate::wit::balius::app::kv as wit;

pub use wit::{Host as CustomKv, KvError, Payload};

#[derive(Clone)]
pub enum Kv {
    Mock,
    Custom(Arc<Mutex<dyn wit::Host + Send + Sync>>),
}

#[async_trait::async_trait]
impl wit::Host for Kv {
    async fn get_value(&mut self, key: String) -> Result<Payload, KvError> {
        match self {
            Self::Mock => todo!(),
            Self::Custom(kv) => {
                let mut lock = kv.lock().await;
                lock.get_value(key).await
            }
        }
    }

    async fn set_value(&mut self, key: String, value: Payload) -> Result<(), KvError> {
        match self {
            Self::Mock => {
                println!("{}:{}", key, hex::encode(value));
                Ok(())
            }
            Self::Custom(kv) => {
                let mut lock = kv.lock().await;
                lock.set_value(key, value).await
            }
        }
    }

    async fn list_values(&mut self, prefix: String) -> Result<Vec<String>, KvError> {
        match self {
            Self::Mock => todo!(),
            Self::Custom(kv) => {
                let mut lock = kv.lock().await;
                lock.list_values(prefix).await
            }
        }
    }
}
