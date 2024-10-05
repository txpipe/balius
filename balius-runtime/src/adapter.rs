use std::sync::Arc;

use crate::{
    ledgers,
    router::Router,
    wit::balius::app::{driver, kv, ledger, submit},
};

pub struct Adapter {
    worker_id: String,
    router: Router,
    pub ledger: ledgers::Ledger,
}

impl Adapter {
    pub fn new(worker_id: String, router: Router, ledger: ledgers::Ledger) -> Self {
        Self {
            worker_id,
            router,
            ledger,
        }
    }
}

#[async_trait::async_trait]
impl kv::Host for Adapter {
    async fn get_value(&mut self, key: String) -> Result<kv::Payload, kv::KvError> {
        todo!()
    }

    async fn set_value(&mut self, key: String, value: kv::Payload) -> Result<(), kv::KvError> {
        println!("{}:{}", key, hex::encode(value));

        Ok(())
    }

    async fn list_values(&mut self, prefix: String) -> Result<Vec<String>, kv::KvError> {
        todo!()
    }
}

#[async_trait::async_trait]
impl submit::Host for Adapter {
    async fn submit_tx(&mut self, tx: submit::Cbor) -> Result<(), submit::SubmitError> {
        println!("{}", hex::encode(tx));

        Ok(())
    }
}

#[async_trait::async_trait]
impl driver::Host for Adapter {
    async fn register_channel(&mut self, id: u32, pattern: driver::EventPattern) -> () {
        self.router.register_channel(&self.worker_id, id, &pattern);
    }
}
