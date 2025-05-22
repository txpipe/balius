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
use crate::wit::balius::app::logging as wit;

use super::LoggerProvider;

pub struct PostgresLogger {
    client: tokio_postgres::Client,
}

impl PostgresLogger {
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
impl LoggerProvider for PostgresLogger {
    async fn log(&mut self, worker_id: &str, level: wit::Level, context: String, message: String) {
        if let Err(err) = self
            .client
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
