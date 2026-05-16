//! Wire-compat invariant for balius_core::proto::v0.
//!
//! Bytes produced from `balius_core::proto::v0::cardano::*` must decode
//! 1:1 under `utxorpc-spec 0.17.0`. Tags absent from the schema are
//! simply not emitted; old SDKs see the missing fields as default-valued,
//! which is the deliberate trade-off of the trimmed schema.
//!
//! Conversion from upstream u5c into this schema lives in the runtime's
//! u5c adapter (`balius-runtime/src/ledgers/u5c/convert.rs`) — balius-core
//! itself stays decoupled from any upstream service.

use prost::Message;

use balius_core::proto::v0::cardano as legacy;
use utxorpc_spec_017::utxorpc::v1alpha::cardano as v17;

#[test]
fn wire_compat_tx_output_roundtrips_via_017() {
    let bal = legacy::TxOutput {
        address: vec![0xAA; 28].into(),
        coin: 1_000_000,
        assets: vec![legacy::Multiasset {
            policy_id: vec![0xBB; 28].into(),
            assets: vec![legacy::Asset {
                name: b"hello".to_vec().into(),
                output_coin: 5,
            }],
        }],
        datum: Some(legacy::Datum {
            hash: vec![0xEE; 32].into(),
            payload: Some(legacy::PlutusData {
                plutus_data: Some(legacy::plutus_data::PlutusData::BoundedBytes(
                    b"datum-payload".to_vec().into(),
                )),
            }),
            original_cbor: vec![0xFF; 8].into(),
        }),
    };

    let bytes = bal.encode_to_vec();
    let decoded = v17::TxOutput::decode(bytes.as_slice()).expect("decode 0.17.0");

    assert_eq!(decoded.address.to_vec(), vec![0xAA; 28]);
    assert_eq!(decoded.coin, 1_000_000);
    assert_eq!(decoded.assets.len(), 1);
    assert_eq!(decoded.assets[0].assets[0].output_coin, 5);
    assert_eq!(decoded.assets[0].assets[0].name.to_vec(), b"hello".to_vec());
    let datum = decoded.datum.as_ref().expect("datum present");
    assert_eq!(datum.hash.to_vec(), vec![0xEE; 32]);
    assert_eq!(datum.original_cbor.to_vec(), vec![0xFF; 8]);
    // Dropped TxOutput tags decode as defaults:
    assert!(decoded.script.is_none());
}

#[test]
fn wire_compat_tx_roundtrips_via_017() {
    let bal = legacy::Tx {
        inputs: vec![legacy::TxInput {
            tx_hash: vec![0xDD; 32].into(),
            output_index: 1,
            as_output: None,
        }],
        outputs: vec![],
        witnesses: Some(legacy::WitnessSet {
            vkeywitness: vec![legacy::VKeyWitness {
                vkey: vec![0x11; 32].into(),
                signature: vec![0x22; 64].into(),
            }],
            script: vec![],
            plutus_datums: vec![],
        }),
        fee: 1234,
        hash: vec![0xCC; 32].into(),
    };
    let bytes = bal.encode_to_vec();
    let decoded = v17::Tx::decode(bytes.as_slice()).expect("decode 0.17.0");
    assert_eq!(decoded.inputs.len(), 1);
    assert_eq!(decoded.inputs[0].output_index, 1);
    assert_eq!(decoded.fee, 1234);
    assert_eq!(decoded.hash.to_vec(), vec![0xCC; 32]);
    let witnesses = decoded.witnesses.as_ref().expect("witnesses present");
    assert_eq!(witnesses.vkeywitness.len(), 1);
    assert_eq!(witnesses.vkeywitness[0].vkey.to_vec(), vec![0x11; 32]);
    assert_eq!(witnesses.vkeywitness[0].signature.to_vec(), vec![0x22; 64]);
    // Dropped tags decode as defaults:
    assert!(decoded.certificates.is_empty());
    assert!(decoded.withdrawals.is_empty());
    assert!(decoded.proposals.is_empty());
    assert!(!decoded.successful);
}

