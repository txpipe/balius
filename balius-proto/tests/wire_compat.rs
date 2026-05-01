//! Wire-compat + conversion tests for balius_proto.
//!
//! Bytes produced from `balius_proto::cardano::*` must decode 1:1 under
//! `utxorpc-spec 0.17.0`. Tags absent from balius_proto are simply not
//! emitted; old SDKs see the missing fields as default-valued, which is
//! the deliberate trade-off of the trimmed schema.
//!
//! Run convert-feature tests with:
//!   cargo test -p balius-proto --features convert

use prost::Message;

use balius_proto::cardano as legacy;
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
    };

    let bytes = bal.encode_to_vec();
    let decoded = v17::TxOutput::decode(bytes.as_slice()).expect("decode 0.17.0");

    assert_eq!(decoded.address.to_vec(), vec![0xAA; 28]);
    assert_eq!(decoded.coin, 1_000_000);
    assert_eq!(decoded.assets.len(), 1);
    assert_eq!(decoded.assets[0].assets[0].output_coin, 5);
    assert_eq!(decoded.assets[0].assets[0].name.to_vec(), b"hello".to_vec());
    // Dropped tags appear as defaults on the decoder side:
    assert!(decoded.datum.is_none());
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
        fee: 1234,
        hash: vec![0xCC; 32].into(),
    };
    let bytes = bal.encode_to_vec();
    let decoded = v17::Tx::decode(bytes.as_slice()).expect("decode 0.17.0");
    assert_eq!(decoded.inputs.len(), 1);
    assert_eq!(decoded.inputs[0].output_index, 1);
    assert_eq!(decoded.fee, 1234);
    assert_eq!(decoded.hash.to_vec(), vec![0xCC; 32]);
    // Dropped tags decode as defaults:
    assert!(decoded.certificates.is_empty());
    assert!(decoded.withdrawals.is_empty());
    assert!(decoded.proposals.is_empty());
    assert!(!decoded.successful);
}

#[cfg(feature = "convert")]
mod convert {
    use super::*;
    use utxorpc_spec::utxorpc::v1alpha::cardano as v18;

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

        let bal: legacy::TxOutput = upstream.try_into().expect("convert");
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
        let res: Result<legacy::TxOutput, _> = upstream.try_into();
        assert!(matches!(
            res,
            Err(balius_proto::convert::ConvertError::Overflow)
        ));
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
        let res: Result<legacy::TxOutput, _> = upstream.try_into();
        assert!(matches!(
            res,
            Err(balius_proto::convert::ConvertError::Overflow)
        ));
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
        let bal: legacy::Tx = upstream.try_into().expect("convert");
        assert_eq!(bal.fee, 9999);
        assert_eq!(bal.hash.to_vec(), vec![0xCC; 32]);
    }

    #[test]
    fn convert_then_decode_with_017() {
        // Most load-bearing: a worker built against utxorpc-spec 0.17 must
        // be able to decode the bytes the runtime path produces.
        let upstream = v18::TxOutput {
            address: vec![0xAA; 28].into(),
            coin: Some(v18::BigInt {
                big_int: Some(v18::big_int::BigInt::Int(2_500_000)),
            }),
            assets: vec![],
            datum: None,
            script: None,
        };
        let bal: legacy::TxOutput = upstream.try_into().expect("convert");
        let bytes = bal.encode_to_vec();
        let decoded_017 = v17::TxOutput::decode(bytes.as_slice()).expect("0.17.0 decode");
        assert_eq!(decoded_017.coin, 2_500_000);
        assert_eq!(decoded_017.address.to_vec(), vec![0xAA; 28]);
    }
}
