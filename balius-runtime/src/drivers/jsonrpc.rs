//! Driver to serve JSON-RPC requests.
//!
//! This driver implements an HTTP server that listens for JSON-RPC requests
//! and funnels them into the Runtime. The path of the request is used as the
//! key to identify the worker that should handle the request. The JSON-RPC
//! method field is used as the key to identify the particular Balius request
//! for the worker. JSON-RPC params are mapped directly into Balius request
//! params.
//!
//! The JSON-RPC server is implemented as a Warp application and adheres to
//! the JSON-RPC 2.0 spec.

use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::SocketAddr;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error};
use warp::Filter as _;

use crate::{wit, Error, Runtime};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub listen_address: String,
}

#[derive(Deserialize)]
struct Request {
    pub id: Option<String>,
    pub method: String,
    pub params: serde_json::Value,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

fn parse_request(body: serde_json::Value) -> Result<Request, ErrorResponse> {
    match serde_json::from_value(body) {
        Ok(x) => Ok(x),
        Err(x) => Err(ErrorResponse {
            error: x.to_string(),
        }),
    }
}

pub async fn handle_request(
    runtime: Runtime,
    worker: String,
    body: serde_json::Value,
) -> warp::reply::Json {
    let request = match parse_request(body) {
        Ok(x) => x,
        Err(err) => return warp::reply::json(&err),
    };

    debug!(
        worker,
        id = request.id,
        method = request.method,
        "handling request"
    );

    let params = serde_json::to_vec(&request.params).unwrap();

    let reply = runtime
        .handle_request(&worker, &request.method, params)
        .await;

    match reply {
        Ok(x) => {
            debug!(worker, id = request.id, "request successful");

            let x = match x {
                wit::Response::Acknowledge => json!({}),
                wit::Response::Json(x) => serde_json::from_slice(&x).unwrap(),
                wit::Response::Cbor(x) => json!({ "cbor": hex::encode(x) }),
                wit::Response::PartialTx(x) => json!({ "tx": hex::encode(x) }),
            };

            warp::reply::json(&x)
        }
        Err(err) => {
            error!(worker, id = request.id, "request failed");
            warp::reply::json(&ErrorResponse {
                error: err.to_string(),
            })
        }
    }
}

pub async fn serve(
    config: Config,
    runtime: Runtime,
    cancel: CancellationToken,
) -> Result<(), Error> {
    let filter = warp::any()
        .map(move || -> Runtime { runtime.clone() })
        .and(warp::path::param())
        .and(warp::post())
        .and(warp::body::json())
        .then(handle_request);

    let address: SocketAddr = config
        .listen_address
        .parse()
        .map_err(|x: std::net::AddrParseError| Error::Config(x.to_string()))?;

    let (addr, server) =
        warp::serve(filter).bind_with_graceful_shutdown(address, cancel.cancelled_owned());

    tracing::info!(%addr, "Json-RPC server listening");

    server.await;

    Ok(())
}
