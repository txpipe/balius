use balius_sdk::{
    txbuilder::{self, FeeChangeReturn, MinUtxoLovelace, OutputBuilder, TxBuilder, UtxoSource},
    Config, FnHandler, Json, NewTx, Params, WorkerResult,
};

use pallas_primitives::PlutusScript;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct AdminConfig {}

#[derive(Serialize, Deserialize)]
struct CreateShipRequest {
    fuel_source: UtxoSource,
}

fn create_ship(_: Config<AdminConfig>, req: Params<CreateShipRequest>) -> WorkerResult<NewTx> {
    let tx = TxBuilder::new();
    Ok(NewTx(Box::new(tx)))
}

#[balius_sdk::main]
fn main() -> balius_sdk::Worker {
    balius_sdk::Worker::new().with_request_handler("create-ship", FnHandler::from(create_ship))
}
