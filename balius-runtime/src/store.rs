use std::{path::Path, sync::Arc};

use itertools::Itertools;
use redb::{ReadableTable as _, TableDefinition, WriteTransaction};
use tracing::warn;

use crate::Error;

pub type WorkerId = String;
pub type LogSeq = u64;
// pub type Block = utxorpc::ChainBlock<utxorpc::spec::cardano::Block>;
pub type Block<'a> = pallas::ledger::traverse::MultiEraBlock<'a>;

const CURSORS: TableDefinition<WorkerId, LogSeq> = TableDefinition::new("cursors");

const DEFAULT_CACHE_SIZE_MB: usize = 50;

pub struct AtomicUpdate {
    wx: WriteTransaction,
    log_seq: LogSeq,
}

impl AtomicUpdate {
    pub fn update_worker_cursor(&mut self, id: &str) -> Result<(), super::Error> {
        let mut table = self.wx.open_table(CURSORS)?;
        table.insert(id.to_owned(), self.log_seq)?;

        Ok(())
    }

    pub fn commit(self) -> Result<(), super::Error> {
        self.wx.commit()?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct Store {
    db: Arc<redb::Database>,
}

impl Store {
    pub fn open(path: impl AsRef<Path>, cache_size: Option<usize>) -> Result<Self, super::Error> {
        let inner = redb::Database::builder()
            .set_repair_callback(|x| {
                warn!(progress = x.progress() * 100f64, "balius db is repairing")
            })
            .set_cache_size(1024 * 1024 * cache_size.unwrap_or(DEFAULT_CACHE_SIZE_MB))
            .create(path)?;

        let out = Self {
            db: Arc::new(inner),
        };

        Ok(out)
    }

    pub fn write_ahead(&self, block: &Block<'_>) -> Result<LogSeq, Error> {
        // TODO: write event to WAL table and return log sequence
        Ok(0)
    }

    // TODO: see if loading in batch is worth it
    pub fn get_worker_cursor(&self, id: &str) -> Result<Option<LogSeq>, super::Error> {
        let rx = self.db.begin_read()?;

        let table = match rx.open_table(CURSORS) {
            Ok(table) => table,
            Err(redb::TableError::TableDoesNotExist(_)) => return Ok(None),
            Err(e) => return Err(e.into()),
        };

        let cursor = table.get(id.to_owned())?;
        Ok(cursor.map(|x| x.value()))
    }

    pub fn start_atomic_update(&self, log_seq: LogSeq) -> Result<AtomicUpdate, super::Error> {
        let wx = self.db.begin_write()?;
        Ok(AtomicUpdate { wx, log_seq })
    }

    // TODO: I don't think we need this since we're going to load each cursor as
    // part of the loaded worker
    pub fn lowest_cursor(&self) -> Result<Option<LogSeq>, super::Error> {
        let rx = self.db.begin_read()?;

        let table = rx.open_table(CURSORS)?;

        let cursors: Vec<_> = table
            .iter()?
            .map_ok(|(_, value)| value.value())
            .try_collect()?;

        let lowest = cursors.iter().fold(None, |all, item| all.min(Some(*item)));

        Ok(lowest)
    }
}
