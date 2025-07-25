use balius_sdk::wit::balius::app as worker;
use balius_sdk::{Config, Error, FnHandler, Json, Params, Utxo, WorkerResult};

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Serialize, Deserialize)]
struct MyConfig {}

#[derive(Serialize, Deserialize)]
struct SayHelloParams {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct SayHelloResponse {
    message: String,
}

fn say_hello(
    _: Config<MyConfig>,
    request: Params<SayHelloParams>,
) -> WorkerResult<Json<SayHelloResponse>> {
    Ok(Json(SayHelloResponse {
        message: format!("Hello, {}!", request.0.name),
    }))
}

#[serde_as]
#[derive(Serialize, Deserialize)]
struct LogParams {
    #[serde_as(as = "DisplayFromStr")]
    level: tracing::Level,
    use_tracing: bool,
    message: String,
}

fn log(_: Config<MyConfig>, request: Params<LogParams>) -> WorkerResult<()> {
    if request.use_tracing {
        match request.0.level {
            tracing::Level::TRACE => tracing::trace!(request.message),
            tracing::Level::DEBUG => tracing::debug!(request.message),
            tracing::Level::INFO => tracing::info!(request.message),
            tracing::Level::WARN => tracing::warn!(request.message),
            tracing::Level::ERROR => tracing::error!(request.message),
        };
    } else {
        let level = match request.0.level {
            tracing::Level::TRACE => worker::logging::Level::Trace,
            tracing::Level::DEBUG => worker::logging::Level::Debug,
            tracing::Level::INFO => worker::logging::Level::Info,
            tracing::Level::WARN => worker::logging::Level::Warn,
            tracing::Level::ERROR => worker::logging::Level::Error,
        };
        worker::logging::log(level, "izquierda", &request.message);
    }

    Ok(())
}

#[serde_as]
#[derive(Serialize, Deserialize)]
struct KvGetParams {
    key: String,
}

#[derive(Serialize, Deserialize)]
struct KvGetResponse {
    value: Option<String>,
}

fn kvget(_: Config<MyConfig>, request: Params<KvGetParams>) -> WorkerResult<Json<KvGetResponse>> {
    Ok(Json(KvGetResponse {
        value: worker::kv::get_value(&request.key)
            .ok()
            .map(|x| String::from_utf8(x).unwrap()),
    }))
}

#[serde_as]
#[derive(Serialize, Deserialize)]
struct KvSetParams {
    key: String,
    value: String,
}

fn kvset(_: Config<MyConfig>, request: Params<KvSetParams>) -> WorkerResult<()> {
    worker::kv::set_value(&request.key, request.value.as_bytes())?;
    Ok(())
}

#[serde_as]
#[derive(Serialize, Deserialize)]
struct KvListParams {
    prefix: String,
}

#[derive(Serialize, Deserialize)]
struct KvListResponse {
    keys: Vec<String>,
}

fn kvlist(
    _: Config<MyConfig>,
    request: Params<KvListParams>,
) -> WorkerResult<Json<KvListResponse>> {
    Ok(Json(KvListResponse {
        keys: worker::kv::list_values(&request.prefix)?,
    }))
}

#[serde_as]
#[derive(Serialize, Deserialize)]
struct SignerGetPublicKeyParams {
    key_name: String,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
struct SignerGetPublicKeyResponse {
    public_key: String,
}

fn signer_get_public_key(
    _: Config<MyConfig>,
    request: Params<SignerGetPublicKeyParams>,
) -> WorkerResult<Json<Option<SignerGetPublicKeyResponse>>> {
    Ok(Json(
        balius_sdk::get_public_keys()
            .get(&request.key_name)
            .map(|pk| SignerGetPublicKeyResponse {
                public_key: hex::encode(pk),
            }),
    ))
}

#[serde_as]
#[derive(Serialize, Deserialize)]
struct SignerSignPayloadParams {
    key_name: String,
    payload: String,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
struct SignerSignPayloadResponse {
    signature: String,
}

fn signer_sign_payload(
    _: Config<MyConfig>,
    request: Params<SignerSignPayloadParams>,
) -> WorkerResult<Json<SignerSignPayloadResponse>> {
    let payload = hex::decode(&request.payload).map_err(|_| Error::BadParams)?;
    let signature = worker::sign::sign_payload(&request.key_name, &payload)?;
    Ok(Json(SignerSignPayloadResponse {
        signature: hex::encode(&signature),
    }))
}

#[serde_as]
#[derive(Serialize, Deserialize)]
struct LedgerSearchUtxosParams {
    address: String,
    max_items: u32,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
struct LedgerSearchUtxosResponse {
    utxos: Vec<String>,
}

fn ledger_search_utxos(
    _: Config<MyConfig>,
    request: Params<LedgerSearchUtxosParams>,
) -> WorkerResult<Json<LedgerSearchUtxosResponse>> {
    let exact_address = hex::decode(&request.address).map_err(|_| Error::BadParams)?;
    let page = worker::ledger::search_utxos(
        &worker::ledger::UtxoPattern {
            address: Some(worker::ledger::AddressPattern { exact_address }),
            asset: None,
        },
        None,
        request.max_items,
    )?;
    let utxos = page
        .utxos
        .iter()
        .map(|x| format!("{}#{}", hex::encode(&x.ref_.tx_hash), x.ref_.tx_index))
        .collect();
    Ok(Json(LedgerSearchUtxosResponse { utxos }))
}

#[derive(Serialize, Deserialize, Clone)]
struct Datum {}

fn handle_utxo(_: Config<MyConfig>, utxo: Utxo<Datum>) -> WorkerResult<()> {
    worker::logging::log(
        worker::logging::Level::Info,
        "handle_utxo",
        "Updating latest utxo in key value",
    );

    if let Err(err) = worker::kv::set_value(
        "latest",
        format!("{}#{}", hex::encode(utxo.tx_hash), utxo.index).as_bytes(),
    ) {
        worker::logging::log(
            worker::logging::Level::Error,
            "handle-utxo",
            &format!("Failed to set latest utxo in kv: {err}"),
        );
    };
    Ok(())
}

#[balius_sdk::main]
fn main() -> balius_sdk::Worker {
    balius_sdk::logging::init();
    balius_sdk::Worker::new()
        .with_utxo_handler(
            worker::driver::UtxoPattern {
                address: None,
                token: None,
            },
            FnHandler::from(handle_utxo),
        )
        .with_request_handler("say-hello", FnHandler::from(say_hello))
        .with_request_handler("log", FnHandler::from(log))
        .with_request_handler("kvget", FnHandler::from(kvget))
        .with_request_handler("kvset", FnHandler::from(kvset))
        .with_request_handler("kvlist", FnHandler::from(kvlist))
        .with_request_handler(
            "signer-get-public-key",
            FnHandler::from(signer_get_public_key),
        )
        .with_request_handler("signer-sign-payload", FnHandler::from(signer_sign_payload))
        .with_request_handler("ledger-search-utxos", FnHandler::from(ledger_search_utxos))
        .with_signer("alice", "ed25519")
        .with_signer("bob", "ed25519")
}
