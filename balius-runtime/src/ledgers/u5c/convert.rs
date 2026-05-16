//! u5c -> balius_core::proto::v0 conversion.
//!
//! Lives in the u5c adapter, not in balius-core: the schema crate must
//! not be coupled to any upstream service. Used at chainsync ingress
//! (`lib.rs::Block::txs`) and at ledger reads (`super::chain_utxo_to_wit`
//! / `super::Ledger::read_params`).
//!
//! Two flavors of output:
//!  - Tx/UTxO/etc. travel as **prost bytes** — `convert_*` functions
//!    produce `balius_core::proto::v0::cardano::*` values, whose wire
//!    format mirrors `utxorpc-spec 0.17` so pre-BigInt workers decode
//!    them unchanged.
//!  - PParams travels as **JSON** — `pparams_to_legacy_json` emits exactly
//!    what `utxorpc-spec 0.17`'s pbjson serializer would produce, so the
//!    same pre-BigInt workers' pbjson decoder reads it unchanged.

use balius_core::proto::v0::cardano as legacy;
use utxorpc::spec::cardano as upstream;

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
        Some(upstream::big_int::BigInt::BigUInt(bytes)) => big_uint_to_u64(bytes),
        _ => Err(ConvertError::Overflow),
    }
}

/// Decode u5c's `BigUInt` (big-endian unsigned bytes) into a `u64`.
/// Returns `Overflow` only when the value truly exceeds `u64::MAX` —
/// leading zero bytes are tolerated so a BigUInt-encoded in-range
/// value is accepted rather than incorrectly halting the worker.
fn big_uint_to_u64(bytes: &[u8]) -> Result<u64, ConvertError> {
    let trimmed = bytes
        .iter()
        .position(|&b| b != 0)
        .map(|i| &bytes[i..])
        .unwrap_or(&[]);
    if trimmed.is_empty() {
        return Ok(0);
    }
    if trimmed.len() > 8 {
        return Err(ConvertError::Overflow);
    }
    let mut buf = [0u8; 8];
    buf[8 - trimmed.len()..].copy_from_slice(trimmed);
    Ok(u64::from_be_bytes(buf))
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

pub fn convert_asset(a: upstream::Asset) -> Result<legacy::Asset, ConvertError> {
    let output_coin = match a.quantity {
        Some(upstream::asset::Quantity::OutputCoin(b)) => unwrap_u64(Some(&b))?,
        _ => 0,
    };
    Ok(legacy::Asset {
        name: a.name,
        output_coin,
    })
}

pub fn convert_multiasset(m: upstream::Multiasset) -> Result<legacy::Multiasset, ConvertError> {
    Ok(legacy::Multiasset {
        policy_id: m.policy_id,
        assets: try_map(m.assets, convert_asset)?,
    })
}

pub fn convert_tx_output(o: upstream::TxOutput) -> Result<legacy::TxOutput, ConvertError> {
    Ok(legacy::TxOutput {
        address: o.address,
        coin: unwrap_u64(o.coin.as_ref())?,
        assets: try_map(o.assets, convert_multiasset)?,
        datum: roundtrip_opt(&o.datum)?,
    })
}

pub fn convert_tx_input(i: upstream::TxInput) -> Result<legacy::TxInput, ConvertError> {
    Ok(legacy::TxInput {
        tx_hash: i.tx_hash,
        output_index: i.output_index,
        as_output: i.as_output.map(convert_tx_output).transpose()?,
    })
}

pub fn convert_tx(t: upstream::Tx) -> Result<legacy::Tx, ConvertError> {
    Ok(legacy::Tx {
        inputs: try_map(t.inputs, convert_tx_input)?,
        outputs: try_map(t.outputs, convert_tx_output)?,
        witnesses: roundtrip_opt(&t.witnesses)?,
        fee: unwrap_u64(t.fee.as_ref())?,
        hash: t.hash,
    })
}

// PParams crosses the WIT boundary as JSON. The wire shape — pbjson
// conventions from utxorpc-spec 0.17 — is owned by balius-core (see
// `balius_core::proto::v0::cardano::PParams` and its nested types). All
// this layer does is flatten upstream BigInts into plain integers and
// copy fields across; serde handles the rest.

fn convert_rational(r: upstream::RationalNumber) -> legacy::RationalNumber {
    legacy::RationalNumber {
        numerator: r.numerator,
        denominator: r.denominator,
    }
}

fn convert_protocol_version(v: upstream::ProtocolVersion) -> legacy::ProtocolVersion {
    legacy::ProtocolVersion {
        major: v.major,
        minor: v.minor,
    }
}

fn convert_ex_units(u: upstream::ExUnits) -> legacy::ExUnits {
    legacy::ExUnits {
        steps: u.steps,
        memory: u.memory,
    }
}

fn convert_ex_prices(p: upstream::ExPrices) -> legacy::ExPrices {
    legacy::ExPrices {
        steps: p.steps.map(convert_rational),
        memory: p.memory.map(convert_rational),
    }
}

fn convert_cost_model(c: upstream::CostModel) -> legacy::CostModel {
    legacy::CostModel { values: c.values }
}

fn convert_cost_models(cm: upstream::CostModels) -> legacy::CostModels {
    legacy::CostModels {
        plutus_v1: cm.plutus_v1.map(convert_cost_model),
        plutus_v2: cm.plutus_v2.map(convert_cost_model),
        plutus_v3: cm.plutus_v3.map(convert_cost_model),
    }
}

fn convert_voting_thresholds(v: upstream::VotingThresholds) -> legacy::VotingThresholds {
    legacy::VotingThresholds {
        thresholds: v.thresholds.into_iter().map(convert_rational).collect(),
    }
}

pub fn convert_pparams(p: upstream::PParams) -> Result<legacy::PParams, ConvertError> {
    Ok(legacy::PParams {
        coins_per_utxo_byte: unwrap_u64(p.coins_per_utxo_byte.as_ref())?,
        max_tx_size: p.max_tx_size,
        min_fee_coefficient: unwrap_u64(p.min_fee_coefficient.as_ref())?,
        min_fee_constant: unwrap_u64(p.min_fee_constant.as_ref())?,
        max_block_body_size: p.max_block_body_size,
        max_block_header_size: p.max_block_header_size,
        stake_key_deposit: unwrap_u64(p.stake_key_deposit.as_ref())?,
        pool_deposit: unwrap_u64(p.pool_deposit.as_ref())?,
        pool_retirement_epoch_bound: p.pool_retirement_epoch_bound,
        desired_number_of_pools: p.desired_number_of_pools,
        pool_influence: p.pool_influence.map(convert_rational),
        monetary_expansion: p.monetary_expansion.map(convert_rational),
        treasury_expansion: p.treasury_expansion.map(convert_rational),
        min_pool_cost: unwrap_u64(p.min_pool_cost.as_ref())?,
        protocol_version: p.protocol_version.map(convert_protocol_version),
        max_value_size: p.max_value_size,
        collateral_percentage: p.collateral_percentage,
        max_collateral_inputs: p.max_collateral_inputs,
        cost_models: p.cost_models.map(convert_cost_models),
        prices: p.prices.map(convert_ex_prices),
        max_execution_units_per_transaction: p.max_execution_units_per_transaction.map(convert_ex_units),
        max_execution_units_per_block: p.max_execution_units_per_block.map(convert_ex_units),
        min_fee_script_ref_cost_per_byte: p.min_fee_script_ref_cost_per_byte.map(convert_rational),
        pool_voting_thresholds: p.pool_voting_thresholds.map(convert_voting_thresholds),
        drep_voting_thresholds: p.drep_voting_thresholds.map(convert_voting_thresholds),
        min_committee_size: p.min_committee_size,
        committee_term_limit: p.committee_term_limit,
        governance_action_validity_period: p.governance_action_validity_period,
        governance_action_deposit: unwrap_u64(p.governance_action_deposit.as_ref())?,
        drep_deposit: unwrap_u64(p.drep_deposit.as_ref())?,
        drep_inactivity_period: p.drep_inactivity_period,
    })
}

#[cfg(test)]
mod tests {
    //! u5c -> balius_core::proto::v0 conversion tests.
    //!
    //! The load-bearing invariants tested here are:
    //!  - Bytes produced by `convert_*` decode unchanged under
    //!    `utxorpc-spec 0.17` prost.
    //!  - JSON produced by `pparams_to_legacy_json` decodes unchanged
    //!    under `utxorpc-spec 0.17` pbjson.
    //! Together these guarantee that pre-BigInt workers keep working.

    use super::*;
    use prost::Message;
    use utxorpc::spec::cardano as v18;
    use utxorpc_spec_017::utxorpc::v1alpha::cardano as v17;

    #[test]
    fn convert_tx_output_int_coin() {
        let upstream = v18::TxOutput {
            address: vec![1, 2, 3].into(),
            coin: Some(v18::BigInt {
                big_int: Some(v18::big_int::BigInt::Int(42)),
            }),
            assets: vec![v18::Multiasset {
                policy_id: vec![9, 9, 9].into(),
                assets: vec![v18::Asset {
                    name: b"asset".to_vec().into(),
                    quantity: Some(v18::asset::Quantity::OutputCoin(v18::BigInt {
                        big_int: Some(v18::big_int::BigInt::Int(7)),
                    })),
                }],
                redeemer: None,
            }],
            datum: None,
            script: None,
        };

        let bal = convert_tx_output(upstream).expect("convert");
        assert_eq!(bal.coin, 42);
        assert_eq!(bal.assets[0].assets[0].output_coin, 7);
    }

    #[test]
    fn convert_overflow_big_uint_errors() {
        let upstream = v18::TxOutput {
            address: vec![].into(),
            coin: Some(v18::BigInt {
                big_int: Some(v18::big_int::BigInt::BigUInt(vec![0xFF; 16].into())),
            }),
            assets: vec![],
            datum: None,
            script: None,
        };
        let res = convert_tx_output(upstream);
        assert!(matches!(res, Err(ConvertError::Overflow)));
    }

    #[test]
    fn convert_big_uint_within_u64_succeeds() {
        // u5c is free to encode an in-range positive value as BigUInt
        // rather than Int — we accept rather than halt when it fits.
        // 5_000_000 == 0x4C 0x4B 0x40 in big-endian.
        let upstream = v18::TxOutput {
            address: vec![].into(),
            coin: Some(v18::BigInt {
                big_int: Some(v18::big_int::BigInt::BigUInt(
                    vec![0x4C, 0x4B, 0x40].into(),
                )),
            }),
            assets: vec![],
            datum: None,
            script: None,
        };
        let bal = convert_tx_output(upstream).expect("convert");
        assert_eq!(bal.coin, 5_000_000);
    }

    #[test]
    fn convert_big_uint_u64_max_succeeds() {
        // Exactly 8 bytes of 0xFF — the u64::MAX boundary.
        let upstream = v18::TxOutput {
            address: vec![].into(),
            coin: Some(v18::BigInt {
                big_int: Some(v18::big_int::BigInt::BigUInt(vec![0xFF; 8].into())),
            }),
            assets: vec![],
            datum: None,
            script: None,
        };
        let bal = convert_tx_output(upstream).expect("convert");
        assert_eq!(bal.coin, u64::MAX);
    }

    #[test]
    fn convert_big_uint_leading_zeros_are_tolerated() {
        // Leading zero bytes are not significant — should not push the
        // value past the u64 boundary check.
        let upstream = v18::TxOutput {
            address: vec![].into(),
            coin: Some(v18::BigInt {
                big_int: Some(v18::big_int::BigInt::BigUInt(
                    vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05].into(),
                )),
            }),
            assets: vec![],
            datum: None,
            script: None,
        };
        let bal = convert_tx_output(upstream).expect("convert");
        assert_eq!(bal.coin, 5);
    }

    #[test]
    fn convert_overflow_negative_to_unsigned_errors() {
        let upstream = v18::TxOutput {
            address: vec![].into(),
            coin: Some(v18::BigInt {
                big_int: Some(v18::big_int::BigInt::Int(-1)),
            }),
            assets: vec![],
            datum: None,
            script: None,
        };
        let res = convert_tx_output(upstream);
        assert!(matches!(res, Err(ConvertError::Overflow)));
    }

    #[test]
    fn convert_tx_fee_int() {
        let upstream = v18::Tx {
            fee: Some(v18::BigInt {
                big_int: Some(v18::big_int::BigInt::Int(9999)),
            }),
            hash: vec![0xCC; 32].into(),
            ..Default::default()
        };
        let bal = convert_tx(upstream).expect("convert");
        assert_eq!(bal.fee, 9999);
        assert_eq!(bal.hash.to_vec(), vec![0xCC; 32]);
    }

    #[test]
    fn convert_then_decode_with_017() {
        // Most load-bearing for Tx/UTxO ABI compat: a worker built against
        // utxorpc-spec 0.17 must be able to decode the bytes the runtime
        // path produces.
        let upstream = v18::TxOutput {
            address: vec![0xAA; 28].into(),
            coin: Some(v18::BigInt {
                big_int: Some(v18::big_int::BigInt::Int(2_500_000)),
            }),
            assets: vec![],
            datum: None,
            script: None,
        };
        let bal: legacy::TxOutput = convert_tx_output(upstream).expect("convert");
        let bytes = bal.encode_to_vec();
        let decoded_017 = v17::TxOutput::decode(bytes.as_slice()).expect("0.17.0 decode");
        assert_eq!(decoded_017.coin, 2_500_000);
        assert_eq!(decoded_017.address.to_vec(), vec![0xAA; 28]);
    }

    #[test]
    fn convert_then_decode_with_017_preserves_datum_payload() {
        // The `roundtrip_opt` helper assumes Datum's wire format is
        // identical between utxorpc-spec 0.17 and 0.18. Round-trip a
        // populated payload through the converter and decode it under
        // 0.17 prost so any future divergence trips at test time
        // instead of as a silent decode error in production.
        let upstream = v18::TxOutput {
            address: vec![0xAA; 28].into(),
            coin: Some(v18::BigInt {
                big_int: Some(v18::big_int::BigInt::Int(1_000_000)),
            }),
            assets: vec![],
            datum: Some(v18::Datum {
                hash: vec![0xEE; 32].into(),
                payload: Some(v18::PlutusData {
                    plutus_data: Some(v18::plutus_data::PlutusData::BoundedBytes(
                        b"datum-payload".to_vec().into(),
                    )),
                }),
                original_cbor: vec![0xFF; 8].into(),
            }),
            script: None,
        };
        let bal = convert_tx_output(upstream).expect("convert");
        let bytes = bal.encode_to_vec();
        let decoded = v17::TxOutput::decode(bytes.as_slice()).expect("0.17.0 decode");
        let datum = decoded.datum.as_ref().expect("datum present");
        assert_eq!(datum.hash.to_vec(), vec![0xEE; 32]);
        assert_eq!(datum.original_cbor.to_vec(), vec![0xFF; 8]);
        let payload = datum.payload.as_ref().expect("datum payload present");
        match payload.plutus_data.as_ref() {
            Some(v17::plutus_data::PlutusData::BoundedBytes(b)) => {
                assert_eq!(b.to_vec(), b"datum-payload".to_vec());
            }
            other => panic!("unexpected datum payload variant: {other:?}"),
        }
    }

    #[test]
    fn convert_then_decode_with_017_preserves_witness_set() {
        // Same `roundtrip_opt` assumption for WitnessSet — round-trip a
        // populated witness set through `convert_tx` and decode under
        // 0.17 prost.
        let upstream = v18::Tx {
            witnesses: Some(v18::WitnessSet {
                vkeywitness: vec![v18::VKeyWitness {
                    vkey: vec![0x11; 32].into(),
                    signature: vec![0x22; 64].into(),
                }],
                script: vec![],
                plutus_datums: vec![v18::PlutusData {
                    plutus_data: Some(v18::plutus_data::PlutusData::BoundedBytes(
                        b"witness-datum".to_vec().into(),
                    )),
                }],
            }),
            hash: vec![0xCC; 32].into(),
            ..Default::default()
        };
        let bal = convert_tx(upstream).expect("convert");
        let bytes = bal.encode_to_vec();
        let decoded = v17::Tx::decode(bytes.as_slice()).expect("0.17.0 decode");
        let witnesses = decoded.witnesses.as_ref().expect("witnesses present");
        assert_eq!(witnesses.vkeywitness.len(), 1);
        assert_eq!(witnesses.vkeywitness[0].vkey.to_vec(), vec![0x11; 32]);
        assert_eq!(witnesses.vkeywitness[0].signature.to_vec(), vec![0x22; 64]);
        assert_eq!(witnesses.plutus_datums.len(), 1);
        match witnesses.plutus_datums[0].plutus_data.as_ref() {
            Some(v17::plutus_data::PlutusData::BoundedBytes(b)) => {
                assert_eq!(b.to_vec(), b"witness-datum".to_vec());
            }
            other => panic!("unexpected plutus datum variant: {other:?}"),
        }
    }

    fn big_int(n: i64) -> v18::BigInt {
        v18::BigInt {
            big_int: Some(v18::big_int::BigInt::Int(n)),
        }
    }

    #[test]
    fn convert_pparams_decodes_under_017_pbjson() {
        // End-to-end: u5c upstream with BigInt fields → convert_pparams →
        // serde_json → utxorpc-spec 0.17 pbjson decode. The shape-only
        // half is also asserted from balius-core
        // (`wire_compat_pparams_roundtrips_via_017`); this test
        // additionally proves BigInt flattening at the conversion seam.
        let upstream = v18::PParams {
            coins_per_utxo_byte: Some(big_int(4310)),
            max_tx_size: 16384,
            min_fee_coefficient: Some(big_int(44)),
            min_fee_constant: Some(big_int(155_381)),
            max_block_body_size: 90112,
            max_block_header_size: 1100,
            stake_key_deposit: Some(big_int(2_000_000)),
            pool_deposit: Some(big_int(500_000_000)),
            pool_retirement_epoch_bound: 18,
            desired_number_of_pools: 500,
            pool_influence: Some(v18::RationalNumber {
                numerator: 3,
                denominator: 10,
            }),
            monetary_expansion: Some(v18::RationalNumber {
                numerator: 3,
                denominator: 1000,
            }),
            treasury_expansion: Some(v18::RationalNumber {
                numerator: 1,
                denominator: 5,
            }),
            min_pool_cost: Some(big_int(170_000_000)),
            protocol_version: Some(v18::ProtocolVersion { major: 10, minor: 0 }),
            max_value_size: 5000,
            collateral_percentage: 150,
            max_collateral_inputs: 3,
            cost_models: Some(v18::CostModels {
                plutus_v1: Some(v18::CostModel {
                    values: vec![100_788, 420, 1, 1],
                }),
                plutus_v2: None,
                plutus_v3: None,
            }),
            prices: Some(v18::ExPrices {
                steps: Some(v18::RationalNumber {
                    numerator: 721,
                    denominator: 10_000_000,
                }),
                memory: Some(v18::RationalNumber {
                    numerator: 577,
                    denominator: 10_000,
                }),
            }),
            max_execution_units_per_transaction: Some(v18::ExUnits {
                steps: 10_000_000_000,
                memory: 14_000_000,
            }),
            max_execution_units_per_block: Some(v18::ExUnits {
                steps: 20_000_000_000,
                memory: 62_000_000,
            }),
            governance_action_deposit: Some(big_int(100_000_000_000)),
            drep_deposit: Some(big_int(500_000_000)),
            min_committee_size: 7,
            ..Default::default()
        };

        let bal = convert_pparams(upstream).expect("convert");
        let bytes = serde_json::to_vec(&bal).unwrap();
        let decoded: v17::PParams = serde_json::from_slice(&bytes).expect("0.17 pbjson decode");

        assert_eq!(decoded.coins_per_utxo_byte, 4310);
        assert_eq!(decoded.max_tx_size, 16384);
        assert_eq!(decoded.min_fee_coefficient, 44);
        assert_eq!(decoded.min_fee_constant, 155_381);
        assert_eq!(decoded.stake_key_deposit, 2_000_000);
        assert_eq!(decoded.pool_deposit, 500_000_000);
        assert_eq!(decoded.min_pool_cost, 170_000_000);
        assert_eq!(decoded.governance_action_deposit, 100_000_000_000);
        assert_eq!(decoded.drep_deposit, 500_000_000);
        assert_eq!(decoded.min_committee_size, 7);
        let pi = decoded.pool_influence.as_ref().unwrap();
        assert_eq!((pi.numerator, pi.denominator), (3, 10));
        assert_eq!(decoded.protocol_version.as_ref().unwrap().major, 10);
        let cm = decoded.cost_models.as_ref().unwrap();
        assert_eq!(
            cm.plutus_v1.as_ref().unwrap().values,
            vec![100_788, 420, 1, 1]
        );
        let prices = decoded.prices.as_ref().unwrap();
        assert_eq!(prices.steps.as_ref().unwrap().numerator, 721);
        assert_eq!(prices.memory.as_ref().unwrap().denominator, 10_000);
        let mex_tx = decoded.max_execution_units_per_transaction.as_ref().unwrap();
        assert_eq!(mex_tx.steps, 10_000_000_000);
        assert_eq!(mex_tx.memory, 14_000_000);
    }

    #[test]
    fn convert_pparams_overflow_propagates() {
        let upstream = v18::PParams {
            coins_per_utxo_byte: Some(v18::BigInt {
                big_int: Some(v18::big_int::BigInt::BigUInt(vec![0xFF; 16].into())),
            }),
            ..Default::default()
        };
        let res = convert_pparams(upstream);
        assert!(matches!(res, Err(ConvertError::Overflow)));
    }
}
