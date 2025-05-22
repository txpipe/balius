use std::sync::Arc;

use tokio::sync::Mutex;

#[derive(Clone)]
pub enum Lease {
    Mock,
    Custom(Arc<Mutex<dyn LeaseHandler + Send + Sync>>),
}

#[async_trait::async_trait]
pub trait LeaseHandler {
    async fn is_leader(&mut self) -> bool;
}

#[async_trait::async_trait]
impl LeaseHandler for Lease {
    async fn is_leader(&mut self) -> bool {
        match self {
            Lease::Mock => true,
            Lease::Custom(lease) => lease.lock().await.is_leader().await,
        }
    }
}
