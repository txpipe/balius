use std::path::PathBuf;

use balius_runtime::{drivers, ledgers, Runtime, Store};
use boilerplate::{init_meter_provider, metrics_server};
use miette::{Context as _, IntoDiagnostic as _};
use prometheus::Registry;
use tracing::info;

mod boilerplate;
mod config;

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
    let config: config::Config = boilerplate::load_config(&None)
        .into_diagnostic()
        .context("loading config")?;

    let registry = Registry::new();
    init_meter_provider(registry.clone())?;
    boilerplate::setup_tracing(&config.logging).unwrap();

    let store = match config.store.as_ref() {
        Some(cfg) => Store::open(cfg.path.clone(), None)
            .into_diagnostic()
            .context("opening store")?,
        None => Store::in_memory()
            .into_diagnostic()
            .context("opening in memory store")?,
    };

    let ledger = ledgers::u5c::Ledger::new(&config.ledger)
        .await
        .into_diagnostic()
        .context("setting up ledger")?;

    let runtime = Runtime::builder(store)
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
