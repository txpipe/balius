#![cfg(test)]
#![cfg(feature = "utxorpc")]

use balius_runtime::{drivers, ledgers, Runtime, Store};
use serde_json::json;
use tokio_util::sync::CancellationToken;

#[tokio::test]
async fn wallet_balance() {
    let store = Store::open("tests/balius.db", None).unwrap();

    let mut runtime = Runtime::builder(store).build().unwrap();

    let config = json!({
      "validator": {
        "ref_txo": {
          "transaction_id": "f7d3837715680f3a170e99cd202b726842d97f82c05af8fcd18053c64e33ec4f",
          "index": 0
        },
        "hash": "ef7a1cebb2dc7de884ddf82f8fcbc91fe9750dcd8c12ec7643a99bbe",
        "address": "addr1qx2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer3n0d3vllmyqwsx5wktcd8cc3sq835lu7drv2xwl2wywfgse35a3x"
      }
    });

    runtime
        .register_worker("wallet", "tests/wallet.wasm", config)
        .await
        .unwrap();

    let chainsync_config = drivers::chainsync::Config {
        endpoint_url: "https://mainnet.utxorpc-v0.demeter.run".to_string(),
        api_key: "dmtr_utxorpc1wgnnj0qcfj32zxsz2uc8d4g7uclm2s2w".to_string(),
    };

    drivers::chainsync::run(chainsync_config, runtime, CancellationToken::new())
        .await
        .unwrap();
}
