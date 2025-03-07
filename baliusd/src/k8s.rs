use aws_sdk_s3::Client as S3Client;
use futures_util::TryStreamExt;
use kube::{
    runtime::watcher::{self, Config as ConfigWatcher, Event},
    Api, Client, CustomResource, ResourceExt,
};
use miette::IntoDiagnostic;
use miette::{miette, Context, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use tokio::pin;
use tracing::{error, info, instrument};

use balius_runtime::Runtime;

use crate::K8sConfig;

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
    {"name": "Key", "jsonPath":".spec.key", "type": "string"}, 
"#)]
#[serde(rename_all = "camelCase")]
pub struct BaliusWorkerSpec {
    pub key: String,
    pub config: serde_json::Map<String, Value>,
}

impl BaliusWorker {
    pub fn module_path(&self, download_dir: &Path) -> miette::Result<PathBuf> {
        let file_name = Path::new(&self.spec.key)
            .file_name()
            .ok_or(miette!("Invalid key"))?;
        Ok(download_dir.join(file_name))
    }
    /// Download WASM file from S3 bucket into download dir, returning full path of WASM file.
    pub async fn download_from_s3(&self, bucket: &str, download_dir: &PathBuf) -> Result<PathBuf> {
        let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
        let client = S3Client::new(&config);

        let resp = client
            .get_object()
            .bucket(bucket)
            .key(&self.spec.key)
            .send()
            .await
            .map_err(|e| miette!(e.to_string()))
            .context("Failed to get object from S3")?;

        let body = resp
            .body
            .collect()
            .await
            .map_err(|e| miette!(e.to_string()))
            .context("Failed to collect object body from S3")?;

        if !download_dir.exists() {
            std::fs::create_dir_all(download_dir)
                .map_err(|e| miette!(e.to_string()))
                .context("Failed to create download directory")?;
        }

        let file_path = self.module_path(download_dir)?;

        let mut file = File::create(&file_path)
            .map_err(|e| miette!(e.to_string()))
            .context("Failed to create file")?;

        file.write_all(&body.into_bytes())
            .map_err(|e| miette!(e.to_string()))
            .context("Failed to write to file")?;

        Ok(file_path)
    }
}

#[instrument("crdwatcher", skip_all)]
pub async fn update_runtime(config: &K8sConfig, runtime: Runtime) -> miette::Result<()> {
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

                let module = crd
                    .download_from_s3(&config.bucket, &config.download_dir)
                    .await?;

                runtime
                    .register_worker(&name, module, Value::Object(crd.spec.config))
                    .await
                    .into_diagnostic()
                    .context("registering worker")?;
            }

            Ok(Some(Event::InitDone)) => {
                info!("Workers registered.");
            }

            Ok(Some(Event::Apply(crd))) => {
                let name = crd.name_any();
                info!("Updateted worker: {}", &name);
                let module = crd
                    .download_from_s3(&config.bucket, &config.download_dir)
                    .await?;

                runtime
                    .register_worker(&name, module, Value::Object(crd.spec.config))
                    .await
                    .into_diagnostic()
                    .context("registering worker")?;
            }

            Ok(Some(Event::Delete(crd))) => {
                info!("Removing worker: {}", crd.name_any());
                let file_path: PathBuf = crd.module_path(&config.download_dir)?;
                // Delete file from filesystem
                if file_path.exists() {
                    tokio::fs::remove_file(&file_path)
                        .await
                        .map_err(|e| miette!(e.to_string()))
                        .with_context(|| {
                            format!("Failed to delete file: {}", file_path.display())
                        })?;
                    info!("Deleted file: {}", file_path.display());
                } else {
                    info!("File not found, skipping deletion: {}", file_path.display());
                }
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
