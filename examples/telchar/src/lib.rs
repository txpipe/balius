use balius_sdk::{
    txbuilder::{self, FeeChangeReturn, MinUtxoLovelace, OutputBuilder, TxBuilder, UtxoSource},
    Config, FnHandler, Json, NewTx, Params, WorkerResult,
};

use pallas_primitives::PlutusScript;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct AdminConfig {}

#[derive(Serialize, Deserialize)]
struct DeployRequest {
    fuel_source: UtxoSource,
    deploy_address: String,
    validator_script: String,
}

fn deploy(_: Config<AdminConfig>, req: Params<DeployRequest>) -> WorkerResult<NewTx> {
    let tx = TxBuilder::new()
        .with_input(req.fuel_source.clone())
        .with_output(
            OutputBuilder::new()
                .with_script(PlutusScript::<3>(
                    hex::decode(&req.validator_script).unwrap().into(),
                ))
                .with_value(MinUtxoLovelace)
                .address(req.deploy_address.clone()),
        )
        .with_fee(2_000_000) // TODO: skip once calc is fixed
        .with_output(FeeChangeReturn(req.fuel_source.clone()));

    Ok(NewTx(Box::new(tx)))
}

#[balius_sdk::main]
fn main() -> balius_sdk::Worker {
    balius_sdk::Worker::new().with_request_handler("deploy", FnHandler::from(deploy))
}
