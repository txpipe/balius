pub mod in_memory;

use std::sync::Arc;

use tokio::sync::Mutex;

use crate::wit::balius::app::sign as wit;

#[derive(Clone)]
pub enum Signer {
    InMemory(in_memory::Signer),
    Custom(Arc<Mutex<dyn wit::Host + Send + Sync>>),
}

impl From<in_memory::Signer> for Signer {
    fn from(signer: in_memory::Signer) -> Self {
        Signer::InMemory(signer)
    }
}

#[async_trait::async_trait]
impl wit::Host for Signer {
    async fn sign_payload(
        &mut self,
        key_name: String,
        algorithm: String,
        payload: wit::Payload,
    ) -> Result<wit::Signature, wit::SignError> {
        match self {
            Self::InMemory(signer) => signer.sign_payload(&key_name, &algorithm, payload),
            Self::Custom(signer) => {
                let mut lock = signer.lock().await;
                lock.sign_payload(key_name, algorithm, payload).await
            }
        }
    }

    async fn get_public_key(
        &mut self,
        key_name: String,
        algorithm: String,
    ) -> Result<wit::PublicKey, wit::SignError> {
        match self {
            Self::InMemory(signer) => signer.get_public_key(&key_name, &algorithm),
            Self::Custom(signer) => {
                let mut lock = signer.lock().await;
                lock.get_public_key(key_name, algorithm).await
            }
        }
    }
}
