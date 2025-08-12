use balius_sdk::http::AsHeader;
use balius_sdk::wit::balius::app as worker;
use balius_sdk::{self as sdk};
use serde::{Deserialize, Serialize};
use utxorpc_spec::utxorpc::v1alpha::cardano::plutus_data::{self};
use utxorpc_spec::utxorpc::v1alpha::cardano::{big_int, PlutusData};

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
            Some(plutus_data::PlutusData::BigInt(x)) => match x.big_int {
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

const ASTERIA_ADDRESS: &str = "70b6c5e14f31af0c92515ce156625afc4749e30ceef178cfae1f929fff";
const FUEL_POLICY: &str = "98b1c97b219c102dd0e9ba014481272d6ec069ec3ff47c63e291f1b7";

fn handle_utxo(_: sdk::Config<SomeConfig>, utxo: sdk::Utxo<Datum>) -> sdk::WorkerResult<()> {
    let utxo_addr = hex::encode(utxo.utxo.address.into_bytes());

    if utxo_addr == ASTERIA_ADDRESS {
        // check how many FUEL we have (must traverse UTxO value)
        let mut fuel: u64 = 0;
        let massets = utxo.utxo.assets;
        for masset in &massets {
            // masset has type Multiasset (https://docs.rs/utxorpc-spec/latest/utxorpc_spec/utxorpc/v1alpha/cardano/struct.Multiasset.html)
            if hex::encode(masset.policy_id.into_bytes()) == FUEL_POLICY {
                // asset has type Asset (https://docs.rs/utxorpc-spec/latest/utxorpc_spec/utxorpc/v1alpha/cardano/struct.Asset.html)
                let asset = masset.assets.first().unwrap();
                fuel = asset.output_coin;
            }
        }
        let _ = worker::kv::set_value("fuel", fuel.to_string().as_bytes());
    }

    // manually parse datum
    if let Some(datum) = utxo.utxo.datum {
        let p = datum.payload.unwrap().plutus_data.unwrap();

        match p {
            plutus_data::PlutusData::Constr(x) => {
                let mut f = x.fields.iter();

                if let Some(pos_x) = integer_plutus_field(f.next()) {
                    let _ = worker::kv::set_value("pos_x", &pos_x.to_string().as_bytes());
                };
                if let Some(pos_y) = integer_plutus_field(f.next()) {
                    let _ = worker::kv::set_value("pos_y", &pos_y.to_string().as_bytes());
                }
                if let Some(asset_name) = string_plutus_field(f.next()) {
                    let _ = worker::kv::set_value("asset_name", &asset_name);
                }
            }
            _ => {}
        }
    }
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct KvGetParams {
    key: String,
}

#[derive(Serialize, Deserialize)]
struct KvGetResponse {
    value: Option<String>,
}

fn kvget(
    _: sdk::Config<SomeConfig>,
    request: sdk::Params<KvGetParams>,
) -> sdk::WorkerResult<sdk::Json<KvGetResponse>> {
    Ok(sdk::Json(KvGetResponse {
        value: worker::kv::get_value(&request.key)
            .ok()
            .map(|x| String::from_utf8(x).unwrap()),
    }))
}

#[balius_sdk::main]
fn main() -> Worker {
    sdk::Worker::new()
        .with_utxo_handler(
            worker::driver::UtxoPattern {
                address: None,
                token: None,
            },
            sdk::FnHandler::from(handle_utxo),
        )
        .with_request_handler("kvget", sdk::FnHandler::from(kvget))
}
