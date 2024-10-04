use balius_sdk::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct FaucetConfig {
    validator: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct FaucetRequest {
    token: String,
    quantity: u64,
    recipient: String,
    fuel: String,
}

fn on_request(config: Config<FaucetConfig>, params: Params<FaucetRequest>) -> WorkerResult<NewTx> {
    todo!()
}

#[balius_sdk::main]
fn balius_main() -> Worker {
    Worker::new().with_request_handler("faucet", FnHandler::from(on_request))
}

fn main() {
    panic!("not meant to be run directly");
}
