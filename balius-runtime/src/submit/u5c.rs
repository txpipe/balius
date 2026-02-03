use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::wit::balius::app::submit as wit;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    pub endpoint_url: String,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Clone)]
pub struct Submit {
    client: utxorpc::CardanoSubmitClient,
}

impl Submit {
    pub async fn new(config: &Config) -> Result<Self, crate::Error> {
        let mut builder = utxorpc::ClientBuilder::new().uri(&config.endpoint_url)?;
        if let Some(headers) = &config.headers {
            for (k, v) in headers.iter() {
                builder = builder.metadata(k, v)?;
            }
        }

        Ok(Self {
            client: builder.build().await,
        })
    }

    pub async fn submit_tx(&mut self, tx: wit::Cbor) -> Result<(), wit::SubmitError> {
        self.client
            .submit_tx(vec![tx])
            .await
            .map_err(|err| match err {
                utxorpc::Error::GrpcError(status) => {
                    let code: i32 = status.code().into();
                    if code == 3 {
                        wit::SubmitError::Invalid(status.to_string())
                    } else {
                        wit::SubmitError::Internal(status.to_string())
                    }
                }
                utxorpc::Error::TransportError(err) => wit::SubmitError::Internal(err.to_string()),
                utxorpc::Error::ParseError(err) => wit::SubmitError::Internal(err.to_string()),
            })?;
        Ok(())
    }
}
