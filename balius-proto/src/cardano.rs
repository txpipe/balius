// Balius-owned schema. Wire-compatible with utxorpc-spec 0.17.0
// `utxorpc.v1alpha.cardano` for the small subset we expose to WASM.
//
// Field tags match the upstream pre-BigInt schema verbatim, so workers
// built against an old SDK decode these bytes unchanged. Tags absent
// from these structs (datum, script, certificates, witnesses, validity,
// auxiliary, mint, withdrawals, collateral, reference_inputs, proposals,
// successful, ...) are deliberately dropped — workers that need them
// must re-add them here and update the converter.

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxInput {
    #[prost(bytes = "bytes", tag = "1")]
    pub tx_hash: ::prost::bytes::Bytes,
    #[prost(uint32, tag = "2")]
    pub output_index: u32,
    #[prost(message, optional, tag = "3")]
    pub as_output: ::core::option::Option<TxOutput>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxOutput {
    #[prost(bytes = "bytes", tag = "1")]
    pub address: ::prost::bytes::Bytes,
    #[prost(uint64, tag = "2")]
    pub coin: u64,
    #[prost(message, repeated, tag = "3")]
    pub assets: ::prost::alloc::vec::Vec<Multiasset>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Asset {
    #[prost(bytes = "bytes", tag = "1")]
    pub name: ::prost::bytes::Bytes,
    #[prost(uint64, tag = "2")]
    pub output_coin: u64,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Multiasset {
    #[prost(bytes = "bytes", tag = "1")]
    pub policy_id: ::prost::bytes::Bytes,
    #[prost(message, repeated, tag = "2")]
    pub assets: ::prost::alloc::vec::Vec<Asset>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tx {
    #[prost(message, repeated, tag = "1")]
    pub inputs: ::prost::alloc::vec::Vec<TxInput>,
    #[prost(message, repeated, tag = "2")]
    pub outputs: ::prost::alloc::vec::Vec<TxOutput>,
    #[prost(uint64, tag = "9")]
    pub fee: u64,
    #[prost(bytes = "bytes", tag = "13")]
    pub hash: ::prost::bytes::Bytes,
}
