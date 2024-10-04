use balius_sdk::{Config, FnHandler, Json, Params, WorkerResult};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyConfig {
    address: String,
}

#[derive(Serialize, Deserialize)]
struct Request {
    my_name: String,
}

#[derive(Serialize, Deserialize)]
struct Reply {
    message: String,
}

fn say_hello(_: Config<MyConfig>, request: Params<Request>) -> WorkerResult<Json<Reply>> {
    Ok(Json(Reply {
        message: format!("Hello, {}!", request.0.my_name),
    }))
}

fn read_state(_: Config<MyConfig>, _: Params<()>) -> WorkerResult<Json<State>> {
    Ok(Json(State {
        lovelace_balance: 3,
    }))
}

#[derive(Serialize, Deserialize)]
struct State {
    lovelace_balance: u64,
}

#[balius_sdk::main]
fn main() -> balius_sdk::Worker {
    balius_sdk::Worker::new()
        //.watch_utxo(with_address(config.address), track_utxo)
        //.watch_utxo_spent(with_address(config.address), clear_utxo)
        //.watch_utxo_undo(with_address(config.address), clear_utxo)
        .with_request_handler("read-state", FnHandler::from(read_state))
        .with_request_handler("say-hello", FnHandler::from(say_hello))
}
