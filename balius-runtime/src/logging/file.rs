/// Log to a file for development.
use std::{
    collections::BTreeMap,
    fs::{File, OpenOptions},
    io::{self, Write}, // Import io for Result
    path::PathBuf,
};

use crate::wit::balius::app::logging as wit;

use super::{level_to_string, LoggerProvider};

/// A logger that writes logs to files.
pub struct FileLogger {
    files: BTreeMap<String, File>,
    folder: PathBuf,
}

impl FileLogger {
    /// Tries to create a new `FileLogger` instance.
    ///
    /// Logs will be saved in the specified `folder`. If `None` is provided,
    /// the current directory will be used.
    pub fn try_new(folder: Option<PathBuf>) -> io::Result<Self> {
        let folder = folder.unwrap_or(std::env::current_dir()?);

        // Ensure the log folder exists
        std::fs::create_dir_all(&folder)?; // Use ? for error propagation

        Ok(FileLogger {
            files: BTreeMap::new(),
            folder,
        })
    }
}

#[async_trait::async_trait]
impl LoggerProvider for FileLogger {
    async fn log(&mut self, worker_id: &str, level: wit::Level, context: String, message: String) {
        let mut file = match self.files.get(worker_id) {
            Some(file) => file,
            None => {
                let Ok(file) = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(self.folder.join(format!("{worker_id}.log")))
                else {
                    return;
                };
                self.files.entry(worker_id.to_string()).or_insert(file)
            }
        };
        let _ = file.write(
            format!(
                "{}: {} - {} - {}\n",
                chrono::Utc::now().to_rfc3339(),
                level_to_string(&level),
                context,
                message
            )
            .as_bytes(),
        );
    }
}
