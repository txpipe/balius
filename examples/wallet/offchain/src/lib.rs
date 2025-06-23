use std::marker::PhantomData;

use balius_sdk::{Ack, UtxoMatcher, WorkerResult};
use balius_sdk::{Config, FnHandler, Utxo, Worker};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct WalletConfig {
    address: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Datum {}

struct KvTable<V> {
    namespace: String,
    _value: PhantomData<V>,
}

impl<V> KvTable<V>
where
    V: pallas_codec::minicbor::Encode<()>,
{
    fn new(namespace: String) -> Self {
        Self {
            namespace,
            _value: PhantomData,
        }
    }

    fn set(&self, key: &str, value: V) -> Result<(), balius_sdk::Error> {
        let key = format!("{}/{}", self.namespace, key);
        let value = pallas_codec::minicbor::to_vec(&value).unwrap();
        balius_sdk::wit::balius::app::kv::set_value(&key, &value.into()).unwrap();
        Ok(())
    }
}

type BalanceTable = KvTable<u64>;

fn handle_utxo(_: Config<WalletConfig>, utxo: Utxo<Datum>) -> WorkerResult<Ack> {
    let balances = BalanceTable::new("balances".to_string());

    balances
        .set(&hex::encode(&utxo.utxo.address), utxo.utxo.coin)
        .unwrap();

    Ok(Ack)
}

#[balius_sdk::main]
fn main() -> Worker {
    Worker::new().with_utxo_handler(UtxoMatcher::all(), FnHandler::from(handle_utxo))
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     use balius_sdk::txbuilder::{primitives, Address, Hash, UtxoSet};
//
//     use std::{collections::HashMap, str::FromStr as _};
//
//     #[test]
//     fn test_happy_path() {
//         let output =
//     primitives::MintedTransactionOutput::PostAlonzo(primitives::MintedPostAlonzoTransactionOutput
//     {         address:
//     Address::from_bech32("
//     addr1qx2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer3n0d3vllmyqwsx5wktcd8cc3sq835lu7drv2xwl2wywfgse35a3x"
//     ).unwrap().to_vec().into(),         value:
//     primitives::Value::Coin(5_000_000),         datum_option: None,
//             script_ref: None,
//         });
//
//         let cbor = pallas_codec::minicbor::to_vec(&output).unwrap();
//
//         let test_utxos: HashMap<_, _> = vec![(
//
// "f7d3837715680f3a170e99cd202b726842d97f82c05af8fcd18053c64e33ec4f#0"
//                 .parse()
//                 .unwrap(),
//             cbor,
//         )]
//         .into_iter()
//         .collect();
//
//         let config = WalletConfig {
//             address:
//     "addr1qx2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer3n0d3vllmyqwsx5wktcd8cc3sq835lu7drv2xwl2wywfgse35a3x"
//     .into(),     };
//
//         handle_utxo(config, utxo).unwrap();
//     }
// }
