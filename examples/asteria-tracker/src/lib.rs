use balius_sdk::http::{AsHeader, HttpRequest};
use balius_sdk::wit::balius::app as worker;
use balius_sdk::{self as sdk};
use serde::{Deserialize, Serialize};
use serde_json::json;
use url::Url;

#[derive(Serialize, Deserialize, Clone)]
struct Config {
    discord_webhook: String,
    spacetime_hex_address: String,
    ship_policy: String,
    fuel_policy: String,
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
        // ship name + fuel are derived from the multiasset list. Position is
        // intentionally not tracked here — it lives in the datum, which is
        // outside the minimal balius-proto schema.
        let mut is_valid: bool = false;
        let mut fuel: u64 = 0;
        let mut asset_name = String::new();
        for masset in &utxo.utxo.assets {
            let masset_policy = hex::encode(masset.policy_id.clone().into_bytes());
            if masset_policy == config.ship_policy {
                is_valid = true;
                if let Some(asset) = masset.assets.first() {
                    asset_name = hex::encode(asset.name.clone().into_bytes());
                }
            } else if masset_policy == config.fuel_policy {
                if let Some(asset) = masset.assets.first() {
                    fuel = asset.output_coin;
                }
            }
        }

        if !is_valid || asset_name.is_empty() {
            worker::logging::log(worker::logging::Level::Debug, "Invalid UTxO", &out_ref);
            return Ok(());
        }

        // find out operation: create ship, move ship or gather fuel
        let prev_fuel_res = worker::kv::get_value(&format!("{asset_name}-fuel"));
        let operation = match prev_fuel_res {
            Ok(prev_fuel_bytes) => {
                let prev_fuel_str: String = String::from_utf8(prev_fuel_bytes).unwrap();
                let prev_fuel: u64 = prev_fuel_str.parse().unwrap();
                if fuel < prev_fuel {
                    Operation::MoveShip
                } else {
                    Operation::GatherFuel
                }
            }
            Err(_err) => Operation::CreateShip,
        };

        let fuel_str = fuel.to_string();
        let _ = worker::kv::set_value(&format!("{asset_name}-fuel"), fuel_str.as_bytes());

        let url = Url::parse(&config.discord_webhook).unwrap();
        let asset_name = String::from_utf8(hex::decode(asset_name).unwrap()).unwrap();
        let header = match operation {
            Operation::CreateShip => format!("🚀 new **{asset_name}** detected!"),
            Operation::MoveShip => format!("🚀 **{asset_name}** just moved!"),
            Operation::GatherFuel => format!("🚀 **{asset_name}** just gathered fuel!"),
        };
        let payload = json!({
            "content": format!("{header}\n⛽ Fuel left: {fuel_str}")
        });
        worker::logging::log(
            worker::logging::Level::Debug,
            &format!("UTxO {}:", &out_ref),
            &format!("{operation:#?} - {fuel_str}"),
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
