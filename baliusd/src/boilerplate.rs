use miette::IntoDiagnostic;
use opentelemetry::global;
use opentelemetry_sdk::metrics::SdkMeterProvider;
use prometheus::{Encoder, Registry};
use serde::de::DeserializeOwned;
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing::{debug, warn};
use tracing_subscriber::{filter::Targets, prelude::*};
use warp::{reply::Reply, Filter};

use crate::config::{LoggingConfig, MetricsConfig};

pub fn setup_tracing(config: &LoggingConfig) -> miette::Result<()> {
    let level = config.max_level;

    let mut filter = Targets::new()
        .with_target("baliusd", level)
        .with_target("balius_runtime", level)
        .with_target("gasket", level);

    if config.include_tokio {
        filter = filter
            .with_target("tokio", level)
            .with_target("runtime", level);
    }

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

    Ok(())
}

#[inline]
#[cfg(unix)]
async fn wait_for_exit_signal() {
    let mut sigterm =
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()).unwrap();

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            warn!("SIGINT detected");
        }
        _ = sigterm.recv() => {
            warn!("SIGTERM detected");
        }
    };
}

#[inline]
#[cfg(windows)]
async fn wait_for_exit_signal() {
    tokio::signal::ctrl_c().await.unwrap()
}

pub fn hook_exit_token() -> CancellationToken {
    let cancel = CancellationToken::new();

    let cancel2 = cancel.clone();
    tokio::spawn(async move {
        wait_for_exit_signal().await;
        debug!("notifying exit");
        cancel2.cancel();
    });

    cancel
}

pub async fn run_pipeline(pipeline: gasket::daemon::Daemon, exit: CancellationToken) {
    loop {
        tokio::select! {
            _ = tokio::time::sleep(Duration::from_secs(5000)) => {
                if pipeline.should_stop() {
                    break;
                }
            }
            _ = exit.cancelled() => {
                debug!("exit requested");
                break;
            }
        }
    }

    debug!("shutting down pipeline");
    pipeline.teardown();
}

#[allow(dead_code)]
pub fn spawn_pipeline(pipeline: gasket::daemon::Daemon, exit: CancellationToken) -> JoinHandle<()> {
    tokio::spawn(run_pipeline(pipeline, exit))
}

pub fn load_config<T>(explicit_file: &Option<std::path::PathBuf>) -> Result<T, config::ConfigError>
where
    T: DeserializeOwned,
{
    let mut s = config::Config::builder();

    // our base config will always be in /etc/baliusd
    s = s.add_source(config::File::with_name("/etc/baliusd/daemon.toml").required(false));

    // but we can override it by having a file in the working dir
    s = s.add_source(config::File::with_name("baliusd.toml").required(false));

    // if an explicit file was passed, then we load it as mandatory
    if let Some(explicit) = explicit_file.as_ref().and_then(|x| x.to_str()) {
        s = s.add_source(config::File::with_name(explicit).required(true));
    }

    // finally, we use env vars to make some last-step overrides
    s = s.add_source(config::Environment::with_prefix("BALIUSD").separator("_"));

    s.build()?.try_deserialize()
}

pub fn init_meter_provider(registry: Registry) -> miette::Result<()> {
    let exporter = opentelemetry_prometheus::exporter()
        .with_registry(registry.clone())
        .build()
        .into_diagnostic()?;
    let provider = SdkMeterProvider::builder().with_reader(exporter).build();

    global::set_meter_provider(provider.clone());
    Ok(())
}

async fn metrics_handler(registry: Registry) -> impl Reply {
    let encoder = prometheus::TextEncoder::new();

    let mut buffer = Vec::new();
    if let Err(e) = encoder.encode(&registry.gather(), &mut buffer) {
        eprintln!("could not encode custom metrics: {e}");
    };
    let res = match String::from_utf8(buffer.clone()) {
        Ok(v) => v,
        Err(e) => {
            tracing::error!("custom metrics could not be from_utf8'd: {e}");
            String::default()
        }
    };
    buffer.clear();

    res
}

pub async fn metrics_server(
    config: Option<MetricsConfig>,
    registry: Registry,
    cancel: CancellationToken,
) -> miette::Result<()> {
    if let Some(config) = config.as_ref() {
        let route = warp::path!("metrics")
            .map(move || registry.clone())
            .then(metrics_handler);

        tokio::select! {
            _ = warp::serve(route).run(config.listen_address) => {

            }
            _ = cancel.cancelled() => {
                tracing::warn!("received cancellation, shutting down metrics server");
            }
        }
    }

    Ok(())
}
