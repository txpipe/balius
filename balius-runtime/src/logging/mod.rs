use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::{metrics::Metrics, wit::balius::app::logging as wit};

pub mod file;

#[derive(Clone)]
pub enum Logger {
    Silent,
    Tracing,
    File(Arc<Mutex<file::FileLogger>>),
    Custom(Arc<Mutex<dyn LoggerProvider + Send + Sync>>),
}

// need this to set the trace level at runtime
macro_rules! dyn_event {
    ($lvl:ident, $($arg:tt)+) => {
        match $lvl {
            ::tracing::Level::TRACE => ::tracing::trace!($($arg)+),
            ::tracing::Level::DEBUG => ::tracing::debug!($($arg)+),
            ::tracing::Level::INFO => ::tracing::info!($($arg)+),
            ::tracing::Level::WARN => ::tracing::warn!($($arg)+),
            ::tracing::Level::ERROR => ::tracing::error!($($arg)+),
        }
    };
}

pub struct LoggerHost {
    worker_id: String,
    provider: Logger,
    metrics: Arc<Metrics>,
}
impl LoggerHost {
    pub fn new(worker_id: &str, provider: &Logger, metrics: &Arc<Metrics>) -> Self {
        Self {
            worker_id: worker_id.to_string(),
            provider: provider.clone(),
            metrics: metrics.clone(),
        }
    }
}

#[async_trait]
pub trait LoggerProvider {
    async fn log(&mut self, worker_id: &str, level: wit::Level, context: String, message: String);
}

impl wit::Host for LoggerHost {
    async fn log(&mut self, level: wit::Level, context: String, message: String) {
        self.metrics.log(&self.worker_id, &level);
        match &mut self.provider {
            Logger::Silent => {}
            Logger::Tracing => {
                let level = match level {
                    wit::Level::Trace => tracing::Level::TRACE,
                    wit::Level::Debug => tracing::Level::DEBUG,
                    wit::Level::Info => tracing::Level::INFO,
                    wit::Level::Warn => tracing::Level::WARN,
                    wit::Level::Error => tracing::Level::ERROR,
                    wit::Level::Critical => tracing::Level::ERROR,
                };
                dyn_event!(level, worker_id = self.worker_id, context, message);
            }
            Logger::File(logger) => {
                let mut lock = logger.lock().await;
                lock.log(&self.worker_id, level, context, message).await
            }
            Logger::Custom(logger) => {
                let mut lock = logger.lock().await;
                lock.log(&self.worker_id, level, context, message).await
            }
        }
    }
}

pub fn level_to_string(level: &wit::Level) -> String {
    match level {
        wit::Level::Info => "INFO".to_string(),
        wit::Level::Trace => "TRACE".to_string(),
        wit::Level::Debug => "DEBUG".to_string(),
        wit::Level::Error => "ERROR".to_string(),
        wit::Level::Warn => "WARN".to_string(),
        wit::Level::Critical => "CRITICAL".to_string(),
    }
}
