/// In memory implementation of KV for development usage.
use std::{path::Path, sync::Arc};

use crate::wit::balius::app::kv as wit;
use redb::{Database, Durability, ReadableTable, TableDefinition};
use tracing::warn;
use wit::{KvError, Payload};

use super::KvProvider;
use crate::Error;

#[derive(Clone)]
pub struct RedbKv {
    db: Arc<Database>,
}

impl RedbKv {
    pub const DEF: TableDefinition<'static, String, Vec<u8>> = TableDefinition::new("kv");
    pub fn try_new(path: impl AsRef<Path>, cache_size: Option<usize>) -> Result<Self, Error> {
        let db = Database::builder()
            .set_repair_callback(|x| warn!(progress = x.progress() * 100f64, "db is repairing"))
            .set_cache_size(1024 * 1024 * cache_size.unwrap_or(10_000))
            .create(path)
            .map_err(|err| Error::KvError(err.to_string()))?;

        let mut wx = db
            .begin_write()
            .map_err(|err| Error::KvError(err.to_string()))?;
        wx.set_durability(Durability::Immediate);
        wx.open_table(Self::DEF)
            .map_err(|err| Error::KvError(err.to_string()))?;
        wx.commit().map_err(|err| Error::KvError(err.to_string()))?;

        Ok(Self { db: Arc::new(db) })
    }

    pub fn key_for_worker(worker_id: &str, key: &str) -> String {
        format!("{worker_id}-{key}")
    }

    pub fn into_ephemeral(&mut self) -> Result<Self, Error> {
        let new_db = redb::Database::builder()
            .create_with_backend(redb::backends::InMemoryBackend::new())
            .map_err(|e| Error::KvError(e.to_string()))?;

        let rx = self.db.begin_read()?;
        let wx = new_db.begin_write()?;

        {
            let source = rx
                .open_table(Self::DEF)
                .map_err(|e| Error::KvError(e.to_string()))?;
            let mut target = wx
                .open_table(Self::DEF)
                .map_err(|e| Error::KvError(e.to_string()))?;

            for entry in source.iter().map_err(|e| Error::KvError(e.to_string()))? {
                let (k, v) = entry.map_err(|e| Error::KvError(e.to_string()))?;
                target
                    .insert(k.value(), v.value())
                    .map_err(|e| Error::KvError(e.to_string()))?;
            }
        }

        wx.commit().map_err(|e| Error::KvError(e.to_string()))?;

        let new = Self {
            db: Arc::new(new_db),
        };

        Ok(new)
    }
}

#[async_trait::async_trait]
impl KvProvider for RedbKv {
    async fn get_value(&mut self, worker_id: &str, key: String) -> Result<Payload, KvError> {
        let rx = self
            .db
            .begin_read()
            .map_err(|err| KvError::Internal(err.to_string()))?;

        let table = rx
            .open_table(Self::DEF)
            .map_err(|err| KvError::Internal(err.to_string()))?;
        match table
            .get(Self::key_for_worker(worker_id, &key))
            .map_err(|err| KvError::Internal(err.to_string()))?
        {
            Some(value) => Ok(value.value()),
            None => Err(KvError::NotFound(key)),
        }
    }

    async fn set_value(
        &mut self,
        worker_id: &str,
        key: String,
        value: Payload,
    ) -> Result<(), KvError> {
        let wx = self
            .db
            .begin_write()
            .map_err(|err| KvError::Internal(err.to_string()))?;

        {
            let mut table = wx
                .open_table(Self::DEF)
                .map_err(|err| KvError::Internal(err.to_string()))?;

            table
                .insert(Self::key_for_worker(worker_id, &key), value)
                .map_err(|err| KvError::Internal(err.to_string()))?;
        }

        wx.commit()
            .map_err(|err| KvError::Internal(err.to_string()))?;

        Ok(())
    }

    async fn list_values(
        &mut self,
        worker_id: &str,
        prefix: String,
    ) -> Result<Vec<String>, KvError> {
        let rx = self
            .db
            .begin_read()
            .map_err(|err| KvError::Internal(err.to_string()))?;

        let table = rx
            .open_table(Self::DEF)
            .map_err(|err| KvError::Internal(err.to_string()))?;

        let mut result = vec![];
        let range = table
            .range(Self::key_for_worker(worker_id, &prefix)..)
            .map_err(|err| KvError::Internal(err.to_string()))?;

        for item in range {
            let (k, _) = item.unwrap();
            if k.value()
                .starts_with(&Self::key_for_worker(worker_id, &prefix))
            {
                result.push(k.value());
            } else {
                break;
            }
        }
        Ok(result)
    }
}
