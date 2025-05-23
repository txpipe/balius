/// Postgres backend for Key Value interface.
///
///
/// This expects to be connected to a DB that has a table named `kv`, which should be created
/// using the following insert statement:
///
/// ```sql
///
/// CREATE TABLE logs (
///     id BIGSERIAL PRIMARY KEY,
///     timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
///     worker VARCHAR(100) NOT NULL,
///     level VARCHAR(50) NOT NULL, -- e.g., INFO, WARN, ERROR, DEBUG
///     message TEXT NOT NULL,
///     context TEXT NOT NULL
/// );
use crate::{wit::balius::app::logging as wit, Error};
use std::str::FromStr;
use tokio_postgres::NoTls;

use super::LoggerProvider;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;

pub struct PostgresLogger {
    pool: Pool<PostgresConnectionManager<NoTls>>,
}

impl PostgresLogger {
    pub async fn try_new(connection: &str, max_size: Option<u32>) -> Result<Self, Error> {
        let config = tokio_postgres::config::Config::from_str(connection)
            .map_err(|err| Error::Config(format!("Failed to parse connection: {}", err)))?;
        let mgr = PostgresConnectionManager::new(config, tokio_postgres::NoTls);
        let pool = Pool::builder()
            .max_size(max_size.unwrap_or(5))
            .build(mgr)
            .await
            .map_err(|err| Error::Config(format!("Failed to connect: {}", err)))?;
        Ok(Self { pool })
    }
}

#[async_trait::async_trait]
impl LoggerProvider for PostgresLogger {
    async fn log(&mut self, worker_id: &str, level: wit::Level, context: String, message: String) {
        let conn = match self.pool.get().await {
            Ok(conn) => conn,
            Err(err) => {
                tracing::error!("Failed to get connection for Postgres logger: {}", err);
                return;
            }
        };

        if let Err(err) = conn
            .query(
                "INSERT INTO logs (worker, level, context, message)
                 VALUES ($1::TEXT, $2::TEXT, $3::TEXT, $4::TEXT)",
                &[
                    &worker_id,
                    match level {
                        wit::Level::Info => &"INFO",
                        wit::Level::Trace => &"TRACE",
                        wit::Level::Debug => &"DEBUG",
                        wit::Level::Error => &"ERROR",
                        wit::Level::Warn => &"WARN",
                        wit::Level::Critical => &"CRITICAL",
                    },
                    &context,
                    &message,
                ],
            )
            .await
        {
            tracing::warn!(
                worker_id = worker_id,
                err = err.to_string(),
                "failed to log into postgres"
            )
        }
    }
}
