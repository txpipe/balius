#![cfg(test)]
#![cfg(feature = "utxorpc")]

use balius_runtime::{ledgers, Runtime, Store};
use serde_json::json;

#[tokio::test]
async fn faucet_claim() {
    let store = Store::open("tests/balius.db", None).unwrap();

    let ledger = ledgers::u5c::Ledger::new(ledgers::u5c::Config {
        endpoint_url: "https://mainnet.utxorpc-v0.demeter.run".to_string(),
        api_key: "dmtr_utxorpc1wgnnj0qcfj32zxsz2uc8d4g7uclm2s2w".to_string(),
    })
    .await
    .unwrap();

    let mut runtime = Runtime::builder(store)
        .with_ledger(ledger.into())
        .build()
        .unwrap();

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
        .register_worker("faucet", "tests/faucet.wasm", Some(config))
        .await
        .unwrap();

    let req = json!({
      "token": "54455354",
      "quantity": 1,
      "recipient": "addr1qx2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer3n0d3vllmyqwsx5wktcd8cc3sq835lu7drv2xwl2wywfgse35a3x",
      "fuel": {
        "Refs": [
          {
            "hash": "6ac91cdc14155f56c8819740ec334c1e3ff5f9055ba27909045e31ebd329b783",
            "index": 0
          }
        ]
      }
    });

    let res = runtime
        .handle_request("faucet", "claim", req)
        .await
        .unwrap();

    println!("{:?}", res);
}
