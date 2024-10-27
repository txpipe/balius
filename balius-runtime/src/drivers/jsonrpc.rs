use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error};
use warp::Filter as _;

use crate::{Error, Runtime};

#[derive(Serialize, Deserialize, Clone)]
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

pub async fn serve(
    config: Config,
    runtime: Runtime,
    cancel: CancellationToken,
) -> Result<(), Error> {
    let filter = warp::any()
        .map(move || runtime.clone())
        .and(warp::path::param())
        .and(warp::post())
        .and(warp::body::json())
        .then(
            |runtime: Runtime, worker: String, body: serde_json::Value| async move {
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

                let reply = runtime
                    .handle_request(&worker, &request.method, request.params)
                    .await;

                match reply {
                    Ok(x) => {
                        debug!(worker, id = request.id, "request successful");
                        warp::reply::json(&x)
                    }
                    Err(err) => {
                        error!(worker, id = request.id, "request failed");
                        warp::reply::json(&ErrorResponse {
                            error: err.to_string(),
                        })
                    }
                }
            },
        );

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
