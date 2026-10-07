#![cfg(test)]

mod util;

use std::{collections::HashMap, sync::Arc, time::Duration};

use balius_runtime::{
    drivers,
    kv::{memory::MemoryKv, Kv, KvProvider as _},
    store::redb::Store as RedbStore,
    Runtime, Store,
};
use serde_json::json;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

#[tokio::test]
async fn wallet_balance() {
    util::build_module("../examples/wallet/offchain", "wallet", "tests/wallet.wasm");

    let store = Store::Redb(RedbStore::in_memory().unwrap());
    let kv = Arc::new(RwLock::new(MemoryKv::default()));
    let runtime = Runtime::builder(store)
        .with_kv(Kv::Memory(kv.clone()))
        .build()
        .unwrap();
    let config = json!({
        "address": "addr1qx2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer3n0d3vllmyqwsx5wktcd8cc3sq835lu7drv2xwl2wywfgse35a3x"
    });

    let wasm = std::fs::read("tests/wallet.wasm").unwrap();
    runtime
        .register_worker("wallet", &wasm, config)
        .await
        .unwrap();

    let chainsync_config = drivers::chainsync::Config {
        endpoint_url: "https://cardano-mainnet.utxorpc-m1.demeter.run".to_string(),
        headers: Some(HashMap::from([(
            "dmtr-api-key".to_string(),
            "utxorpc1af7qyjwrhw8pgpkwe32".to_string(),
        )])),
    };

    let cancel = CancellationToken::new();
    let driver = tokio::spawn(drivers::chainsync::run(
        chainsync_config,
        runtime.clone(),
        cancel.clone(),
    ));

    // Wait until the cursor has advanced and a balance entry exists. The worker
    // only writes a balance when it handles a utxo event, so a block can be
    // committed while the table is still empty.
    let balances = tokio::time::timeout(Duration::from_secs(120), async {
        loop {
            if runtime.chain_cursor().await.unwrap().is_some() {
                let keys = kv
                    .write()
                    .await
                    .list_values("wallet", "balances/".to_string())
                    .await
                    .unwrap();
                if !keys.is_empty() {
                    break keys;
                }
            }
            if driver.is_finished() {
                panic!("chain-sync driver exited before applying a block");
            }
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    })
    .await
    .expect("timed out waiting for a balance entry");

    let value = kv
        .write()
        .await
        .get_value("wallet", balances[0].clone())
        .await
        .unwrap();
    let _coin: u64 = pallas::codec::minicbor::decode(&value).unwrap();

    cancel.cancel();
    driver.await.unwrap().unwrap();
}
