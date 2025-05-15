use balius_sdk::{Config, FnHandler, Params, Json, Worker, WorkerResult};
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

fn say_hello(config: Config<SomeConfig>, params: Params<SayHelloParams>) -> WorkerResult<Json<Reply>> {
    let custom_hello = config.custom_hello.clone().unwrap_or("Hello".to_string());

    Ok(Json(Reply {
        message: format!("{}, {}", custom_hello, params.0.name),
    }))
}

#[balius_sdk::main]
fn main() -> Worker {
    Worker::new().with_request_handler("say-hello", FnHandler::from(say_hello))
} 