use std::path::PathBuf;

use balius_runtime::{drivers, ledgers, Error, Runtime};
use miette::{Context as _, IntoDiagnostic as _};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use tracing::info;

pub mod boilerplate;
pub mod k8s;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoggingConfig {
    #[serde_as(as = "DisplayFromStr")]
    max_level: tracing::Level,

    #[serde(default)]
    include_tokio: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct K8sConfig {
    pub bucket: String,
    pub download_dir: PathBuf,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WorkerConfig {
    pub name: String,
    pub module: PathBuf,
    pub config: Option<PathBuf>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Config {
    pub rpc: drivers::jsonrpc::Config,
    pub ledger: ledgers::u5c::Config,
    pub chainsync: drivers::chainsync::Config,
    pub workers: Option<Vec<WorkerConfig>>,
    pub logging: LoggingConfig,
    pub k8s: Option<K8sConfig>,
}

fn load_worker_config(config_path: Option<PathBuf>) -> Result<serde_json::Value, Error> {
    match config_path {
        Some(path) => {
            let config_str =
                std::fs::read_to_string(&path).map_err(|err| Error::Config(err.to_string()))?;

            serde_json::from_str::<serde_json::Value>(&config_str)
                .map_err(|err| Error::Config(err.to_string()))
        }
        None => Ok(serde_json::Value::Null),
    }
}

async fn update_runtime_with_config(config: &Config, runtime: Runtime) -> miette::Result<()> {
    let workers = config
        .workers
        .clone()
        .expect("Workers must be defined when running on static mode.");

    for worker in workers {
        let config = load_worker_config(worker.config)
            .into_diagnostic()
            .context("loading worker config")?;

        runtime
            .register_worker(&worker.name, worker.module, config)
            .await
            .into_diagnostic()
            .context("registering worker")?;

        info!(name = worker.name, "registered worker");
    }

    Ok(())
}

pub async fn update_runtime(config: &Config, runtime: Runtime) -> miette::Result<()> {
    match &config.k8s {
        Some(k8sconfig) => {
            info!("Running on K8s mode.");
            k8s::update_runtime(k8sconfig, runtime.clone()).await
        }
        None => update_runtime_with_config(config, runtime.clone()).await,
    }
}
