use std::{collections::HashMap, net::SocketAddr, path::PathBuf, sync::Arc};

use balius_runtime::{
    drivers, ledgers,
    logging::file::FileLogger,
    sign::in_memory::{Ed25519Key, SignerKey},
};
use pallas::crypto::key::ed25519;
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};
use tokio::sync::{Mutex, RwLock};

#[derive(Deserialize, Clone, Debug)]
pub struct StoreConfig {
    pub path: PathBuf,
}

#[serde_as]
#[derive(Deserialize, Debug, Clone)]
pub struct LoggingConfig {
    #[serde_as(as = "DisplayFromStr")]
    pub max_level: tracing::Level,

    #[serde(default)]
    pub include_tokio: bool,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RedbKvConfig {
    pub path: PathBuf,
    pub cache_size: Option<usize>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum KvConfig {
    Memory,
    Redb(RedbKvConfig),
}

#[derive(Deserialize, Clone, Debug)]
pub struct FileLoggerConfig {
    pub folder: Option<PathBuf>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum LoggerConfig {
    Silent,
    Tracing,
    File(FileLoggerConfig),
}

#[derive(Deserialize, Clone, Debug)]
pub struct WorkerConfig {
    pub name: String,
    pub module: PathBuf,
    pub since_slot: Option<u64>,
    pub until_slot: Option<u64>,
    pub config: Option<PathBuf>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct MetricsConfig {
    pub listen_address: SocketAddr,
}

#[derive(Deserialize, Clone, Debug)]
pub struct MemorySignerKeyConfig {
    pub worker: String,
    pub name: String,
    pub algorithm: String,
    #[serde(with = "hex::serde")]
    pub private_key: Vec<u8>,
}

impl From<&MemorySignerKeyConfig> for SignerKey {
    fn from(value: &MemorySignerKeyConfig) -> Self {
        if value.algorithm != "ed25519" {
            panic!("Only ed25519 keys are supported")
        }

        if let Ok(fixed_array) =
            <&[u8; ed25519::SecretKey::SIZE]>::try_from(value.private_key.as_slice())
        {
            return SignerKey::Ed25519(Ed25519Key::SecretKey(ed25519::SecretKey::from(
                fixed_array.to_owned(),
            )));
        }

        if let Ok(fixed_array) =
            <&[u8; ed25519::SecretKeyExtended::SIZE]>::try_from(value.private_key.as_slice())
        {
            if let Ok(key) = ed25519::SecretKeyExtended::from_bytes(fixed_array.to_owned()) {
                return SignerKey::Ed25519(Ed25519Key::SecretKeyExtended(key));
            }
        }

        panic!("Invalid key: {value:?}");
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct MemorySignerConfig {
    pub keys: Option<Vec<MemorySignerKeyConfig>>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum SignerConfig {
    Memory(MemorySignerConfig),
}

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub rpc: drivers::jsonrpc::Config,
    pub ledger: ledgers::u5c::Config,
    pub chainsync: drivers::chainsync::Config,
    pub workers: Vec<WorkerConfig>,
    pub logging: LoggingConfig,
    pub kv: Option<KvConfig>,
    pub logger: Option<LoggerConfig>,
    pub metrics: Option<MetricsConfig>,
    pub signing: Option<SignerConfig>,
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
    fn from(value: &Config) -> Self {
        // Only one option for now
        let signer = if let Some(SignerConfig::Memory(cfg)) = &value.signing {
            if let Some(keys) = &cfg.keys {
                let mut map: HashMap<String, HashMap<String, SignerKey>> = HashMap::new();
                for key in keys {
                    let worker_map = map.entry(key.worker.clone()).or_default();
                    worker_map.insert(key.name.clone(), key.into());
                }
                balius_runtime::sign::in_memory::Signer::from(map)
            } else {
                Default::default()
            }
        } else {
            Default::default()
        };
        balius_runtime::sign::Signer::InMemory(signer)
    }
}
