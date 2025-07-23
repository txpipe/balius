use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use balius_runtime::{drivers, ledgers, logging::file::FileLogger};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use tokio::sync::{Mutex, RwLock};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct StoreConfig {
    pub path: PathBuf,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoggingConfig {
    #[serde_as(as = "DisplayFromStr")]
    pub max_level: tracing::Level,

    #[serde(default)]
    pub include_tokio: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RedbKvConfig {
    pub path: PathBuf,
    pub cache_size: Option<usize>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum KvConfig {
    Memory,
    Redb(RedbKvConfig),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FileLoggerConfig {
    pub folder: Option<PathBuf>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum LoggerConfig {
    Silent,
    Tracing,
    File(FileLoggerConfig),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WorkerConfig {
    pub name: String,
    pub module: PathBuf,
    pub since_slot: Option<u64>,
    pub until_slot: Option<u64>,
    pub config: Option<PathBuf>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MetricsConfig {
    pub listen_address: SocketAddr,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum SignerConfig {
    Memory,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Config {
    pub rpc: drivers::jsonrpc::Config,
    pub ledger: ledgers::u5c::Config,
    pub chainsync: drivers::chainsync::Config,
    pub workers: Vec<WorkerConfig>,
    pub logging: LoggingConfig,
    pub kv: Option<KvConfig>,
    pub logger: Option<LoggerConfig>,
    pub metrics: Option<MetricsConfig>,
    pub sign: Option<SignerConfig>,
    pub store: Option<StoreConfig>,
}

impl From<&Config> for balius_runtime::kv::Kv {
    fn from(value: &Config) -> Self {
        match &value.kv {
            Some(KvConfig::Memory) => balius_runtime::kv::Kv::Custom(Arc::new(Mutex::new(
                balius_runtime::kv::memory::MemoryKv::default(),
            ))),
            Some(KvConfig::Redb(cfg)) => balius_runtime::kv::Kv::Redb(Arc::new(RwLock::new(
                balius_runtime::kv::redb::RedbKv::try_new(&cfg.path, cfg.cache_size)
                    .expect("Failed to open Redb KV store"),
            ))),
            None => balius_runtime::kv::Kv::Mock,
        }
    }
}
impl From<&Config> for balius_runtime::logging::Logger {
    fn from(value: &Config) -> Self {
        match &value.logger {
            Some(LoggerConfig::Silent) => balius_runtime::logging::Logger::Silent,
            Some(LoggerConfig::Tracing) => balius_runtime::logging::Logger::Tracing,
            Some(LoggerConfig::File(cfg)) => balius_runtime::logging::Logger::File(Arc::new(
                Mutex::new(FileLogger::try_new(cfg.folder.clone()).expect("cant open log folder")),
            )),
            None => balius_runtime::logging::Logger::Silent,
        }
    }
}
impl From<&Config> for balius_runtime::sign::Signer {
    fn from(_value: &Config) -> Self {
        // Only one option for now
        balius_runtime::sign::Signer::InMemory(balius_runtime::sign::in_memory::Signer::default())
    }
}
