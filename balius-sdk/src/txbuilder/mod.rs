pub(crate) mod asset_math;

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
    #[error("Invalid asset policy id hex")]
    MalformedAssetPolicyIdHex,
    #[error("Malformed TxoRef")]
    MalformedTxoRef,
}

pub use pallas_codec as codec;
pub use pallas_primitives::babbage as primitives;

mod build;
mod dsl;

pub use build::*;
pub use dsl::*;
