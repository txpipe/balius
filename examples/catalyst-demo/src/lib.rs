use balius::{
    bindings::balius::odk::driver::{Event, HandleError, Response},
    Json,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Param {
    my_name: String,
}

#[derive(Serialize, Deserialize)]
struct Reply {
    message: String,
}

fn say_hello(param: Json<Param>) -> Result<Json<Reply>, HandleError> {
    Ok(Json(Reply {
        message: format!("Hello, {}!", param.0.my_name),
    }))
}

fn read_state(evt: Event) -> Result<Json<State>, HandleError> {
    Ok(Json(State {
        lovelace_balance: 3,
    }))
}

#[derive(Serialize, Deserialize)]
struct State {
    lovelace_balance: u64,
}

#[derive(Default)]
struct Config {
    address: String,
}

//#[balius::main]
fn main(config: Config) -> balius::Worker {
    balius::Worker::new()
        .handle_request("read-state", read_state)
        .handle_request("say-hello", say_hello)
}

balius::entrypoint!(main);
