#![cfg(test)]

use std::{
    path::{Path, PathBuf},
    process::Command,
};

use balius_runtime::{ledgers, Runtime, Store};
use serde_json::json;
use wit_component::ComponentEncoder;

fn build_module(src_dir: impl AsRef<Path>, module_name: &str, target: impl AsRef<Path>) {
    let output = Command::new("cargo")
        .arg("build")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .current_dir(src_dir.as_ref())
        .output()
        .unwrap();
    if !output.stderr.is_empty() {
        eprintln!("{}", std::str::from_utf8(&output.stderr).unwrap());
    }
    if !output.status.success() {
        panic!("command failed: {}", output.status);
    }

    let wasm_path =
        PathBuf::from("../target/wasm32-unknown-unknown/debug").join(format!("{module_name}.wasm"));
    let module = wat::Parser::new().parse_file(wasm_path).unwrap();
    let component = ComponentEncoder::default()
        .validate(true)
        .module(&module)
        .unwrap()
        .encode()
        .unwrap();

    std::fs::write(target.as_ref(), component).unwrap();
}

#[tokio::test]
async fn faucet_claim() {
    build_module("../examples/minter/offchain", "minter", "tests/faucet.wasm");

    let store = Store::open("tests/balius.db", None).unwrap();

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

    let absolute = std::fs::canonicalize("tests/faucet.wasm").unwrap();
    let url = url::Url::parse(&format!("file://{}", absolute.display())).unwrap();
    runtime
        .register_worker("faucet", url, config)
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
