use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{metrics::Metrics, wit::balius::app::submit as wit};

pub mod u5c;

#[derive(Clone)]
#[allow(clippy::large_enum_variant)]
pub enum Submit {
    Mock,
    U5C(u5c::Submit),
    Custom(Arc<Mutex<dyn wit::Host + Send + Sync>>),
}

pub struct SubmitHost {
    worker_id: String,
    submit: Submit,
    metrics: Arc<Metrics>,
}
impl SubmitHost {
    pub fn new(worker_id: &str, submit: &Submit, metrics: &Arc<Metrics>) -> Self {
        Self {
            worker_id: worker_id.to_string(),
            submit: submit.clone(),
            metrics: metrics.clone(),
        }
    }
}

#[async_trait::async_trait]
impl wit::Host for SubmitHost {
    async fn submit_tx(&mut self, tx: wit::Cbor) -> Result<(), wit::SubmitError> {
        self.metrics.submit_tx(&self.worker_id);
        match &mut self.submit {
            Submit::Mock => {
                println!("{}", hex::encode(tx));
                Ok(())
            }
            Submit::U5C(x) => x.submit_tx(tx).await,
            Submit::Custom(x) => {
                let mut lock = x.lock().await;
                lock.submit_tx(tx).await
            }
        }
    }
}
