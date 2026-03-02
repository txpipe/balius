use balius_sdk::http::{AsHeader, HttpRequest};
use balius_sdk::wit::balius::app as worker;
use balius_sdk::{self as sdk};
use serde::{Deserialize, Serialize};
use serde_json::json;
use url::Url;
use utxorpc_spec::utxorpc::v1alpha::cardano::plutus_data::{self};
use utxorpc_spec::utxorpc::v1alpha::cardano::{big_int, PlutusData};

#[derive(Serialize, Deserialize, Clone)]
struct Config {
    discord_webhook: String,
    spacetime_hex_address: String,
    ship_policy: String,
    fuel_policy: String,
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
struct Datum {}

#[derive(Debug)]
enum Operation {
    CreateShip,
    MoveShip,
    GatherFuel,
}

fn handle_utxo(config: sdk::Config<Config>, utxo: sdk::Utxo<Datum>) -> sdk::WorkerResult<()> {
    let tx_hash = hex::encode(utxo.tx_hash);
    let output_index = utxo.index;
    let out_ref = format!("{tx_hash}#{output_index}");
    worker::logging::log(worker::logging::Level::Info, "Handling UTxO", &out_ref);

    let utxo_addr = hex::encode(utxo.utxo.address.into_bytes());

    if utxo_addr == config.spacetime_hex_address {
        // check UTxO validity and obtain fuel amount
        let mut is_valid: bool = false;
        let mut fuel: u64 = 0;
        let massets = utxo.utxo.assets;
        for masset in &massets {
            // masset has type Multiasset (utxorpc_spec)
            let masset_policy = hex::encode(masset.policy_id.into_bytes());
            if masset_policy == config.ship_policy {
                is_valid = true;
            } else if masset_policy == config.fuel_policy {
                // asset has type Asset (utxorpc_spec)
                let asset = masset.assets.first().unwrap();
                fuel = asset.output_coin;
            }
        }

        if !is_valid {
            worker::logging::log(worker::logging::Level::Debug, "Invalid UTxO", &out_ref);
            return Ok(());
        }

        // manually parse datum to obtain ship name and position
        let mut pos_x = 0;
        let mut pos_y = 0;
        let mut asset_name = String::new();
        if let Some(datum) = utxo.utxo.datum {
            let p = datum.payload.unwrap().plutus_data.unwrap();

            if let plutus_data::PlutusData::Constr(x) = p {
                let mut f = x.fields.iter();

                pos_x = integer_plutus_field(f.next()).unwrap();
                pos_y = integer_plutus_field(f.next()).unwrap();
                asset_name = hex::encode(string_plutus_field(f.next()).unwrap());
            }
        }

        // find out operation: create ship, move ship or gather fuel
        let prev_fuel_res = worker::kv::get_value(&format!("{asset_name}-fuel"));
        let operation = match prev_fuel_res {
            Ok(prev_fuel_bytes) => {
                // this is an existing ship
                // did it move or gather fuel?
                let prev_fuel_str: String = String::from_utf8(prev_fuel_bytes).unwrap();
                let prev_fuel: u64 = prev_fuel_str.parse().unwrap();
                if fuel < prev_fuel {
                    // it consumed fuel, so it moved
                    Operation::MoveShip
                } else {
                    // it gathered fuel
                    Operation::GatherFuel
                }
            }
            Err(_err) => {
                // this is a new ship (at least in the storage)
                Operation::CreateShip
            }
        };

        // save ship information in storage
        let pos_x_str = pos_x.to_string();
        let pos_y_str = pos_y.to_string();
        let fuel_str = fuel.to_string();

        let _ = worker::kv::set_value(&format!("{asset_name}-pos_x"), pos_x_str.as_bytes());
        let _ = worker::kv::set_value(&format!("{asset_name}-pos_y"), pos_y_str.as_bytes());
        let _ = worker::kv::set_value(&format!("{asset_name}-fuel"), fuel_str.as_bytes());

        // send notification
        let url = Url::parse(&config.discord_webhook).unwrap();
        let asset_name = String::from_utf8(hex::decode(asset_name).unwrap()).unwrap();
        let header = match operation {
            Operation::CreateShip => format!("🚀 new **{asset_name}** detected!"),
            Operation::MoveShip => format!("🚀 **{asset_name}** just moved!"),
            Operation::GatherFuel => format!("🚀 **{asset_name}** just gathered fuel!"),
        };
        let payload = json!({
            "content":
                format!(
                    "{header}\n📍 Position: ({pos_x_str}, {pos_y_str})\n⛽ Fuel left: {fuel_str}"
                )
        });
        worker::logging::log(
            worker::logging::Level::Debug,
            &format!("UTxO {}:", &out_ref),
            &format!("{operation:#?} - {pos_x_str} - {pos_y_str} - {fuel_str}"),
        );

        let _ = HttpRequest::post(url).json(&payload)?.send()?;
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
    _: sdk::Config<Config>,
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
    balius_sdk::logging::init();
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
