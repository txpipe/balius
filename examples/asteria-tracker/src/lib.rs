use balius_sdk::wit::balius::app as worker;
use balius_sdk::{self as sdk};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct SomeConfig {
    custom_hello: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct SayHelloParams {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Reply {
    message: String,
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
