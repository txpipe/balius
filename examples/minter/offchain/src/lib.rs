use balius_sdk::{Config, FnHandler, Params, Worker, WorkerResult};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[derive(Serialize, Deserialize, Clone)]
struct FaucetConfig {}

#[serde_as]
#[derive(Serialize, Deserialize)]
struct ClaimRequest {
    quantity: u64,
    password: String,
    requester: String,
}

tx3_lang::include_tx3_build!("faucet");

fn claim(
    config: Config<FaucetConfig>,
    params: Params<ClaimRequest>,
) -> WorkerResult<tx3_lang::ProtoTx> {
    let tx_args = ClaimWithPasswordParams {
        requester: params.requester.clone().into(),
        password: hex::decode(&params.password).unwrap(),
        quantity: params.quantity as i64,
    };

    new_claim_with_password_tx(tx_args).map_err(|e| balius_sdk::Error::Internal(e.to_string()))
}

#[balius_sdk::main]
fn main() -> Worker {
    Worker::new().with_request_handler("claim", FnHandler::from(claim))
}
