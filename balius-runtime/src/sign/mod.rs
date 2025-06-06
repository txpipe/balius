pub mod in_memory;

use std::sync::Arc;

use tokio::sync::Mutex;

use crate::wit::balius::app::sign as wit;

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
impl Signer {
    pub async fn add_key(
        &mut self,
        worker_id: &str,
        key_name: String,
    ) -> Result<(), wit::SignError> {
        match self {
            Signer::InMemory(signer) => signer.add_key(worker_id, key_name).await,
            Signer::Custom(signer) => {
                let mut lock = signer.lock().await;
                lock.add_key(worker_id, key_name).await
            }
        }
    }
}

pub struct SignerHost {
    worker_id: String,
    provider: Signer,
}
impl SignerHost {
    pub fn new(worker_id: &str, provider: &Signer) -> Self {
        Self {
            worker_id: worker_id.to_string(),
            provider: provider.clone(),
        }
    }
}

#[async_trait::async_trait]
pub trait SignerProvider {
    async fn add_key(&mut self, worker_id: &str, key_name: String) -> Result<(), wit::SignError>;
    async fn sign_payload(
        &mut self,
        worker_id: &str,
        key_name: String,
        algorithm: String,
        payload: wit::Payload,
    ) -> Result<wit::Signature, wit::SignError>;
    async fn get_public_key(
        &mut self,
        worker_id: &str,
        key_name: String,
        algorithm: String,
    ) -> Result<wit::PublicKey, wit::SignError>;
}

#[async_trait::async_trait]
impl wit::Host for SignerHost {
    async fn sign_payload(
        &mut self,
        key_name: String,
        algorithm: String,
        payload: wit::Payload,
    ) -> Result<wit::Signature, wit::SignError> {
        match &mut self.provider {
            Signer::InMemory(signer) => {
                signer
                    .sign_payload(&self.worker_id, key_name, algorithm, payload)
                    .await
            }
            Signer::Custom(signer) => {
                let mut lock = signer.lock().await;
                lock.sign_payload(&self.worker_id, key_name, algorithm, payload)
                    .await
            }
        }
    }

    async fn get_public_key(
        &mut self,
        key_name: String,
        algorithm: String,
    ) -> Result<wit::PublicKey, wit::SignError> {
        match &mut self.provider {
            Signer::InMemory(signer) => {
                signer
                    .get_public_key(&self.worker_id, key_name, algorithm)
                    .await
            }
            Signer::Custom(signer) => {
                let mut lock = signer.lock().await;
                lock.get_public_key(&self.worker_id, key_name, algorithm)
                    .await
            }
        }
    }
}
