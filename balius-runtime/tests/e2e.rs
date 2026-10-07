#![cfg(test)]

mod util;

use balius_runtime::{ledgers, store::redb::Store as RedbStore, Runtime, Store};
use serde_json::json;

#[tokio::test]
async fn faucet_claim() {
    util::build_module("../examples/minter/offchain", "minter", "tests/faucet.wasm");

    let store = Store::Redb(RedbStore::open("tests/balius.db", None).unwrap());

    let runtime = Runtime::builder(store)
        .with_ledger(ledgers::mock::Ledger.into())
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
        .register_worker_from_file("faucet", "tests/faucet.wasm", config)
        .await
        .unwrap();

    let req = json!({
      "token": "54455354",
      "quantity": 1,
      "recipient": "addr1qx2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer3n0d3vllmyqwsx5wktcd8cc3sq835lu7drv2xwl2wywfgse35a3x",
      "fuel": {
        "Refs": [
          {
            "hash": "f7d3837715680f3a170e99cd202b726842d97f82c05af8fcd18053c64e33ec4f",
            "index": 0
          }
        ]
      }
    });

    let res = runtime
        .handle_request("faucet", "claim", serde_json::to_vec(&req).unwrap())
        .await
        .unwrap();

    println!("{:?}", res);
}
