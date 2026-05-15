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

// Convert upstream `PParams` to the JSON shape that pre-BigInt workers
// expect: pbjson conventions from utxorpc-spec 0.17 — camelCase keys,
// u64/i64 emitted as JSON strings, u32/i32 as plain numbers,
// None/default-zero fields omitted. BigInt-typed fields (which were
// uint64 in 0.17) are flattened with `unwrap_u64` and overflow halts.
//
// Built imperatively rather than via a mirror struct because PParams
// crosses the WIT boundary as JSON, not protobuf — there is nothing for
// prost wire-compat to anchor against.

fn rational_to_json(r: &upstream::RationalNumber) -> serde_json::Value {
    let mut o = serde_json::Map::new();
    if r.numerator != 0 {
        o.insert("numerator".into(), serde_json::Value::from(r.numerator));
    }
    if r.denominator != 0 {
        o.insert("denominator".into(), serde_json::Value::from(r.denominator));
    }
    serde_json::Value::Object(o)
}

fn protocol_version_to_json(v: &upstream::ProtocolVersion) -> serde_json::Value {
    let mut o = serde_json::Map::new();
    if v.major != 0 {
        o.insert("major".into(), serde_json::Value::from(v.major));
    }
    if v.minor != 0 {
        o.insert("minor".into(), serde_json::Value::from(v.minor));
    }
    serde_json::Value::Object(o)
}

fn ex_units_to_json(u: &upstream::ExUnits) -> serde_json::Value {
    let mut o = serde_json::Map::new();
    if u.steps != 0 {
        o.insert("steps".into(), serde_json::Value::String(u.steps.to_string()));
    }
    if u.memory != 0 {
        o.insert("memory".into(), serde_json::Value::String(u.memory.to_string()));
    }
    serde_json::Value::Object(o)
}

fn ex_prices_to_json(p: &upstream::ExPrices) -> serde_json::Value {
    let mut o = serde_json::Map::new();
    if let Some(s) = &p.steps {
        o.insert("steps".into(), rational_to_json(s));
    }
    if let Some(m) = &p.memory {
        o.insert("memory".into(), rational_to_json(m));
    }
    serde_json::Value::Object(o)
}

fn cost_model_to_json(c: &upstream::CostModel) -> serde_json::Value {
    let mut o = serde_json::Map::new();
    if !c.values.is_empty() {
        let arr: Vec<serde_json::Value> = c
            .values
            .iter()
            .map(|v| serde_json::Value::String(v.to_string()))
            .collect();
        o.insert("values".into(), serde_json::Value::Array(arr));
    }
    serde_json::Value::Object(o)
}

fn cost_models_to_json(cm: &upstream::CostModels) -> serde_json::Value {
    let mut o = serde_json::Map::new();
    if let Some(v) = &cm.plutus_v1 {
        o.insert("plutusV1".into(), cost_model_to_json(v));
    }
    if let Some(v) = &cm.plutus_v2 {
        o.insert("plutusV2".into(), cost_model_to_json(v));
    }
    if let Some(v) = &cm.plutus_v3 {
        o.insert("plutusV3".into(), cost_model_to_json(v));
    }
    serde_json::Value::Object(o)
}

fn voting_thresholds_to_json(v: &upstream::VotingThresholds) -> serde_json::Value {
    let mut o = serde_json::Map::new();
    if !v.thresholds.is_empty() {
        let arr: Vec<serde_json::Value> = v.thresholds.iter().map(rational_to_json).collect();
        o.insert("thresholds".into(), serde_json::Value::Array(arr));
    }
    serde_json::Value::Object(o)
}

