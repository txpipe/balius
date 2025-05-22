/// Postgres backend for Key Value interface.
///
///
/// This expects to be connected to a DB that has a table named `kv`, which should be created
/// using the following insert statement:
///
/// ```sql
///
/// CREATE TABLE kv (
///   worker VARCHAR(255) NOT NULL, -- String column for the worker identifier
///   key VARCHAR(255) NOT NULL,    -- String column for the key
///   value BYTEA,                  -- Bytea column for binary data (e.g., images, serialized objects)
///   PRIMARY KEY (worker, key)     -- Composite primary key on worker and key
/// );
use crate::wit::balius::app::kv as wit;
use wit::{KvError, Payload};

use super::KvProvider;

pub struct PostgresKv {
    client: tokio_postgres::Client,
}

impl PostgresKv {
    pub async fn try_new(config: &str) -> Result<Self, tokio_postgres::Error> {
        let (client, connection) = tokio_postgres::connect(config, tokio_postgres::NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Ok(Self { client })
    }
}

#[async_trait::async_trait]
impl KvProvider for PostgresKv {
    async fn get_value(&mut self, worker_id: &str, key: String) -> Result<Payload, KvError> {
        match self
            .client
            .query_opt(
                "SELECT value FROM kv WHERE worker = $1::TEXT AND key = $2::TEXT",
                &[&worker_id, &key],
            )
            .await
        {
            Ok(Some(row)) => Ok(row.get(0)),
            Ok(None) => Err(KvError::NotFound(key)),
            Err(err) => Err(KvError::Internal(err.to_string())),
        }
    }

    async fn set_value(
        &mut self,
        worker_id: &str,
        key: String,
        value: Payload,
    ) -> Result<(), KvError> {
        match self
            .client
            .query(
                "INSERT INTO kv (worker, key, value)
                 VALUES ($1::TEXT, $2::TEXT, $3::BYTEA)
                 ON CONFLICT(worker, key) 
                 DO UPDATE SET value = EXCLUDED.value;",
                &[&worker_id, &key, &value],
            )
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(KvError::Internal(err.to_string())),
        }
    }

    async fn list_values(
        &mut self,
        worker_id: &str,
        prefix: String,
    ) -> Result<Vec<String>, KvError> {
        match self
            .client
            .query(
                "SELECT key FROM kv WHERE worker = $1::TEXT AND key LIKE $2::TEXT ORDER BY key",
                &[&worker_id, &format!("{}%", prefix)],
            )
            .await
        {
            Ok(rows) => Ok(rows.iter().map(|row| row.get(0)).collect()),
            Err(err) => Err(KvError::Internal(err.to_string())),
        }
    }
}
