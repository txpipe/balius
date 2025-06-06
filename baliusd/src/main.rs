use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use balius_runtime::{drivers, ledgers, logging::file::FileLogger, Runtime, Store};
use boilerplate::{init_meter_provider, metrics_server};
use miette::{Context as _, IntoDiagnostic as _};
use prometheus::Registry;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use tokio::sync::Mutex;
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
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum KvConfig {
    Memory,
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
}

impl From<&Config> for balius_runtime::kv::Kv {
    fn from(value: &Config) -> Self {
        match value.kv {
            Some(KvConfig::Memory) => balius_runtime::kv::Kv::Custom(Arc::new(Mutex::new(
                balius_runtime::kv::memory::MemoryKv::default(),
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

    let registry = Registry::new();
    init_meter_provider(registry.clone())?;
    boilerplate::setup_tracing(&config.logging).unwrap();

    let store = Store::open("baliusd.db", None)
        .into_diagnostic()
        .context("opening store")?;

    let ledger = ledgers::u5c::Ledger::new(&config.ledger)
        .await
        .into_diagnostic()
        .context("setting up ledger")?;

    let mut runtime = Runtime::builder(store)
        .with_ledger(ledger.into())
        .with_kv((&config).into())
        .with_logger((&config).into())
        .with_signer((&config).into())
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

    let metrics_server = tokio::spawn(metrics_server(
        config.metrics.clone(),
        registry.clone(),
        cancel.clone(),
    ));

    let chainsync_driver = tokio::spawn(drivers::chainsync::run(
        config.chainsync,
        runtime.clone(),
        cancel.clone(),
    ));

    let (jsonrpc, chainsync, metrics_server) =
        tokio::try_join!(jsonrpc_server, chainsync_driver, metrics_server).unwrap();

    jsonrpc.unwrap();
    chainsync.unwrap();
    metrics_server.unwrap();

    Ok(())
}
