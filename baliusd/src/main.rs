use std::path::PathBuf;

use balius_runtime::{drivers, ledgers, Runtime, Store};
use miette::{Context as _, IntoDiagnostic as _};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::{serde_as, DisplayFromStr};
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
    pub workers: Vec<WorkerConfig>,
    pub logging: LoggingConfig,
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

    let ledger = ledgers::u5c::Ledger::new(config.ledger)
        .await
        .into_diagnostic()
        .context("setting up ledger")?;

    let mut runtime = Runtime::builder(store)
        .with_ledger(ledger.into())
        .build()
        .into_diagnostic()
        .context("setting up runtime")?;

    for worker in config.workers {
        let config = load_worker_config(worker.config)?;

        runtime
            .register_worker(&worker.name, worker.module, config)
            .await
            .into_diagnostic()
            .context("registering worker")?;

        info!(name = worker.name, "registered worker");
    }

    let cancel = boilerplate::hook_exit_token();

    balius_runtime::drivers::jsonrpc::serve(config.rpc, runtime, cancel)
        .await
        .into_diagnostic()
        .context("serving json-rpc requests")?;

    Ok(())
}
