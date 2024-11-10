use crate::wit::balius::app::ledger as wit;

pub mod mock;
pub mod u5c;

#[derive(Clone)]
pub enum Ledger {
    Mock(mock::Ledger),
    U5C(u5c::Ledger),
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
        }
    }
}
