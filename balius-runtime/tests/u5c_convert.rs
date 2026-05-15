//! u5c -> balius_proto conversion tests for the runtime's u5c adapter.
//!
//! Moved out of `balius-proto/tests/wire_compat.rs` so that balius-proto
//! stays a pure schema crate. The load-bearing invariant tested here is
//! that bytes produced by the conversion functions decode unchanged
//! under utxorpc-spec 0.17 — i.e. workers built against pre-BigInt SDKs
//! keep working.

use prost::Message;

use balius_proto::cardano as legacy;
use balius_runtime::ledgers::u5c::{
    convert_tx, convert_tx_output, pparams_to_legacy_json, ConvertError,
};
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
