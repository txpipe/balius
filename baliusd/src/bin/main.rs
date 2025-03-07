use balius_runtime::{drivers, ledgers, Runtime, Store};
use miette::{Context as _, IntoDiagnostic as _};

#[tokio::main]
async fn main() -> miette::Result<()> {
    let config: baliusd::Config = baliusd::boilerplate::load_config(&None)
        .into_diagnostic()
        .context("loading config")?;

    baliusd::boilerplate::setup_tracing(&config.logging).unwrap();

    let store = Store::open("baliusd.db", None)
        .into_diagnostic()
        .context("opening store")?;

    let ledger = ledgers::u5c::Ledger::new(config.ledger.clone())
        .await
        .into_diagnostic()
        .context("setting up ledger")?;

    let runtime = Runtime::builder(store)
        .with_ledger(ledger.into())
        .with_kv(balius_runtime::kv::Kv::Mock)
        .build()
        .into_diagnostic()
        .context("setting up runtime")?;

    let cancel = baliusd::boilerplate::hook_exit_token();

    let jsonrpc_server = async {
        balius_runtime::drivers::jsonrpc::serve(config.rpc.clone(), runtime.clone(), cancel.clone())
            .await
            .into_diagnostic()
            .context("Running JsonRPC server")
    };

    let chainsync_driver = async {
        drivers::chainsync::run(config.chainsync.clone(), runtime.clone(), cancel.clone())
            .await
            .into_diagnostic()
            .context("Running chainsync driver")
    };

    let runtime_update = async { baliusd::update_runtime(&config, runtime.clone()).await };

    tokio::try_join!(jsonrpc_server, chainsync_driver, runtime_update)?;
    Ok(())
}
