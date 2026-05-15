use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use utxorpc::CardanoQueryClient;

use crate::wit::balius::app::ledger as wit;

pub mod convert;

impl From<utxorpc::Error> for crate::Error {
    fn from(error: utxorpc::Error) -> Self {
        crate::Error::Ledger(error.to_string())
    }
}

impl From<utxorpc::Error> for wit::LedgerError {
    fn from(error: utxorpc::Error) -> Self {
        wit::LedgerError::Upstream(error.to_string())
    }
}

impl From<wit::TxoRef> for utxorpc::spec::query::TxoRef {
    fn from(value: wit::TxoRef) -> Self {
        utxorpc::spec::query::TxoRef {
            hash: value.tx_hash.into(),
            index: value.tx_index,
        }
    }
}

impl From<utxorpc::spec::query::TxoRef> for wit::TxoRef {
    fn from(value: utxorpc::spec::query::TxoRef) -> Self {
        wit::TxoRef {
            tx_hash: value.hash.into(),
            tx_index: value.index,
        }
    }
}

fn chain_utxo_to_wit(
    value: utxorpc::ChainUtxo<utxorpc::spec::cardano::TxOutput>,
) -> Result<wit::Utxo, wit::LedgerError> {
    use prost::Message;

    let bytes = value
        .parsed
        .map(convert::convert_tx_output)
        .transpose()
        .map_err(|e| wit::LedgerError::Upstream(format!("u5c -> balius_proto conversion: {e}")))?
        .map(|t| t.encode_to_vec())
        .unwrap_or_default();

    Ok(wit::Utxo {
        body: bytes,
        ref_: value.txo_ref.unwrap_or_default().into(),
    })
}

impl From<wit::AddressPattern> for utxorpc::spec::cardano::AddressPattern {
    fn from(value: wit::AddressPattern) -> Self {
        utxorpc::spec::cardano::AddressPattern {
            exact_address: value.exact_address.into(),
            ..Default::default()
        }
    }
}

impl From<wit::AssetPattern> for utxorpc::spec::cardano::AssetPattern {
    fn from(value: wit::AssetPattern) -> Self {
        utxorpc::spec::cardano::AssetPattern {
            policy_id: value.policy.into(),
            asset_name: value.name.map(|n| n.into()).unwrap_or_default(),
        }
    }
}

impl From<wit::UtxoPattern> for utxorpc::spec::cardano::TxOutputPattern {
    fn from(value: wit::UtxoPattern) -> Self {
        utxorpc::spec::cardano::TxOutputPattern {
            address: value.address.map(|a| a.into()),
            asset: value.asset.map(|a| a.into()),
        }
    }
}

fn utxo_page_to_wit(
    value: utxorpc::UtxoPage<utxorpc::Cardano>,
) -> Result<wit::UtxoPage, wit::LedgerError> {
    Ok(wit::UtxoPage {
        utxos: value
            .items
            .into_iter()
            .map(chain_utxo_to_wit)
            .collect::<Result<Vec<_>, _>>()?,
        next_token: value.next,
    })
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    pub endpoint_url: String,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Clone)]
pub struct Ledger {
    queries: utxorpc::CardanoQueryClient,
}

impl Ledger {
    pub async fn new(config: &Config) -> Result<Self, crate::Error> {
        let mut builder = utxorpc::ClientBuilder::new().uri(&config.endpoint_url)?;
        if let Some(headers) = &config.headers {
            for (k, v) in headers.iter() {
                builder = builder.metadata(k, v)?;
            }
        }

        let queries = builder.build::<CardanoQueryClient>().await;

        Ok(Self { queries })
    }

    pub async fn read_utxos(
        &mut self,
        refs: Vec<wit::TxoRef>,
    ) -> Result<Vec<wit::Utxo>, wit::LedgerError> {
        let refs = refs.into_iter().map(|r| r.into()).collect();
        let utxos = self.queries.read_utxos(refs).await?;
        utxos.into_iter().map(chain_utxo_to_wit).collect()
    }

    pub async fn search_utxos(
        &mut self,
        pattern: wit::UtxoPattern,
        start: Option<String>,
        max_items: u32,
    ) -> Result<wit::UtxoPage, wit::LedgerError> {
        let pattern = pattern.into();
        let utxos = self.queries.match_utxos(pattern, start, max_items).await?;
        utxo_page_to_wit(utxos)
    }

    pub async fn read_params(&mut self) -> Result<wit::Json, wit::LedgerError> {
        let res = self
            .queries
            .read_params()
            .await
            .map_err(|err| wit::LedgerError::Upstream(format!("{err:?}")))?;

        let params = res.params.ok_or(wit::LedgerError::Upstream(
            "unexpected response from read_params".to_string(),
        ))?;
        match params {
            utxorpc::spec::query::any_chain_params::Params::Cardano(params) => {
                let json = convert::pparams_to_legacy_json(&params).map_err(|e| {
                    wit::LedgerError::Upstream(format!("u5c -> balius pparams conversion: {e}"))
                })?;
                Ok(serde_json::to_vec(&json).unwrap())
            }
            #[allow(unreachable_patterns)]
            _ => Err(wit::LedgerError::Upstream(
                "unexpected response from read_params".to_string(),
            )),
        }
    }
}
