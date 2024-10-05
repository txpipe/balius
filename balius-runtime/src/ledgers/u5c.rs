use std::sync::Arc;
use utxorpc::CardanoQueryClient;

use crate::wit::balius::app::ledger as wit;

impl From<utxorpc::Error> for crate::Error {
    fn from(error: utxorpc::Error) -> Self {
        crate::Error::Ledger(error.to_string())
    }
}

pub struct Config {
    endpoint_url: String,
    api_key: String,
}

#[derive(Clone)]
pub struct Ledger {
    queries: Arc<utxorpc::CardanoQueryClient>,
}

impl Ledger {
    pub async fn new(config: Config) -> Result<Self, crate::Error> {
        let queries = utxorpc::ClientBuilder::new()
            .uri(&config.endpoint_url)?
            .metadata("dmtr-api-key", config.api_key)?
            .build::<CardanoQueryClient>()
            .await;

        Ok(Self {
            queries: Arc::new(queries),
        })
    }
}

#[async_trait::async_trait]
impl crate::wit::balius::app::ledger::Host for Ledger {
    async fn read_utxos(
        &mut self,
        refs: Vec<wit::TxoRef>,
    ) -> Result<Vec<wit::Utxo>, wit::LedgerError> {
        todo!()
    }

    async fn search_utxos(
        &mut self,
        pattern: wit::UtxoPattern,
    ) -> Result<Vec<wit::Utxo>, wit::LedgerError> {
        todo!()
    }
}
