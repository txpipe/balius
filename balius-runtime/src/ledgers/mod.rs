use crate::wit::balius::app::ledger as wit;

pub mod mock;

#[cfg(feature = "utxorpc")]
pub mod u5c;

#[derive(Clone)]
pub enum Ledger {
    Mock(mock::Ledger),

    #[cfg(feature = "utxorpc")]
    U5C(u5c::Ledger),
}

impl From<mock::Ledger> for Ledger {
    fn from(ledger: mock::Ledger) -> Self {
        Ledger::Mock(ledger)
    }
}

#[cfg(feature = "utxorpc")]
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

            #[cfg(feature = "utxorpc")]
            Ledger::U5C(ledger) => ledger.read_utxos(refs).await,
        }
    }

    async fn search_utxos(
        &mut self,
        pattern: wit::UtxoPattern,
    ) -> Result<Vec<wit::Utxo>, wit::LedgerError> {
        match self {
            Ledger::Mock(ledger) => ledger.search_utxos(pattern).await,

            #[cfg(feature = "utxorpc")]
            Ledger::U5C(ledger) => ledger.search_utxos(pattern).await,
        }
    }
}
