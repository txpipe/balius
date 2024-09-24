// fn track_utxo(utxo: Utxo) -> Result<()> {
//     bindings::balius::odk::kv::set_value("balance",
// &handler.as_bytes().to_vec()).unwrap();

//     Ok(())
// }

// fn clear_utxo(utxo: Utxo, state: Saved<State>) -> Result<()> {
//     let state = state.get_mut();

//     state.lovelace_balance -= utxo.value.lovelace;

//     Ok(())
// }

// fn read_state(state: Saved<State>) -> Result<State> {
//     let state = state.get();

//     Ok(state)
// }

use balius::{bindings::balius::odk::driver::Event, Env, Json, Result};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Param {
    my_name: String,
}

#[derive(Serialize, Deserialize)]
struct Reply {
    message: String,
}

fn say_hello(_: Env<()>, param: Json<Param>) -> Result<Json<Reply>> {
    Ok(Json(Reply {
        message: format!("Hello, {}!", param.0.my_name),
    }))
}

fn read_state(_: Env<()>, _: Event) -> Result<Json<State>> {
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
fn main() -> balius::Worker {
    balius::Worker::new()
        //.watch_utxo(with_address(config.address), track_utxo)
        //.watch_utxo_spent(with_address(config.address), clear_utxo)
        //.watch_utxo_undo(with_address(config.address), clear_utxo)
        .handle_request("read-state", read_state)
        .handle_request("say-hello", say_hello)
}

balius::entrypoint!(main);
