use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::wit::balius::app::logging as wit;

#[derive(Clone)]
pub enum Logger {
    Silent,
    Tracing,
    Custom(Arc<Mutex<dyn wit::Host + Send + Sync>>),
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

#[async_trait]
impl wit::Host for Logger {
    async fn log(&mut self, level: wit::Level, context: String, message: String) {
        match self {
            Self::Silent => {}
            Self::Tracing => {
                let level = match level {
                    wit::Level::Trace => tracing::Level::TRACE,
                    wit::Level::Debug => tracing::Level::DEBUG,
                    wit::Level::Info => tracing::Level::INFO,
                    wit::Level::Warn => tracing::Level::WARN,
                    wit::Level::Error => tracing::Level::ERROR,
                    wit::Level::Critical => tracing::Level::ERROR,
                };
                dyn_event!(level, context, message);
            }
            Self::Custom(logger) => {
                let mut lock = logger.lock().await;
                lock.log(level, context, message).await
            }
        }
    }
}
