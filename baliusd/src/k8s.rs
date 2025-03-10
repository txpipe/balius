use futures_util::TryStreamExt;
use kube::{
    runtime::watcher::{self, Config as ConfigWatcher, Event},
    Api, Client, CustomResource, ResourceExt,
};
use miette::{Context, IntoDiagnostic};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::pin;
use tracing::{error, info, instrument};

use balius_runtime::Runtime;

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    kind = "BaliusWorker",
    group = "txpipe.io",
    version = "v1alpha1",
    category = "balius-worker",
    shortname = "baliusw",
    namespaced
)]
#[kube(printcolumn = r#"
    {"name": "Version", "jsonPath":".spec.version", "type": "string"},
    {"name": "URL", "jsonPath":".spec.url", "type": "string"}
"#)]
#[serde(rename_all = "camelCase")]
pub struct BaliusWorkerSpec {
    pub version: String,
    pub url: String,
    pub config: serde_json::Map<String, Value>,
}

#[instrument("crdwatcher", skip_all)]
pub async fn update_runtime(runtime: Runtime) -> miette::Result<()> {
    let client = Client::try_default()
        .await
        .expect("failed to create kube client");

    let api = Api::<BaliusWorker>::all(client.clone());
    let stream = watcher::watcher(api.clone(), ConfigWatcher::default());
    pin!(stream);

    loop {
        let result = stream.try_next().await;
        match result {
            Ok(Some(Event::Init)) => {
                info!("Watcher restarted, registering workers");
            }

            Ok(Some(Event::InitApply(crd))) => {
                let name = crd.name_any();
                info!("Registering worker: {}", &name);

                match url::Url::parse(&crd.spec.url) {
                    Ok(module) => {
                        runtime
                            .register_worker(&name, module, Value::Object(crd.spec.config))
                            .await
                            .into_diagnostic()
                            .context("registering worker")?;
                    }
                    Err(err) => {
                        error!(
                            err = err.to_string(),
                            "Failed to parse URL for worker: {}", name
                        );
                    }
                };
            }

            Ok(Some(Event::InitDone)) => {
                info!("Workers registered.");
            }

            Ok(Some(Event::Apply(crd))) => {
                let name = crd.name_any();
                info!("Updateted worker: {}", &name);

                match url::Url::parse(&crd.spec.url) {
                    Ok(module) => {
                        runtime
                            .register_worker(&name, module, Value::Object(crd.spec.config))
                            .await
                            .into_diagnostic()
                            .context("registering worker")?;
                    }
                    Err(err) => {
                        error!(
                            err = err.to_string(),
                            "Failed to parse URL for worker: {}", name
                        );
                    }
                };
            }

            Ok(Some(Event::Delete(crd))) => {
                info!("Removing worker: {}", crd.name_any());
                runtime
                    .remove_worker(&crd.name_any())
                    .await
                    .into_diagnostic()
                    .context("removing worker from runtime")?;
            }

            Ok(None) => {
                error!("Empty response from crdwatcher.");
                continue;
            }
            // Unexpected error when streaming CRDs.
            Err(err) => {
                error!(error = err.to_string(), "Error consuming CRDs. Exiting");
                std::process::exit(1);
            }
        }
    }
}
