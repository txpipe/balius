use std::path::PathBuf;

use balius_runtime::{drivers, ledgers, Error, Runtime};
use miette::{bail, Context as _, IntoDiagnostic as _};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use tracing::info;

pub mod boilerplate;
#[cfg(feature = "k8s")]
pub mod k8s;

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
    pub module: String,
    pub config: Option<PathBuf>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Config {
    pub k8s: Option<bool>,
    pub rpc: drivers::jsonrpc::Config,
    pub ledger: ledgers::u5c::Config,
    pub chainsync: drivers::chainsync::Config,
    pub workers: Option<Vec<WorkerConfig>>,
    pub logging: LoggingConfig,
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

        let module = match url::Url::parse(&worker.module) {
            Ok(url) => url,
            Err(err) => match err {
                url::ParseError::RelativeUrlWithoutBase => {
                    // Assume relative local file path.
                    let pathbuf = PathBuf::from(&worker.module);
                    let absolute = std::fs::canonicalize(pathbuf)
                        .into_diagnostic()
                        .context("failed to parse worker module as absolute path")?;
                    url::Url::parse(&format!("file://{}", absolute.display()))
                        .into_diagnostic()
                        .context("parsing worker module as url")?
                }
                _ => bail!("Failed to parse worker module: {}", worker.name),
            },
        };

        runtime
            .register_worker(&worker.name, module, config)
            .await
            .into_diagnostic()
            .context("registering worker")?;

        info!(name = worker.name, "registered worker");
    }

    Ok(())
}

pub async fn update_runtime(config: &Config, runtime: Runtime) -> miette::Result<()> {
    if config.k8s.unwrap_or(false) {
        #[cfg(feature = "k8s")]
        {
            info!("Running on K8s mode.");
            k8s::update_runtime(runtime.clone()).await
        }
        #[cfg(not(feature = "k8s"))]
        {
            panic!("Missing k8s feature flag to run in k8s mode.")
        }
    } else {
        update_runtime_with_config(config, runtime.clone()).await
    }
}

pub fn print_crd() {
    #[cfg(feature = "k8s")]
    {
        use kube::CustomResourceExt;
        print!(
            "{}",
            serde_yaml::to_string(&k8s::BaliusWorker::crd()).unwrap()
        )
    }
    #[cfg(not(feature = "k8s"))]
    {
        panic!("Missing k8s feature flag to run in k8s mode.")
    }
}
