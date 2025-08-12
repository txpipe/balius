use balius_sdk::wit::balius::app as worker;
use balius_sdk::{self as sdk};

use serde::{Deserialize, Serialize};
use utxorpc_spec::utxorpc::v1alpha::cardano::plutus_data::{self};
use utxorpc_spec::utxorpc::v1alpha::cardano::{big_int, BigInt, PlutusData};

#[derive(Serialize, Deserialize, Clone)]
struct SomeConfig {
    custom_hello: Option<String>,
}

fn string_plutus_field(p: Option<&PlutusData>) -> Option<Vec<u8>> {
    match p {
        Some(x) => match x.plutus_data.clone() {
            Some(plutus_data::PlutusData::BoundedBytes(x)) => Some(x.to_vec()),
            _ => None,
        },
        _ => None,
    }
}

fn integer_plutus_field(p: Option<&PlutusData>) -> Option<i64> {
    match p {
        Some(x) => match x.plutus_data.clone() {
            Some(plutus_data::PlutusData::BigInt(x)) => match x.bigint {
                Some(big_int::BigInt::Int(x)) => Some(x),
                Some(big_int::BigInt::BigUInt(_)) => None,
                Some(big_int::BigInt::BigNInt(_)) => None,
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}

#[derive(Serialize, Deserialize)]
struct UtxoHandlerResponse {
    msg: String,
}

#[derive(Serialize, Deserialize)]
struct Datum {}

fn handle_utxo(
    _: sdk::Config<SomeConfig>,
    _utxo: sdk::Utxo<Datum>,
) -> sdk::WorkerResult<sdk::Json<UtxoHandlerResponse>> {
    Ok(sdk::Json(UtxoHandlerResponse {
        msg: String::from("UTxO handled"),
    }))
}

#[balius_sdk::main]
fn main() -> Worker {
    sdk::Worker::new().with_utxo_handler(
        worker::driver::UtxoPattern {
            address: None,
            token: None,
        },
        sdk::FnHandler::from(handle_utxo),
    )
}
