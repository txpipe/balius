use itertools::Itertools;
use prost::Message;
use redb::{ReadableTable as _, TableDefinition, WriteTransaction};
use std::{collections::VecDeque, path::Path, sync::Arc};
use tracing::warn;

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

impl redb::Value for LogEntry {
    type SelfType<'a>
        = LogEntry
    where
        Self: 'a;

    type AsBytes<'a>
        = Vec<u8>
    where
        Self: 'a;

    fn fixed_width() -> Option<usize> {
        None
    }

    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where
        Self: 'a,
    {
        prost::Message::decode(data).unwrap()
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
    where
        Self: 'a,
        Self: 'b,
    {
        value.encode_to_vec()
    }

    fn type_name() -> redb::TypeName {
        redb::TypeName::new("LogEntry")
    }
}

const CURSORS: TableDefinition<WorkerId, LogSeq> = TableDefinition::new("cursors");
const WAL: TableDefinition<LogSeq, LogEntry> = TableDefinition::new("wal");

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
    log_seq: LogSeq,
}

impl Store {
    pub fn in_memory() -> Result<Self, super::Error> {
        let db = Arc::new(
            redb::Database::builder().create_with_backend(redb::backends::InMemoryBackend::new())?,
        );
        Ok(Self { db, log_seq: 0 })
    }

    pub fn open(path: impl AsRef<Path>, cache_size: Option<usize>) -> Result<Self, super::Error> {
        let inner = redb::Database::builder()
            .set_repair_callback(|x| {
                warn!(progress = x.progress() * 100f64, "balius db is repairing")
            })
            .set_cache_size(1024 * 1024 * cache_size.unwrap_or(DEFAULT_CACHE_SIZE_MB))
            .create(path)?;

        let log_seq = Self::load_log_seq(&inner)?.unwrap_or_default();

        let out = Self {
            db: Arc::new(inner),
            log_seq,
        };

        Ok(out)
    }

    pub fn into_ephemeral(&mut self) -> Result<Self, super::Error> {
        let new_db =
            redb::Database::builder().create_with_backend(redb::backends::InMemoryBackend::new())?;

        let rx = self.db.begin_read()?;
        let wx = new_db.begin_write()?;

        {
            if let Ok(source) = rx.open_table(WAL) {
                let mut target = wx.open_table(WAL)?;

                for entry in source.iter()? {
                    let (k, v) = entry?;
                    target.insert(k.value(), v.value())?;
                }
            }

            if let Ok(source) = rx.open_table(CURSORS) {
                let mut target = wx.open_table(CURSORS)?;

                for entry in source.iter()? {
                    let (k, v) = entry?;
                    target.insert(k.value(), v.value())?;
                }
            }
        }

        wx.commit()?;

        let log_seq = Self::load_log_seq(&new_db)?.unwrap_or_default();
        let new = Store {
            db: Arc::new(new_db),
            log_seq,
        };

        Ok(new)
    }

    fn load_log_seq(db: &redb::Database) -> Result<Option<LogSeq>, Error> {
        let rx = db.begin_read()?;

        match rx.open_table(WAL) {
            Ok(table) => {
                let last = table.last()?;
                Ok(last.map(|(k, _)| k.value()))
            }
            Err(redb::TableError::TableDoesNotExist(_)) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    fn get_entry(&self, seq: LogSeq) -> Result<Option<LogEntry>, Error> {
        let rx = self.db.begin_read()?;
        let table = rx.open_table(WAL)?;
        let entry = table.get(seq)?;
        Ok(entry.map(|x| x.value()))
    }

    pub fn find_chain_point(&self, seq: LogSeq) -> Result<Option<ChainPoint>, Error> {
        let entry = self.get_entry(seq)?;
        let block = Block::from_bytes(&entry.unwrap().next_block);

        Ok(Some(block.chain_point()))
    }

    pub fn write_ahead(
        &mut self,
        undo_blocks: &[Block],
        next_block: &Block,
    ) -> Result<LogSeq, Error> {
        self.log_seq += 1;

        let wx = self.db.begin_write()?;
        {
            wx.open_table(WAL)?.insert(
                self.log_seq,
                LogEntry {
                    next_block: next_block.to_bytes(),
                    undo_blocks: undo_blocks.iter().map(|x| x.to_bytes()).collect(),
                },
            )?;
        }

        wx.commit()?;
        Ok(self.log_seq)
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

    /// Return list of blocks to undo after receiving a reset response from chainsync.
    pub fn handle_reset(&self, point: ChainPoint) -> Result<Vec<Block>, super::Error> {
        let rx = self.db.begin_read()?;
        let table = rx.open_table(WAL)?;

        let mut undos = VecDeque::new();
        for result in table.iter()?.rev() {
            let (_, v) = result?;
            let block = v.value().next_block.clone();
            let decoded = Block::from_bytes(&block);
            if decoded.slot() <= point.slot() {
                break;
            } else {
                undos.push_front(decoded);
            }
        }
        Ok(undos.into())
    }
}
