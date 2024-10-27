use serde::{Deserialize, Serialize};
use utxorpc::CardanoQueryClient;

use crate::wit::balius::app::ledger as wit;

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

impl From<utxorpc::ChainUtxo<utxorpc::spec::cardano::TxOutput>> for wit::Utxo {
    fn from(value: utxorpc::ChainUtxo<utxorpc::spec::cardano::TxOutput>) -> Self {
        wit::Utxo {
            body: value.native.into(),
            ref_: value.txo_ref.unwrap_or_default().into(),
        }
    }
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

impl From<utxorpc::UtxoPage<utxorpc::Cardano>> for wit::UtxoPage {
    fn from(value: utxorpc::UtxoPage<utxorpc::Cardano>) -> Self {
        wit::UtxoPage {
            utxos: value.items.into_iter().map(|u| u.into()).collect(),
            next_token: value.next,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    pub endpoint_url: String,
    pub api_key: String,
}

#[derive(Clone)]
pub struct Ledger {
    queries: utxorpc::CardanoQueryClient,
}

impl Ledger {
    pub async fn new(config: Config) -> Result<Self, crate::Error> {
        let queries = utxorpc::ClientBuilder::new()
            .uri(&config.endpoint_url)?
            .metadata("dmtr-api-key", config.api_key)?
            .build::<CardanoQueryClient>()
            .await;

        Ok(Self { queries })
    }

    pub async fn read_utxos(
        &mut self,
        refs: Vec<wit::TxoRef>,
    ) -> Result<Vec<wit::Utxo>, wit::LedgerError> {
        let refs = refs.into_iter().map(|r| r.into()).collect();
        let utxos = self.queries.read_utxos(refs).await?;
        Ok(utxos.into_iter().map(|u| u.into()).collect())
    }

    pub async fn search_utxos(
        &mut self,
        pattern: wit::UtxoPattern,
        start: Option<String>,
        max_items: u32,
    ) -> Result<wit::UtxoPage, wit::LedgerError> {
        let pattern = pattern.into();
        let utxos = self.queries.match_utxos(pattern, start, max_items).await?;
        Ok(utxos.into())
    }
}
