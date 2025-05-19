use std::sync::Arc;

use tokio::sync::Mutex;

use crate::wit::balius::app::ledger as wit;

pub mod mock;
pub mod u5c;

pub use wit::{Host as CustomLedger, LedgerError, TxoRef, Utxo, UtxoPage, UtxoPattern};

#[derive(Clone)]
#[allow(clippy::large_enum_variant)]
pub enum Ledger {
    Mock(mock::Ledger),
    U5C(u5c::Ledger),
    Custom(Arc<Mutex<dyn wit::Host + Send + Sync>>),
}

impl From<mock::Ledger> for Ledger {
    fn from(ledger: mock::Ledger) -> Self {
        Ledger::Mock(ledger)
    }
}

impl From<u5c::Ledger> for Ledger {
    fn from(ledger: u5c::Ledger) -> Self {
        Ledger::U5C(ledger)
    }
}

#[async_trait::async_trait]
impl wit::Host for Ledger {
    async fn read_utxos(
        &mut self,
        refs: Vec<wit::TxoRef>,
    ) -> Result<Vec<wit::Utxo>, wit::LedgerError> {
        match self {
            Ledger::Mock(ledger) => ledger.read_utxos(refs).await,
            Ledger::U5C(ledger) => ledger.read_utxos(refs).await,
            Ledger::Custom(ledger) => {
                let mut lock = ledger.lock().await;
                lock.read_utxos(refs).await
            }
        }
    }

    async fn search_utxos(
        &mut self,
        pattern: wit::UtxoPattern,
        start: Option<String>,
        max_items: u32,
    ) -> Result<wit::UtxoPage, wit::LedgerError> {
        match self {
            Ledger::Mock(ledger) => ledger.search_utxos(pattern, start, max_items).await,
            Ledger::U5C(ledger) => ledger.search_utxos(pattern, start, max_items).await,
            Ledger::Custom(ledger) => {
                let mut lock = ledger.lock().await;
                lock.search_utxos(pattern, start, max_items).await
            }
        }
    }

    async fn read_params(&mut self) -> Result<wit::Json, wit::LedgerError> {
        match self {
            Ledger::Mock(ledger) => ledger.read_params().await,
            Ledger::U5C(ledger) => ledger.read_params().await,
            Ledger::Custom(ledger) => {
                let mut lock = ledger.lock().await;
                lock.read_params().await
            }
        }
    }
}
