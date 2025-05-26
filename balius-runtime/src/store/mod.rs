pub mod redb;

use prost::Message;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{Block, ChainPoint, Error};

pub type WorkerId = String;
pub type LogSeq = u64;

#[derive(Message)]
pub struct LogEntry {
    #[prost(bytes, tag = "1")]
    pub next_block: Vec<u8>,
    #[prost(bytes, repeated, tag = "2")]
    pub undo_blocks: Vec<Vec<u8>>,
}

#[async_trait::async_trait]
pub trait AtomicUpdateTrait {
    async fn update_worker_cursor(&mut self, id: &str) -> Result<(), super::Error>;
    async fn commit(&mut self) -> Result<(), super::Error>;
}

#[allow(clippy::large_enum_variant)]
pub enum AtomicUpdate {
    Redb(redb::AtomicUpdate),
    Custom(Arc<Mutex<dyn AtomicUpdateTrait + Send + Sync>>),
}

#[async_trait::async_trait]
impl AtomicUpdateTrait for AtomicUpdate {
    async fn update_worker_cursor(&mut self, id: &str) -> Result<(), super::Error> {
        match self {
            AtomicUpdate::Redb(au) => au.update_worker_cursor(id).await,
            AtomicUpdate::Custom(au) => au.lock().await.update_worker_cursor(id).await,
        }
    }
    async fn commit(&mut self) -> Result<(), super::Error> {
        match self {
            AtomicUpdate::Redb(au) => au.commit().await,
            AtomicUpdate::Custom(au) => au.lock().await.commit().await,
        }
    }
}

#[derive(Clone)]
pub enum Store {
    Redb(redb::Store),
    Custom(Arc<Mutex<dyn StoreTrait + Send + Sync>>),
}

#[async_trait::async_trait]
pub trait StoreTrait {
    async fn find_chain_point(&self, seq: LogSeq) -> Result<Option<ChainPoint>, Error>;
    async fn write_ahead(
        &mut self,
        undo_blocks: &[Block],
        next_block: &Block,
    ) -> Result<LogSeq, Error>;
    async fn get_worker_cursor(&self, id: &str) -> Result<Option<LogSeq>, super::Error>;
    async fn start_atomic_update(&self, log_seq: LogSeq) -> Result<AtomicUpdate, super::Error>;
}

#[async_trait::async_trait]
impl StoreTrait for Store {
    async fn find_chain_point(&self, seq: LogSeq) -> Result<Option<ChainPoint>, Error> {
        match self {
            Store::Redb(store) => store.find_chain_point(seq).await,
            Store::Custom(store) => store.lock().await.find_chain_point(seq).await,
        }
    }
    async fn write_ahead(
        &mut self,
        undo_blocks: &[Block],
        next_block: &Block,
    ) -> Result<LogSeq, Error> {
        match self {
            Store::Redb(store) => store.write_ahead(undo_blocks, next_block).await,
            Store::Custom(store) => {
                store
                    .lock()
                    .await
                    .write_ahead(undo_blocks, next_block)
                    .await
            }
        }
    }
    async fn get_worker_cursor(&self, id: &str) -> Result<Option<LogSeq>, super::Error> {
        match self {
            Store::Redb(store) => store.get_worker_cursor(id).await,
            Store::Custom(store) => store.lock().await.get_worker_cursor(id).await,
        }
    }
    async fn start_atomic_update(&self, log_seq: LogSeq) -> Result<AtomicUpdate, super::Error> {
        match self {
            Store::Redb(store) => store.start_atomic_update(log_seq).await,
            Store::Custom(store) => store.lock().await.start_atomic_update(log_seq).await,
        }
    }
}
