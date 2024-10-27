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

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct WorkerConfig {
    pub name: String,
    pub module: PathBuf,
    pub since_slot: Option<u64>,
    pub until_slot: Option<u64>,
    pub config: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub rpc: drivers::jsonrpc::Config,
    pub ledger: ledgers::u5c::Config,
    pub workers: Vec<WorkerConfig>,
    pub logging: LoggingConfig,
}

#[tokio::main]
async fn main() -> miette::Result<()> {
    let config = Config {
        rpc: drivers::jsonrpc::Config {
            listen_address: "0.0.0.0:3000".to_string(),
        },
        workers: vec![WorkerConfig {
            name: "faucet".to_string(),
            module: PathBuf::from("faucet.wasm"),
            since_slot: None,
            until_slot: None,
            config: Some(json!({
                "validator": {
                    "ref_txo": {
                        "transaction_id": "f7d3837715680f3a170e99cd202b726842d97f82c05af8fcd18053c64e33ec4f",
                        "index": 0
                    },
                    "hash": "ef7a1cebb2dc7de884ddf82f8fcbc91fe9750dcd8c12ec7643a99bbe",
                    "address": "addr1qx2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer3n0d3vllmyqwsx5wktcd8cc3sq835lu7drv2xwl2wywfgse35a3x"
                }
            })),
        }],
        logging: LoggingConfig {
            max_level: "debug".parse().unwrap(),
            include_tokio: true,
        },
        ledger: ledgers::u5c::Config {
            endpoint_url: "https://mainnet.utxorpc-v0.demeter.run".to_string(),
            api_key: "dmtr_utxorpc1wgnnj0qcfj32zxsz2uc8d4g7uclm2s2w".to_string(),
        },
    };

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
        runtime
            .register_worker(&worker.name, worker.module, worker.config)
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
