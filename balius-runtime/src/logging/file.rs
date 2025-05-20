/// Log to a file for development.
use std::{
    fs::{File, OpenOptions},
    io::Write,
};

use crate::wit::balius::app::logging as wit;

pub struct FileLogger {
    file: File,
}
impl FileLogger {
    pub fn try_new(path: &str) -> std::io::Result<Self> {
        // Open a file with append option
        Ok(Self {
            file: OpenOptions::new().create(true).append(true).open(path)?,
        })
    }
}

#[async_trait::async_trait]
impl wit::Host for FileLogger {
    async fn log(&mut self, level: wit::Level, context: String, message: String) {
        // Errors logging are ignored.
        let _ = self.file.write(
            format!(
                "{}: {} - {} - {}\n",
                chrono::Utc::now().to_rfc3339(),
                match level {
                    wit::Level::Info => "INFO",
                    wit::Level::Trace => "TRACE",
                    wit::Level::Debug => "DEBUG",
                    wit::Level::Error => "ERROR",
                    wit::Level::Warn => "WARN",
                    wit::Level::Critical => "CRITICAL",
                },
                context,
                message
            )
            .as_bytes(),
        );
    }
}
