use balius_sdk::http::{AsHeader, HttpRequest};
use balius_sdk::wit::balius::app as worker;
use balius_sdk::{self as sdk};
use serde::{Deserialize, Serialize};
use utxorpc_spec::utxorpc::v1alpha::cardano::plutus_data::{self};
use utxorpc_spec::utxorpc::v1alpha::cardano::{big_int, PlutusData};
use url::Url;

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

const BASE_URL: &str = "http://localhost:8080";
const SPACETIME_ADDRESS: &str = "70b6c5e14f31af0c92515ce156625afc4749e30ceef178cfae1f929fff";
const SHIP_POLICY: &str = "b6c5e14f31af0c92515ce156625afc4749e30ceef178cfae1f929fff";
const FUEL_POLICY: &str = "98b1c97b219c102dd0e9ba014481272d6ec069ec3ff47c63e291f1b7";

fn handle_utxo(_: sdk::Config<SomeConfig>, utxo: sdk::Utxo<Datum>) -> sdk::WorkerResult<()> {
    let utxo_addr = hex::encode(utxo.utxo.address.into_bytes());

    if utxo_addr == SPACETIME_ADDRESS {
        // check how many FUEL we have (must traverse UTxO value)
        let mut is_valid: bool = false;
        let mut fuel: u64 = 0;
        let massets = utxo.utxo.assets;
        for masset in &massets {
            // masset has type Multiasset (https://docs.rs/utxorpc-spec/latest/utxorpc_spec/utxorpc/v1alpha/cardano/struct.Multiasset.html)
            if hex::encode(masset.policy_id.into_bytes()) == SHIP_POLICY {
                is_valid = true;
            } else if hex::encode(masset.policy_id.into_bytes()) == FUEL_POLICY {
                // asset has type Asset (https://docs.rs/utxorpc-spec/latest/utxorpc_spec/utxorpc/v1alpha/cardano/struct.Asset.html)
                let asset = masset.assets.first().unwrap();
                fuel = asset.output_coin;
            }
        }

        if !is_valid {
            return Ok(())
        }

        // manually parse datum
        if let Some(datum) = utxo.utxo.datum {
            let p = datum.payload.unwrap().plutus_data.unwrap();

            match p {
                plutus_data::PlutusData::Constr(x) => {
                    let mut f = x.fields.iter();

                    let pos_x = integer_plutus_field(f.next()).unwrap();
                    let pos_y = integer_plutus_field(f.next()).unwrap();
                    let asset_name = hex::encode(string_plutus_field(f.next()).unwrap());

                    let _ = worker::kv::set_value(
                        &format!("{asset_name}-pos_x"),
                        pos_x.to_string().as_bytes(),
                    );
                    let _ = worker::kv::set_value(
                        &format!("{asset_name}-pos_y"),
                        pos_y.to_string().as_bytes(),
                    );
                    let _ = worker::kv::set_value(
                        &format!("{asset_name}-fuel"),
                        fuel.to_string().as_bytes(),
                    );

                    let pos_x_str = pos_x.to_string();
                    let pos_y_str = pos_y.to_string();
                    let fuel_str = fuel.to_string();

                    let msg = &format!("{BASE_URL}/ship?name={asset_name}&x={pos_x_str}&y={pos_y_str}&fuel={fuel_str}");
                    let url = Url::parse(msg).unwrap();
                    let _ = HttpRequest::get(url).send();
                }
                _ => {}
            }
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
