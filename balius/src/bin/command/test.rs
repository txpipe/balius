use std::{collections::HashMap, path::PathBuf};

use balius_runtime::{ledgers, Runtime, Store};
use miette::{Context as _, IntoDiagnostic as _};
use tokio_util::sync::CancellationToken;
use tracing::{info, warn, Level};
use tracing_subscriber::{filter::Targets, prelude::*};

use balius::utils::get_project_info;

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

#[inline]
#[cfg(unix)]
async fn wait_for_exit_signal() {
    let mut sigterm =
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()).unwrap();

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            warn!("SIGINT detected")
        }
        _ = sigterm.recv() => {
            warn!("SIGTERM detected")
        }
    };

    let _ = std::fs::remove_file("baliusd.db");
}

#[inline]
#[cfg(windows)]
async fn wait_for_exit_signal() {
    tokio::signal::ctrl_c().await.unwrap();

    let _ = std::fs::remove_file("baliusd.db");
}

pub fn hook_exit_token() -> CancellationToken {
    let cancel = CancellationToken::new();

    let cancel2 = cancel.clone();
    tokio::spawn(async move {
        wait_for_exit_signal().await;
        cancel2.cancel();
    });

    cancel
}

pub fn setup_tracing() -> miette::Result<()> {
    let level = Level::INFO;

    let mut filter = Targets::new()
        .with_target("cargo_balius", level)
        .with_target("balius_runtime", level);

    filter = filter
        .with_target("tokio", level)
        .with_target("runtime", level);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

    Ok(())
}

async fn run_project_with_config(
    project_name: &str,
    config_path: Option<PathBuf>,
    port: u16,
    utxo_url: String,
    utxo_api_key: String,
) -> miette::Result<()> {
    setup_tracing()?;

    info!("Running Balius project on daemon...");
    let store: Store = Store::open("baliusd.db", None)
        .into_diagnostic()
        .context("opening store")?;

    let config = ledgers::u5c::Config {
        endpoint_url: utxo_url.clone(),
        headers: Some(HashMap::from([(
            "dmtr-api-key".to_string(),
            utxo_api_key.to_string(),
        )])),
    };
    let ledger = ledgers::u5c::Ledger::new(&config)
        .await
        .into_diagnostic()
        .context("setting up ledger")?;

    let runtime = Runtime::builder(store)
        .with_ledger(ledger.into())
        .with_kv(balius_runtime::kv::Kv::Mock)
        .build()
        .into_diagnostic()
        .context("setting up runtime")?;

    let config: serde_json::Value = load_worker_config(config_path)?;

    let wasm_path = format!("{}-c.wasm", project_name);

    runtime
        .register_worker_from_file(project_name, &wasm_path, config)
        .await
        .into_diagnostic()
        .context(format!("registering worker {}", &wasm_path))?;

    let cancel = hook_exit_token();

    let jsonrpc_server = tokio::spawn(balius_runtime::drivers::jsonrpc::serve(
        balius_runtime::drivers::jsonrpc::Config {
            listen_address: format!("0.0.0.0:{}", port),
        },
        runtime.clone(),
        cancel.clone(),
    ));

    let chainsync_driver = tokio::spawn(balius_runtime::drivers::chainsync::run(
        balius_runtime::drivers::chainsync::Config {
            endpoint_url: utxo_url.clone(),
            headers: Some(HashMap::from([(
                "dmtr-api-key".to_string(),
                utxo_api_key.clone(),
            )])),
        },
        runtime.clone(),
        cancel.clone(),
    ));

    let (jsonrpc, chainsync) = tokio::try_join!(jsonrpc_server, chainsync_driver).unwrap();

    jsonrpc.unwrap();
    chainsync.unwrap();

    Ok(())
}

pub async fn execute(
    config_path: Option<String>,
    port: u16,
    utxo_url: String,
    utxo_api_key: String,
) {
    let (_, package_name) = get_project_info();

    // Convert config_path from String to PathBuf if provided
    let config_path_buf = config_path.map(PathBuf::from);

    let result =
        run_project_with_config(&package_name, config_path_buf, port, utxo_url, utxo_api_key).await;
    if result.is_err() {
        info!("Error running project: {}", result.err().unwrap());
    }
}
