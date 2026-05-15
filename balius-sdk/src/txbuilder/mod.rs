pub(crate) mod asset_math;
pub mod plutus;

#[derive(Debug, thiserror::Error)]
pub enum BuildError {
    #[error("Builder is incomplete")]
    Incomplete,
    #[error("Conflicting requirement")]
    Conflicting,
    #[error("UTxO decoding failed")]
    UtxoDecode,
    #[error("UTxO set is empty")]
    EmptyUtxoSet,
    #[error("Transaction has no inputs")]
    MalformedScript,
    #[error("Could not decode address")]
    MalformedAddress,
    #[error("Could not decode datum bytes")]
    MalformedDatum,
    #[error("Invalid bytes length for datum hash")]
    MalformedDatumHash,
    #[error("Input/policy pointed to by redeemer not found in tx")]
    RedeemerTargetMissing,
    #[error("Invalid network ID")]
    InvalidNetworkId,
    #[error("Corrupted transaction bytes in built transaction")]
    CorruptedTxBytes,
    #[error("Public key for private key is malformed")]
    MalformedKey,
    #[error("Asset name must be 32 bytes or less")]
    AssetNameTooLong,
    #[error("Asset value must be less than 9223372036854775807")]
    AssetValueTooHigh,
    #[error("Total outputs of this transaction are greater than total inputs")]
    OutputsTooHigh,
    #[error("Invalid asset policy id hex")]
    MalformedAssetPolicyIdHex,
    #[error("Malformed TxoRef")]
    MalformedTxoRef,
    #[error("Ledger error: {0}")]
    LedgerError(String),
}

impl From<crate::wit::balius::app::ledger::LedgerError> for BuildError {
    fn from(err: crate::wit::balius::app::ledger::LedgerError) -> Self {
        BuildError::LedgerError(err.to_string())
    }
}

use std::sync::Arc;

pub use pallas_codec as codec;
pub use pallas_primitives::conway as primitives;

/// Protocol parameters surface the SDK currently consumes for tx-building.
///
/// `read-params` hands over the JSON shape that utxorpc-spec 0.17's
/// pbjson serializer produces (camelCase keys, u64s as strings). Workers
/// built against pre-BigInt SDKs deserialize the same payload unchanged
/// via their own pbjson decoder — that is the ABI we are preserving.
///
/// Add fields here as they become needed; serde ignores anything the
/// runtime emits that we don't read.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PParams {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[serde(rename = "coinsPerUtxoByte")]
    pub coins_per_utxo_byte: u64,
}

pub trait Ledger {
    fn read_utxos(&self, refs: &[dsl::TxoRef]) -> Result<dsl::UtxoSet, BuildError>;
    fn search_utxos(&self, pattern: &dsl::UtxoPattern) -> Result<dsl::UtxoSet, BuildError>;
    fn read_params(&self) -> Result<PParams, BuildError>;
}

#[derive(Clone)]
pub struct BuildContext {
    pub network: primitives::NetworkId,
    pub pparams: PParams,
    pub total_input: primitives::Value,
    pub spent_output: primitives::Value,
    pub estimated_fee: u64,
    pub ledger: Arc<Box<dyn Ledger>>,

    pub tx_body: Option<primitives::TransactionBody>,
    pub parent_output: Option<primitives::TransactionOutput>,
}

impl BuildContext {
    pub fn with_parent_output(&self, output: primitives::TransactionOutput) -> Self {
        let mut ctx = self.clone();
        ctx.parent_output = Some(output);
        ctx
    }
}

mod build;
mod dsl;

pub use build::*;
pub use dsl::*;
