// Balius-owned schema. Wire-compatible with utxorpc-spec 0.17.0
// `utxorpc.v1alpha.cardano` for the small subset we expose to WASM.
//
// Field tags match the upstream pre-BigInt schema verbatim, so workers
// built against an old SDK decode these bytes unchanged. Tags absent
// from these structs (script, certificates, withdrawals, mint,
// reference_inputs, validity, collateral, auxiliary, proposals,
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
    #[prost(message, optional, tag = "4")]
    pub datum: ::core::option::Option<Datum>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Datum {
    #[prost(bytes = "bytes", tag = "1")]
    pub hash: ::prost::bytes::Bytes,
    #[prost(message, optional, tag = "2")]
    pub payload: ::core::option::Option<PlutusData>,
    #[prost(bytes = "bytes", tag = "3")]
    pub original_cbor: ::prost::bytes::Bytes,
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
pub struct WitnessSet {
    #[prost(message, repeated, tag = "1")]
    pub vkeywitness: ::prost::alloc::vec::Vec<VKeyWitness>,
    #[prost(message, repeated, tag = "2")]
    pub script: ::prost::alloc::vec::Vec<Script>,
    #[prost(message, repeated, tag = "3")]
    pub plutus_datums: ::prost::alloc::vec::Vec<PlutusData>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tx {
    #[prost(message, repeated, tag = "1")]
    pub inputs: ::prost::alloc::vec::Vec<TxInput>,
    #[prost(message, repeated, tag = "2")]
    pub outputs: ::prost::alloc::vec::Vec<TxOutput>,
    #[prost(message, optional, tag = "7")]
    pub witnesses: ::core::option::Option<WitnessSet>,
    #[prost(uint64, tag = "9")]
    pub fee: u64,
    #[prost(bytes = "bytes", tag = "13")]
    pub hash: ::prost::bytes::Bytes,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VKeyWitness {
    #[prost(bytes = "bytes", tag = "1")]
    pub vkey: ::prost::bytes::Bytes,
    #[prost(bytes = "bytes", tag = "2")]
    pub signature: ::prost::bytes::Bytes,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativeScript {
    #[prost(oneof = "native_script::NativeScript", tags = "1, 2, 3, 4, 5, 6")]
    pub native_script: ::core::option::Option<native_script::NativeScript>,
}

pub mod native_script {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum NativeScript {
        #[prost(bytes, tag = "1")]
        ScriptPubkey(::prost::bytes::Bytes),
        #[prost(message, tag = "2")]
        ScriptAll(super::NativeScriptList),
        #[prost(message, tag = "3")]
        ScriptAny(super::NativeScriptList),
        #[prost(message, tag = "4")]
        ScriptNOfK(super::ScriptNOfK),
        #[prost(uint64, tag = "5")]
        InvalidBefore(u64),
        #[prost(uint64, tag = "6")]
        InvalidHereafter(u64),
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativeScriptList {
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<NativeScript>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScriptNOfK {
    #[prost(uint32, tag = "1")]
    pub k: u32,
    #[prost(message, repeated, tag = "2")]
    pub scripts: ::prost::alloc::vec::Vec<NativeScript>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Constr {
    #[prost(uint32, tag = "1")]
    pub tag: u32,
    #[prost(uint64, tag = "2")]
    pub any_constructor: u64,
    #[prost(message, repeated, tag = "3")]
    pub fields: ::prost::alloc::vec::Vec<PlutusData>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigInt {
    #[prost(oneof = "big_int::BigInt", tags = "1, 2, 3")]
    pub big_int: ::core::option::Option<big_int::BigInt>,
}

pub mod big_int {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum BigInt {
        #[prost(int64, tag = "1")]
        Int(i64),
        #[prost(bytes, tag = "2")]
        BigUInt(::prost::bytes::Bytes),
        #[prost(bytes, tag = "3")]
        BigNInt(::prost::bytes::Bytes),
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlutusDataPair {
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<PlutusData>,
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<PlutusData>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlutusData {
    #[prost(oneof = "plutus_data::PlutusData", tags = "2, 3, 4, 5, 6")]
    pub plutus_data: ::core::option::Option<plutus_data::PlutusData>,
}

pub mod plutus_data {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PlutusData {
        #[prost(message, tag = "2")]
        Constr(super::Constr),
        #[prost(message, tag = "3")]
        Map(super::PlutusDataMap),
        #[prost(message, tag = "4")]
        BigInt(super::BigInt),
        #[prost(bytes, tag = "5")]
        BoundedBytes(::prost::bytes::Bytes),
        #[prost(message, tag = "6")]
        Array(super::PlutusDataArray),
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlutusDataMap {
    #[prost(message, repeated, tag = "1")]
    pub pairs: ::prost::alloc::vec::Vec<PlutusDataPair>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlutusDataArray {
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<PlutusData>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Script {
    #[prost(oneof = "script::Script", tags = "1, 2, 3, 4")]
    pub script: ::core::option::Option<script::Script>,
}

pub mod script {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Script {
        #[prost(message, tag = "1")]
        Native(super::NativeScript),
        #[prost(bytes, tag = "2")]
        PlutusV1(::prost::bytes::Bytes),
        #[prost(bytes, tag = "3")]
        PlutusV2(::prost::bytes::Bytes),
        #[prost(bytes, tag = "4")]
        PlutusV3(::prost::bytes::Bytes),
    }
}
