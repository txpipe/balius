//! Conversions from upstream `utxorpc-spec` 0.18.x types into the
//! Balius-owned schema. The runtime uses these once at chainsync ingress
//! and at ledger reads to produce bytes that workers built against the
//! pre-BigInt SDK can decode.

use crate::cardano as legacy;
use utxorpc_spec::utxorpc::v1alpha::cardano as upstream;

#[derive(Debug)]
pub enum ConvertError {
    /// BigInt is `BigUInt`/`BigNInt`, or `Int(v)` doesn't fit the legacy
    /// uint64/int64 target. Halts the worker per design.
    Overflow,
    /// Re-decoding a wire-stable type with the legacy schema failed —
    /// indicates a wire-format assumption is wrong.
    Decode(prost::DecodeError),
}

impl std::fmt::Display for ConvertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConvertError::Overflow => {
                write!(f, "BigInt value out of range for legacy uint64/int64 target")
            }
            ConvertError::Decode(e) => write!(f, "wire-stable roundtrip decode failed: {e}"),
        }
    }
}

impl std::error::Error for ConvertError {}

impl From<prost::DecodeError> for ConvertError {
    fn from(e: prost::DecodeError) -> Self {
        ConvertError::Decode(e)
    }
}

fn unwrap_u64(b: Option<&upstream::BigInt>) -> Result<u64, ConvertError> {
    match b.and_then(|x| x.big_int.as_ref()) {
        None => Ok(0),
        Some(upstream::big_int::BigInt::Int(v)) if *v >= 0 => Ok(*v as u64),
        _ => Err(ConvertError::Overflow),
    }
}

fn try_map<U, L, F>(items: Vec<U>, f: F) -> Result<Vec<L>, ConvertError>
where
    F: Fn(U) -> Result<L, ConvertError>,
{
    items.into_iter().map(f).collect()
}

/// Re-encodes a wire-stable type via prost roundtrip into its legacy
/// counterpart. Only safe when the upstream and legacy types share an
/// identical wire format — used here for `Datum` and `WitnessSet`,
/// which didn't change between 0.17 and 0.18.
fn roundtrip<U, L>(u: &U) -> Result<L, ConvertError>
where
    U: prost::Message,
    L: prost::Message + Default,
{
    let bytes = u.encode_to_vec();
    L::decode(bytes.as_slice()).map_err(ConvertError::from)
}

fn roundtrip_opt<U, L>(u: &Option<U>) -> Result<Option<L>, ConvertError>
where
    U: prost::Message,
    L: prost::Message + Default,
{
    u.as_ref().map(roundtrip).transpose()
}

impl TryFrom<upstream::Asset> for legacy::Asset {
    type Error = ConvertError;

    fn try_from(a: upstream::Asset) -> Result<Self, Self::Error> {
        let output_coin = match a.quantity {
            Some(upstream::asset::Quantity::OutputCoin(b)) => unwrap_u64(Some(&b))?,
            _ => 0,
        };
        Ok(legacy::Asset {
            name: a.name,
            output_coin,
        })
    }
}

impl TryFrom<upstream::Multiasset> for legacy::Multiasset {
    type Error = ConvertError;

    fn try_from(m: upstream::Multiasset) -> Result<Self, Self::Error> {
        Ok(legacy::Multiasset {
            policy_id: m.policy_id,
            assets: try_map(m.assets, legacy::Asset::try_from)?,
        })
    }
}

impl TryFrom<upstream::TxOutput> for legacy::TxOutput {
    type Error = ConvertError;

    fn try_from(o: upstream::TxOutput) -> Result<Self, Self::Error> {
        Ok(legacy::TxOutput {
            address: o.address,
            coin: unwrap_u64(o.coin.as_ref())?,
            assets: try_map(o.assets, legacy::Multiasset::try_from)?,
            datum: roundtrip_opt(&o.datum)?,
        })
    }
}

impl TryFrom<upstream::TxInput> for legacy::TxInput {
    type Error = ConvertError;

    fn try_from(i: upstream::TxInput) -> Result<Self, Self::Error> {
        Ok(legacy::TxInput {
            tx_hash: i.tx_hash,
            output_index: i.output_index,
            as_output: i.as_output.map(legacy::TxOutput::try_from).transpose()?,
        })
    }
}

impl TryFrom<upstream::Tx> for legacy::Tx {
    type Error = ConvertError;

    fn try_from(t: upstream::Tx) -> Result<Self, Self::Error> {
        Ok(legacy::Tx {
            inputs: try_map(t.inputs, legacy::TxInput::try_from)?,
            outputs: try_map(t.outputs, legacy::TxOutput::try_from)?,
            witnesses: roundtrip_opt(&t.witnesses)?,
            fee: unwrap_u64(t.fee.as_ref())?,
            hash: t.hash,
        })
    }
}

/// Extract `coins_per_utxo_byte` from upstream `PParams`. The runtime
/// uses this to construct the minimal JSON payload that `read-params`
/// hands the SDK — we do not mirror the full `PParams` structure.
pub fn coins_per_utxo_byte(p: &upstream::PParams) -> Result<u64, ConvertError> {
    unwrap_u64(p.coins_per_utxo_byte.as_ref())
}
