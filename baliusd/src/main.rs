use std::{path::PathBuf, sync::Arc};

use balius_runtime::{
    drivers, ledgers,
    logging::{file::FileLogger, postgres::PostgresLogger},
    Runtime, Store,
};
use miette::{Context as _, IntoDiagnostic as _};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use tokio::sync::{Mutex, RwLock};
use tracing::info;

mod boilerplate;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoggingConfig {
    #[serde_as(as = "DisplayFromStr")]
    max_level: tracing::Level,

    #[serde(default)]
    include_tokio: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PostgresKvConfig {
    connection: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum KvConfig {
    Memory,
    Postgres(PostgresKvConfig),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FileLoggerConfig {
    pub folder: Option<PathBuf>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PostgresLoggerConfig {
    pub connection: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum LoggerConfig {
    Silent,
    Tracing,
    File(FileLoggerConfig),
    Postgres(PostgresLoggerConfig),
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
pub struct Config {
    pub rpc: drivers::jsonrpc::Config,
    pub ledger: ledgers::u5c::Config,
    pub chainsync: drivers::chainsync::Config,
    pub workers: Vec<WorkerConfig>,
    pub logging: LoggingConfig,
    pub kv: Option<KvConfig>,
    pub logger: Option<LoggerConfig>,
}

impl Config {
    pub async fn kv(&self) -> balius_runtime::kv::Kv {
        match &self.kv {
            Some(KvConfig::Memory) => balius_runtime::kv::Kv::Memory(Arc::new(RwLock::new(
                balius_runtime::kv::memory::MemoryKv::default(),
            ))),
            Some(KvConfig::Postgres(cfg)) => {
                balius_runtime::kv::Kv::Postgres(Arc::new(Mutex::new(
                    balius_runtime::kv::postgres::PostgresKv::try_new(&cfg.connection)
                        .await
                        .expect("Failed to connect"),
                )))
            }
            None => balius_runtime::kv::Kv::Mock,
        }
    }

    pub async fn logger(&self) -> balius_runtime::logging::Logger {
        match &self.logger {
            Some(LoggerConfig::Silent) => balius_runtime::logging::Logger::Silent,
            Some(LoggerConfig::Tracing) => balius_runtime::logging::Logger::Tracing,
            Some(LoggerConfig::File(cfg)) => balius_runtime::logging::Logger::File(Arc::new(
                Mutex::new(FileLogger::try_new(cfg.folder.clone()).expect("cant open log folder")),
            )),
            Some(LoggerConfig::Postgres(cfg)) => {
                balius_runtime::logging::Logger::Postgres(Arc::new(Mutex::new(
                    PostgresLogger::try_new(&cfg.connection)
                        .await
                        .expect("failed to connect"),
                )))
            }
            None => balius_runtime::logging::Logger::Silent,
        }
    }
}

fn load_worker_config(config_path: Option<PathBuf>) -> miette::Result<serde_json::Value> {
    match config_path {
        Some(path) => {
            let config_str = std::fs::read_to_string(&path)
                .into_diagnostic()
                .context("reading worker config file")?;

            serde_json::from_str::<serde_json::Value>(&config_str)
                .into_diagnostic()
                .context("parsing worker config as JSON")
        }
        None => Ok(serde_json::Value::Null),
    }
}

#[tokio::main]
async fn main() -> miette::Result<()> {
    let config: Config = boilerplate::load_config(&None)
        .into_diagnostic()
        .context("loading config")?;

    boilerplate::setup_tracing(&config.logging).unwrap();

    let store = Store::open("baliusd.db", None)
        .into_diagnostic()
        .context("opening store")?;

    let ledger = ledgers::u5c::Ledger::new(&config.ledger)
        .await
        .into_diagnostic()
        .context("setting up ledger")?;

    let runtime = Runtime::builder(store)
        .with_ledger(ledger.into())
        .with_kv(config.kv().await)
        .with_logger(config.logger().await)
        .build()
        .into_diagnostic()
        .context("setting up runtime")?;

    for worker in config.workers {
        let config = load_worker_config(worker.config)?;

        runtime
            .register_worker_from_file(&worker.name, worker.module, config)
            .await
            .into_diagnostic()
            .context("registering worker")?;

        info!(name = worker.name, "registered worker");
    }

    let cancel = boilerplate::hook_exit_token();

    let jsonrpc_server = tokio::spawn(balius_runtime::drivers::jsonrpc::serve(
        config.rpc,
        runtime.clone(),
        cancel.clone(),
    ));

    let chainsync_driver = tokio::spawn(drivers::chainsync::run(
        config.chainsync,
        runtime.clone(),
        cancel.clone(),
    ));

    let (jsonrpc, chainsync) = tokio::try_join!(jsonrpc_server, chainsync_driver).unwrap();

    jsonrpc.unwrap();
    chainsync.unwrap();

    Ok(())
}