/// Produce the JSON payload that `read-params` hands the SDK. The shape
/// is exactly what `utxorpc-spec 0.17`'s pbjson serializer would emit
/// for a `PParams` message — so workers compiled against pre-BigInt
/// SDKs deserialize it unchanged.
pub fn pparams_to_legacy_json(
    p: &upstream::PParams,
) -> Result<serde_json::Value, ConvertError> {
    use serde_json::Value;
    let mut o = serde_json::Map::new();

    let put_u64_str = |o: &mut serde_json::Map<String, Value>, k: &str, v: u64| {
        if v != 0 {
            o.insert(k.into(), Value::String(v.to_string()));
        }
    };

    let cput = unwrap_u64(p.coins_per_utxo_byte.as_ref())?;
    put_u64_str(&mut o, "coinsPerUtxoByte", cput);
    put_u64_str(&mut o, "maxTxSize", p.max_tx_size);
    let mfc = unwrap_u64(p.min_fee_coefficient.as_ref())?;
    put_u64_str(&mut o, "minFeeCoefficient", mfc);
    let mfcst = unwrap_u64(p.min_fee_constant.as_ref())?;
    put_u64_str(&mut o, "minFeeConstant", mfcst);
    put_u64_str(&mut o, "maxBlockBodySize", p.max_block_body_size);
    put_u64_str(&mut o, "maxBlockHeaderSize", p.max_block_header_size);
    let skd = unwrap_u64(p.stake_key_deposit.as_ref())?;
    put_u64_str(&mut o, "stakeKeyDeposit", skd);
    let pd = unwrap_u64(p.pool_deposit.as_ref())?;
    put_u64_str(&mut o, "poolDeposit", pd);
    put_u64_str(&mut o, "poolRetirementEpochBound", p.pool_retirement_epoch_bound);
    put_u64_str(&mut o, "desiredNumberOfPools", p.desired_number_of_pools);
    if let Some(v) = &p.pool_influence {
        o.insert("poolInfluence".into(), rational_to_json(v));
    }
    if let Some(v) = &p.monetary_expansion {
        o.insert("monetaryExpansion".into(), rational_to_json(v));
    }
    if let Some(v) = &p.treasury_expansion {
        o.insert("treasuryExpansion".into(), rational_to_json(v));
    }
    let mpc = unwrap_u64(p.min_pool_cost.as_ref())?;
    put_u64_str(&mut o, "minPoolCost", mpc);
    if let Some(v) = &p.protocol_version {
        o.insert("protocolVersion".into(), protocol_version_to_json(v));
    }
    put_u64_str(&mut o, "maxValueSize", p.max_value_size);
    put_u64_str(&mut o, "collateralPercentage", p.collateral_percentage);
    put_u64_str(&mut o, "maxCollateralInputs", p.max_collateral_inputs);
    if let Some(v) = &p.cost_models {
        o.insert("costModels".into(), cost_models_to_json(v));
    }
    if let Some(v) = &p.prices {
        o.insert("prices".into(), ex_prices_to_json(v));
    }
    if let Some(v) = &p.max_execution_units_per_transaction {
        o.insert(
            "maxExecutionUnitsPerTransaction".into(),
            ex_units_to_json(v),
        );
    }
    if let Some(v) = &p.max_execution_units_per_block {
        o.insert("maxExecutionUnitsPerBlock".into(), ex_units_to_json(v));
    }
    if let Some(v) = &p.min_fee_script_ref_cost_per_byte {
        o.insert("minFeeScriptRefCostPerByte".into(), rational_to_json(v));
    }
    if let Some(v) = &p.pool_voting_thresholds {
        o.insert("poolVotingThresholds".into(), voting_thresholds_to_json(v));
    }
    if let Some(v) = &p.drep_voting_thresholds {
        o.insert("drepVotingThresholds".into(), voting_thresholds_to_json(v));
    }
    if p.min_committee_size != 0 {
        o.insert(
            "minCommitteeSize".into(),
            Value::from(p.min_committee_size),
        );
    }
    put_u64_str(&mut o, "committeeTermLimit", p.committee_term_limit);
    put_u64_str(
        &mut o,
        "governanceActionValidityPeriod",
        p.governance_action_validity_period,
    );
    let gad = unwrap_u64(p.governance_action_deposit.as_ref())?;
    put_u64_str(&mut o, "governanceActionDeposit", gad);
    let drd = unwrap_u64(p.drep_deposit.as_ref())?;
    put_u64_str(&mut o, "drepDeposit", drd);
    put_u64_str(&mut o, "drepInactivityPeriod", p.drep_inactivity_period);

    Ok(Value::Object(o))
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

    fn big_int(n: i64) -> v18::BigInt {
        v18::BigInt {
            big_int: Some(v18::big_int::BigInt::Int(n)),
        }
    }

    #[test]
    fn convert_pparams_decodes_under_017_pbjson() {
        // Load-bearing for PParams ABI compat: the JSON produced by
        // `pparams_to_legacy_json` must deserialize cleanly under
        // utxorpc-spec 0.17's pbjson decoder, which is what pre-BigInt
        // worker SDKs ship with.
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

        let json = pparams_to_legacy_json(&upstream).expect("convert");
        let bytes = serde_json::to_vec(&json).unwrap();
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
        let res = pparams_to_legacy_json(&upstream);
        assert!(matches!(res, Err(ConvertError::Overflow)));
    }

    #[test]
    fn convert_pparams_omits_default_fields() {
        // Mirrors pbjson's omit-default behavior — fields left at their
        // proto3 default must not appear in the JSON output, otherwise
        // old workers might trip on unexpected zero-encoded entries.
        let upstream = v18::PParams {
            coins_per_utxo_byte: Some(big_int(4310)),
            ..Default::default()
        };
        let json = pparams_to_legacy_json(&upstream).expect("convert");
        let obj = json.as_object().expect("object");
        assert_eq!(obj.len(), 1);
        assert!(obj.contains_key("coinsPerUtxoByte"));
    }
}
