pub mod in_memory;

use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{metrics::Metrics, wit::balius::app::sign as wit};

#[derive(Clone)]
pub enum Signer {
    InMemory(in_memory::Signer),
    Custom(Arc<Mutex<dyn SignerProvider + Send + Sync>>),
}

impl From<in_memory::Signer> for Signer {
    fn from(signer: in_memory::Signer) -> Self {
        Signer::InMemory(signer)
    }
}

pub struct SignerHost {
    worker_id: String,
    provider: Signer,
    metrics: Arc<Metrics>,
}
impl SignerHost {
    pub fn new(worker_id: &str, provider: &Signer, metrics: &Arc<Metrics>) -> Self {
        Self {
            worker_id: worker_id.to_string(),
            provider: provider.clone(),
            metrics: metrics.clone(),
        }
    }

    pub async fn add_key(&mut self, key_name: String, algorithm: String) -> Vec<u8> {
        match &mut self.provider {
            Signer::InMemory(signer) => signer.add_key(&self.worker_id, key_name, algorithm).await,
            Signer::Custom(signer) => {
                let mut lock = signer.lock().await;
                lock.add_key(&self.worker_id, key_name, algorithm).await
            }
        }
    }
}

#[async_trait::async_trait]
pub trait SignerProvider {
    async fn add_key(&mut self, worker_id: &str, key_name: String, algorithm: String) -> Vec<u8>;
    async fn sign_payload(
        &mut self,
        worker_id: &str,
        key_name: String,
        payload: wit::Payload,
    ) -> Result<wit::Signature, wit::SignError>;
}

impl wit::Host for SignerHost {
    async fn sign_payload(
        &mut self,
        key_name: String,
        payload: wit::Payload,
    ) -> Result<wit::Signature, wit::SignError> {
        self.metrics.signer_sign_payload(&self.worker_id);
        match &mut self.provider {
            Signer::InMemory(signer) => {
                signer
                    .sign_payload(&self.worker_id, key_name, payload)
                    .await
            }
            Signer::Custom(signer) => {
                let mut lock = signer.lock().await;
                lock.sign_payload(&self.worker_id, key_name, payload).await
            }
        }
    }
}