#[test]
fn wire_compat_pparams_roundtrips_via_017() {
    // PParams crosses WIT as JSON, not prost. Same invariant applies:
    // what balius-core emits must decode 1:1 under utxorpc-spec 0.17's
    // pbjson decoder — that's the decoder pre-BigInt workers ship with.
    let bal = legacy::PParams {
        coins_per_utxo_byte: 4310,
        max_tx_size: 16384,
        min_fee_coefficient: 44,
        min_fee_constant: 155_381,
        max_block_body_size: 90_112,
        max_block_header_size: 1100,
        stake_key_deposit: 2_000_000,
        pool_deposit: 500_000_000,
        pool_retirement_epoch_bound: 18,
        desired_number_of_pools: 500,
        pool_influence: Some(legacy::RationalNumber {
            numerator: 3,
            denominator: 10,
        }),
        monetary_expansion: Some(legacy::RationalNumber {
            numerator: 3,
            denominator: 1000,
        }),
        treasury_expansion: Some(legacy::RationalNumber {
            numerator: 1,
            denominator: 5,
        }),
        min_pool_cost: 170_000_000,
        protocol_version: Some(legacy::ProtocolVersion { major: 10, minor: 0 }),
        max_value_size: 5000,
        collateral_percentage: 150,
        max_collateral_inputs: 3,
        cost_models: Some(legacy::CostModels {
            plutus_v1: Some(legacy::CostModel {
                values: vec![100_788, 420, 1, 1],
            }),
            plutus_v2: None,
            plutus_v3: None,
        }),
        prices: Some(legacy::ExPrices {
            steps: Some(legacy::RationalNumber {
                numerator: 721,
                denominator: 10_000_000,
            }),
            memory: Some(legacy::RationalNumber {
                numerator: 577,
                denominator: 10_000,
            }),
        }),
        max_execution_units_per_transaction: Some(legacy::ExUnits {
            steps: 10_000_000_000,
            memory: 14_000_000,
        }),
        max_execution_units_per_block: Some(legacy::ExUnits {
            steps: 20_000_000_000,
            memory: 62_000_000,
        }),
        governance_action_deposit: 100_000_000_000,
        drep_deposit: 500_000_000,
        min_committee_size: 7,
        ..Default::default()
    };

    let json = serde_json::to_value(&bal).expect("serialize");
    let decoded: v17::PParams = serde_json::from_value(json).expect("decode 0.17.0 pbjson");

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
    let pi = decoded.pool_influence.as_ref().expect("pool_influence present");
    assert_eq!((pi.numerator, pi.denominator), (3, 10));
    assert_eq!(decoded.protocol_version.as_ref().unwrap().major, 10);
    let cm = decoded.cost_models.as_ref().expect("cost_models present");
    assert_eq!(
        cm.plutus_v1.as_ref().unwrap().values,
        vec![100_788, 420, 1, 1]
    );
    let prices = decoded.prices.as_ref().expect("prices present");
    assert_eq!(prices.steps.as_ref().unwrap().numerator, 721);
    assert_eq!(prices.memory.as_ref().unwrap().denominator, 10_000);
    let mex_tx = decoded
        .max_execution_units_per_transaction
        .as_ref()
        .unwrap();
    assert_eq!(mex_tx.steps, 10_000_000_000);
    assert_eq!(mex_tx.memory, 14_000_000);
}

#[test]
fn wire_compat_pparams_omits_default_fields() {
    // Mirrors pbjson omit-default — pre-BigInt workers shouldn't see
    // spurious zero-valued entries for fields the runtime didn't set.
    let bal = legacy::PParams {
        coins_per_utxo_byte: 4310,
        ..Default::default()
    };
    let json = serde_json::to_value(&bal).expect("serialize");
    let obj = json.as_object().expect("object");
    assert_eq!(obj.len(), 1);
    assert!(obj.contains_key("coinsPerUtxoByte"));
}
