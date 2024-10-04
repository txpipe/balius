use pallas_primitives::babbage;
use pallas_traverse::MultiEraOutput;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::collections::{HashMap, HashSet};

use super::*;

pub type Hash<const N: usize> = pallas_crypto::hash::Hash<N>;
pub type Address = pallas_addresses::Address;
pub type Value = pallas_primitives::babbage::Value;
pub type Bytes = pallas_codec::utils::Bytes;
pub type KeyValuePairs<K, V> = pallas_codec::utils::KeyValuePairs<K, V>;

pub struct PParams {
    pub min_fee_a: u64,
    pub min_fee_b: u64,
    pub min_utxo_value: u64,
}

pub trait Ledger {
    fn read_utxos(&self, refs: &[TxoRef]) -> Result<UtxoSet, BuildError>;
    fn search_utxos(&self, pattern: &UtxoPattern) -> Result<UtxoSet, BuildError>;
}

pub type Cbor = Vec<u8>;

#[derive(Debug, Clone, Default)]
pub struct UtxoSet(HashMap<TxoRef, Cbor>);

impl UtxoSet {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn refs(&self) -> impl Iterator<Item = &TxoRef> {
        self.0.keys()
    }

    pub fn txos(&self) -> impl Iterator<Item = MultiEraOutput<'_>> {
        self.0
            .values()
            .map(|v| MultiEraOutput::decode(pallas_traverse::Era::Babbage, v).unwrap())
    }
}

impl From<HashMap<TxoRef, Cbor>> for UtxoSet {
    fn from(value: HashMap<TxoRef, Cbor>) -> Self {
        UtxoSet(value)
    }
}

impl Ledger for UtxoSet {
    fn read_utxos(&self, refs: &[TxoRef]) -> Result<UtxoSet, BuildError> {
        let out: HashMap<_, _> = refs
            .iter()
            .filter_map(|r| self.0.get(r).map(|v| (r.clone(), v.clone())))
            .collect();

        Ok(UtxoSet(out))
    }

    fn search_utxos(&self, _pattern: &UtxoPattern) -> Result<UtxoSet, BuildError> {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UtxoPattern;

pub struct BuildContext {
    pub network: babbage::NetworkId,
    pub pparams: PParams,
    pub estimated_fee: u64,
    pub ledger: Box<dyn Ledger>,

    pub tx_body: Option<babbage::TransactionBody>,
}

impl BuildContext {
    pub fn mint_redeemer_index(&self, policy: Hash<28>) -> Result<u32, BuildError> {
        if let Some(tx_body) = &self.tx_body {
            let mut out: Vec<_> = tx_body
                .mint
                .iter()
                .flat_map(|x| x.iter())
                .map(|(p, _)| *p)
                .collect();

            out.sort();
            out.dedup();

            if let Some(index) = out.iter().position(|p| *p == policy) {
                return Ok(index as u32);
            }
        }

        Err(BuildError::RedeemerTargetMissing)
    }

    pub fn eval_ex_units(
        &self,
        _script: Hash<28>,
        _data: &babbage::PlutusData,
    ) -> babbage::ExUnits {
        // TODO
        babbage::ExUnits { mem: 8, steps: 8 }
    }
}

pub trait InputExpr: 'static + Send + Sync {
    fn eval(&self, ctx: &BuildContext) -> Result<Vec<babbage::TransactionInput>, BuildError>;
}

#[derive(Clone, Serialize, Deserialize)]
pub enum UtxoSource {
    Refs(Vec<TxoRef>),
    Search(UtxoPattern),
}

impl UtxoSource {
    pub fn resolve(&self, ctx: &BuildContext) -> Result<UtxoSet, BuildError> {
        match self {
            Self::Refs(refs) => ctx.ledger.read_utxos(refs),
            Self::Search(utxo_pattern) => ctx.ledger.search_utxos(utxo_pattern),
        }
    }
}

#[serde_as]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct ReferenceScript {
    pub ref_txo: babbage::TransactionInput,
    pub hash: Hash<28>,
    #[serde_as(as = "DisplayFromStr")]
    pub address: Address,
}

impl InputExpr for ReferenceScript {
    fn eval(&self, _: &dsl::BuildContext) -> Result<Vec<babbage::TransactionInput>, BuildError> {
        Ok(vec![self.ref_txo.clone()])
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Hash, Serialize, Deserialize)]
pub struct AssetPolicyId(Hash<28>);

impl AssetPolicyId {
    pub fn new(hash: Hash<28>) -> Self {
        Self(hash)
    }
}

impl From<Hash<28>> for AssetPolicyId {
    fn from(value: Hash<28>) -> Self {
        Self(value)
    }
}

impl Into<Hash<28>> for AssetPolicyId {
    fn into(self) -> Hash<28> {
        self.0
    }
}

impl TryFrom<&str> for AssetPolicyId {
    type Error = BuildError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let hash = <Hash<28> as std::str::FromStr>::from_str(value)
            .map_err(|_| BuildError::MalformedAssetPolicyIdHex)?;
        Ok(AssetPolicyId(hash))
    }
}

impl std::ops::Deref for AssetPolicyId {
    type Target = Hash<28>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for AssetPolicyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Hash, Serialize, Deserialize)]
pub struct AssetName(Bytes);

impl AssetName {
    pub fn new(name: Bytes) -> Result<Self, BuildError> {
        if name.len() > 32 {
            panic!("Asset name too long");
        }

        Ok(Self(name))
    }
}

impl TryFrom<Vec<u8>> for AssetName {
    type Error = BuildError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Self::new(value.into())
    }
}

impl TryFrom<&str> for AssetName {
    type Error = BuildError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.as_bytes().to_vec().into())
    }
}

impl From<AssetName> for Bytes {
    fn from(value: AssetName) -> Self {
        value.0
    }
}

impl std::ops::Deref for AssetName {
    type Target = Bytes;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// TODO: Don't want our wrapper types in fields public
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Hash, Clone)]
pub struct TxoRef {
    pub hash: Hash<32>,
    pub index: u64,
}

impl std::str::FromStr for TxoRef {
    type Err = BuildError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hash, index) = s.split_once("#").ok_or(BuildError::MalformedTxoRef)?;
        let hash = Hash::from_str(hash).map_err(|_| BuildError::MalformedTxoRef)?;
        let index = index.parse().map_err(|_| BuildError::MalformedTxoRef)?;
        Ok(TxoRef::new(hash, index))
    }
}

impl TxoRef {
    pub fn new(hash: Hash<32>, index: u64) -> Self {
        Self { hash, index }
    }
}

impl dsl::InputExpr for TxoRef {
    fn eval(&self, _: &BuildContext) -> Result<Vec<babbage::TransactionInput>, BuildError> {
        Ok(vec![self.into()])
    }
}

impl Into<babbage::TransactionInput> for &TxoRef {
    fn into(self) -> babbage::TransactionInput {
        babbage::TransactionInput {
            transaction_id: self.hash.into(),
            index: self.index,
        }
    }
}

impl InputExpr for UtxoSource {
    fn eval(&self, ctx: &BuildContext) -> Result<Vec<babbage::TransactionInput>, BuildError> {
        let out = self.resolve(ctx)?.refs().map(|i| i.into()).collect();

        Ok(out)
    }
}

pub trait ValueExpr: 'static + Send + Sync {
    fn eval(&self, ctx: &BuildContext) -> Result<babbage::Value, BuildError>;

    fn eval_as_mint(&self, ctx: &BuildContext) -> Result<babbage::Mint, BuildError> {
        let value = self.eval(ctx)?;

        match value {
            babbage::Value::Multiasset(_, assets) => {
                Ok(asset_math::multiasset_coin_to_mint(assets))
            }
            babbage::Value::Coin(_) => Err(BuildError::Conflicting),
        }
    }

    fn eval_as_burn(&self, ctx: &BuildContext) -> Result<babbage::Mint, BuildError> {
        let value = self.eval(ctx)?;

        match value {
            babbage::Value::Multiasset(_, assets) => {
                Ok(asset_math::multiasset_coin_to_burn(assets))
            }
            babbage::Value::Coin(_) => Err(BuildError::Conflicting),
        }
    }
}

impl ValueExpr for u64 {
    fn eval(&self, _ctx: &BuildContext) -> Result<babbage::Value, BuildError> {
        Ok(babbage::Value::Coin(*self))
    }
}

impl<F> ValueExpr for F
where
    F: Fn(&BuildContext) -> Result<babbage::Value, BuildError> + 'static + Send + Sync,
{
    fn eval(&self, ctx: &BuildContext) -> Result<babbage::Value, BuildError> {
        self(ctx)
    }
}

impl<T: ValueExpr> ValueExpr for Option<T> {
    fn eval(&self, ctx: &BuildContext) -> Result<babbage::Value, BuildError> {
        match self {
            Some(v) => v.eval(ctx),
            None => Err(BuildError::Incomplete),
        }
    }
}

pub struct MinUtxoLovelace;

impl ValueExpr for MinUtxoLovelace {
    fn eval(&self, ctx: &BuildContext) -> Result<babbage::Value, BuildError> {
        Ok(babbage::Value::Coin(ctx.pparams.min_utxo_value))
    }
}

impl ValueExpr for Box<dyn ValueExpr> {
    fn eval(&self, ctx: &BuildContext) -> Result<babbage::Value, BuildError> {
        (**self).eval(ctx)
    }
}

impl<T: ValueExpr> ValueExpr for Vec<T> {
    fn eval(&self, ctx: &BuildContext) -> Result<babbage::Value, BuildError> {
        let values = self
            .iter()
            .map(|v| v.eval(ctx))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(asset_math::aggregate_values(values))
    }
}

pub trait AddressExpr: 'static + Send + Sync {
    fn eval(&self, ctx: &BuildContext) -> Result<Address, BuildError>;
}

impl AddressExpr for Address {
    fn eval(&self, _ctx: &BuildContext) -> Result<Address, BuildError> {
        Ok(self.clone())
    }
}

impl AddressExpr for Box<dyn AddressExpr> {
    fn eval(&self, ctx: &BuildContext) -> Result<Address, BuildError> {
        (**self).eval(ctx)
    }
}

impl<T: AddressExpr> AddressExpr for Option<T> {
    fn eval(&self, ctx: &BuildContext) -> Result<Address, BuildError> {
        match self {
            Some(v) => v.eval(ctx),
            None => Err(BuildError::Incomplete),
        }
    }
}

impl<F> AddressExpr for F
where
    F: Fn(&BuildContext) -> Result<Address, BuildError> + 'static + Send + Sync,
{
    fn eval(&self, ctx: &BuildContext) -> Result<Address, BuildError> {
        self(ctx)
    }
}

pub trait OutputExpr: 'static + Send + Sync {
    fn eval(&self, ctx: &BuildContext) -> Result<babbage::TransactionOutput, BuildError>;
}

pub struct ChangeAddress(pub UtxoSource);

impl AddressExpr for ChangeAddress {
    fn eval(&self, ctx: &BuildContext) -> Result<Address, BuildError> {
        let utxo_set = &self.0.resolve(ctx)?;

        if utxo_set.is_empty() {
            return Err(BuildError::EmptyUtxoSet);
        }

        let addresses: HashSet<_> = utxo_set
            .txos()
            .map(|x| x.address())
            .collect::<Result<HashSet<_>, _>>()
            .map_err(|_| BuildError::UtxoDecode)?;

        if addresses.len() > 1 {
            return Err(BuildError::Conflicting);
        }

        Ok(addresses.into_iter().next().unwrap())
    }
}

pub struct TotalLovelaceMinusFee(pub UtxoSource);

impl ValueExpr for TotalLovelaceMinusFee {
    fn eval(&self, ctx: &BuildContext) -> Result<babbage::Value, BuildError> {
        let utxo_set = &self.0.resolve(ctx)?;
        let values = utxo_set.txos().map(|o| o.value().into_alonzo());
        let total = asset_math::aggregate_values(values);

        let fee = ctx.estimated_fee;
        let diff = asset_math::value_saturating_add_coin(total, -(fee as i64));

        Ok(diff)
    }
}

pub struct FeeChangeReturn(pub UtxoSource);

impl OutputExpr for FeeChangeReturn {
    fn eval(&self, ctx: &BuildContext) -> Result<babbage::TransactionOutput, BuildError> {
        OutputBuilder::new()
            .address(ChangeAddress(self.0.clone()))
            .with_value(TotalLovelaceMinusFee(self.0.clone()))
            .eval(ctx)
    }
}

pub trait PlutusDataExpr: 'static + Send + Sync {
    fn eval(&self, ctx: &BuildContext) -> Result<babbage::PlutusData, BuildError>;
}

impl PlutusDataExpr for babbage::PlutusData {
    fn eval(&self, _ctx: &BuildContext) -> Result<babbage::PlutusData, BuildError> {
        Ok(self.clone())
    }
}

impl<F> PlutusDataExpr for F
where
    F: Fn(&BuildContext) -> Result<babbage::PlutusData, BuildError> + 'static + Send + Sync,
{
    fn eval(&self, ctx: &BuildContext) -> Result<babbage::PlutusData, BuildError> {
        self(ctx)
    }
}

impl PlutusDataExpr for Box<dyn PlutusDataExpr> {
    fn eval(&self, ctx: &BuildContext) -> Result<babbage::PlutusData, BuildError> {
        (**self).eval(ctx)
    }
}

impl PlutusDataExpr for () {
    fn eval(&self, _ctx: &BuildContext) -> Result<babbage::PlutusData, BuildError> {
        Ok(babbage::PlutusData::Constr(babbage::Constr {
            tag: 121,
            any_constructor: None,
            fields: vec![],
        }))
    }
}

pub trait MintExpr: 'static + Send + Sync {
    fn eval(&self, ctx: &BuildContext) -> Result<babbage::Mint, BuildError>;
    fn eval_redeemer(&self, ctx: &BuildContext) -> Result<babbage::Redeemer, BuildError>;
}

#[derive(Default)]
pub struct MintBuilder {
    pub assets: Vec<Box<dyn ValueExpr>>,
    pub burn: Vec<Box<dyn ValueExpr>>,
    pub redeemer: Option<Box<dyn PlutusDataExpr>>,
}

impl MintBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_asset(mut self, asset: impl ValueExpr) -> Self {
        self.assets.push(Box::new(asset));
        self
    }

    pub fn with_burn(mut self, burn: impl ValueExpr) -> Self {
        self.burn.push(Box::new(burn));
        self
    }

    pub fn using_redeemer(mut self, redeemer: impl PlutusDataExpr) -> Self {
        self.redeemer = Some(Box::new(redeemer));
        self
    }
}

impl MintExpr for MintBuilder {
    fn eval(&self, ctx: &BuildContext) -> Result<babbage::Mint, BuildError> {
        let out = HashMap::new();

        let out = self.assets.iter().try_fold(out, |mut acc, v| {
            let v = v.eval_as_mint(ctx)?;
            asset_math::fold_multiassets(&mut acc, v);
            Ok(acc)
        })?;

        let out = self.burn.iter().try_fold(out, |mut acc, v| {
            let v = v.eval_as_burn(ctx)?;
            asset_math::fold_multiassets(&mut acc, v);
            Ok(acc)
        })?;

        let mint: HashMap<_, _> = out
            .into_iter()
            .map(|(policy, assets)| (policy, KeyValuePairs::from(assets)))
            .collect();

        Ok(KeyValuePairs::from(mint))
    }

    fn eval_redeemer(&self, ctx: &BuildContext) -> Result<babbage::Redeemer, BuildError> {
        let mint = self.eval(ctx)?;

        if mint.is_empty() {
            return Err(BuildError::Incomplete);
        }

        if mint.len() > 1 {
            return Err(BuildError::Conflicting);
        }

        let (policy, _) = mint.iter().next().unwrap();

        let data = self
            .redeemer
            .as_ref()
            .ok_or(BuildError::Incomplete)?
            .eval(ctx)?;

        let out = babbage::Redeemer {
            tag: babbage::RedeemerTag::Mint,
            index: ctx.mint_redeemer_index(*policy)?,
            ex_units: ctx.eval_ex_units(*policy, &data),
            data,
        };

        Ok(out)
    }
}

#[derive(Default)]
pub struct OutputBuilder {
    pub address: Option<Box<dyn dsl::AddressExpr>>,
    pub values: Vec<Box<dyn dsl::ValueExpr>>,
    // TODO: inline / hash datum
    // TODO: script
}

impl OutputBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn address(mut self, address: impl AddressExpr + 'static) -> Self {
        self.address = Some(Box::new(address));
        self
    }

    pub fn with_value(mut self, value: impl ValueExpr + 'static) -> Self {
        self.values.push(Box::new(value));
        self
    }
}

impl OutputExpr for OutputBuilder {
    fn eval(&self, ctx: &BuildContext) -> Result<babbage::TransactionOutput, BuildError> {
        Ok(babbage::TransactionOutput::PostAlonzo(
            babbage::PostAlonzoTransactionOutput {
                address: self.address.eval(ctx)?.to_vec().into(),
                value: self.values.eval(ctx)?,
                datum_option: None, // TODO
                script_ref: None,   // TODO
            },
        ))
    }
}

pub trait TxExpr: 'static + Send + Sync {
    fn eval_body(&self, ctx: &BuildContext) -> Result<babbage::TransactionBody, BuildError>;
    fn eval_witness_set(&self, ctx: &BuildContext) -> Result<babbage::WitnessSet, BuildError>;
}

impl<T: TxExpr> TxExpr for &'static T {
    fn eval_body(&self, ctx: &BuildContext) -> Result<babbage::TransactionBody, BuildError> {
        (**self).eval_body(ctx)
    }

    fn eval_witness_set(&self, ctx: &BuildContext) -> Result<babbage::WitnessSet, BuildError> {
        (**self).eval_witness_set(ctx)
    }
}

impl<T: TxExpr> TxExpr for Box<T> {
    fn eval_body(&self, ctx: &BuildContext) -> Result<babbage::TransactionBody, BuildError> {
        (**self).eval_body(ctx)
    }

    fn eval_witness_set(&self, ctx: &BuildContext) -> Result<babbage::WitnessSet, BuildError> {
        (**self).eval_witness_set(ctx)
    }
}

#[derive(Default)]
pub struct TxBuilder {
    pub reference_inputs: Vec<Box<dyn InputExpr>>,
    pub inputs: Vec<Box<dyn InputExpr>>,
    pub outputs: Vec<Box<dyn OutputExpr>>,
    // pub outputs: Option<Vec<Output>>,
    // pub fee: Option<u64>,
    pub mint: Vec<Box<dyn MintExpr>>,
    // pub valid_from_slot: Option<u64>,
    // pub invalid_from_slot: Option<u64>,
    // pub network_id: Option<u8>,
    // pub collateral_inputs: Option<Vec<TxoRef>>,
    // pub collateral_output: Option<Output>,
    // pub disclosed_signers: Option<Vec<PubKeyHash>>,
    // pub scripts: Option<HashMap<ScriptHash, Script>>,
    // pub datums: Option<HashMap<DatumHash, DatumBytes>>,
    // pub redeemers: Option<Redeemers>,
    // pub script_data_hash: Option<ScriptDataHash>,
    // pub signature_amount_override: Option<u8>,
    // #[serde_as(as = "Option<DisplayFromStr>")]
    // pub change_address: Option<Address>,
    // pub certificates: TODO
    // pub withdrawals: TODO
    // pub updates: TODO
    // pub auxiliary_data: TODO
    // pub phase_2_valid: TODO
}

impl TxBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_reference_input(mut self, input: impl InputExpr) -> Self {
        self.reference_inputs.push(Box::new(input));
        self
    }

    pub fn with_input(mut self, input: impl InputExpr) -> Self {
        self.inputs.push(Box::new(input));
        self
    }

    pub fn with_output(mut self, output: impl OutputExpr) -> Self {
        self.outputs.push(Box::new(output));
        self
    }

    pub fn with_mint(mut self, mint: impl MintExpr) -> Self {
        self.mint.push(Box::new(mint));
        self
    }
}

impl TxExpr for TxBuilder {
    fn eval_body(&self, ctx: &BuildContext) -> Result<babbage::TransactionBody, BuildError> {
        let out = babbage::TransactionBody {
            inputs: self
                .inputs
                .iter()
                .map(|i| i.eval(ctx))
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .flatten()
                .collect(),
            outputs: self
                .outputs
                .iter()
                .map(|o| o.eval(ctx))
                .collect::<Result<Vec<_>, _>>()?,
            fee: ctx.estimated_fee,
            ttl: None,
            validity_interval_start: None,
            certificates: None,
            withdrawals: None,
            update: None,
            auxiliary_data_hash: None,
            mint: {
                let mints = self
                    .mint
                    .iter()
                    .map(|m| m.eval(ctx))
                    .collect::<Result<Vec<_>, _>>()?;

                if mints.is_empty() {
                    None
                } else {
                    Some(asset_math::aggregate_assets(mints))
                }
            },
            script_data_hash: None,
            collateral: None,
            required_signers: None,
            network_id: None,
            collateral_return: None,
            total_collateral: None,
            reference_inputs: {
                let refs: Vec<_> = self
                    .reference_inputs
                    .iter()
                    .map(|i| i.eval(ctx))
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
                    .flatten()
                    .collect();

                if refs.is_empty() {
                    None
                } else {
                    Some(refs)
                }
            },
        };

        Ok(out)
    }

    fn eval_witness_set(&self, ctx: &BuildContext) -> Result<babbage::WitnessSet, BuildError> {
        let out = babbage::WitnessSet {
            redeemer: {
                let redeemers = self
                    .mint
                    .iter()
                    .map(|m| m.eval_redeemer(ctx))
                    .collect::<Result<Vec<_>, _>>()?;

                if redeemers.is_empty() {
                    None
                } else {
                    Some(redeemers)
                }
            },
            vkeywitness: None,
            native_script: None,
            bootstrap_witness: None,
            plutus_v1_script: None,
            plutus_data: None,
            plutus_v2_script: None,
        };

        Ok(out)
    }
}

#[macro_export]
macro_rules! define_asset_class {
    ($struct_name:ident, $policy:expr) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
        pub struct $struct_name($crate::txbuilder::Bytes, i64);

        impl $struct_name {
            pub fn value(name: $crate::txbuilder::AssetName, quantity: u64) -> Self {
                Self(name.into(), quantity as i64)
            }
        }

        impl $crate::txbuilder::ValueExpr for $struct_name {
            fn eval(
                &self,
                _: &$crate::txbuilder::BuildContext,
            ) -> std::result::Result<$crate::txbuilder::Value, $crate::txbuilder::BuildError> {
                let policy = $crate::txbuilder::Hash::from(*$policy);
                let name = $crate::txbuilder::Bytes::from(self.0.clone());
                let asset = $crate::txbuilder::KeyValuePairs::from(vec![(name, self.1 as u64)]);
                let val = $crate::txbuilder::Value::Multiasset(
                    0,
                    $crate::txbuilder::KeyValuePairs::from(vec![(policy, asset)]),
                );

                Ok(val)
            }
        }
    };
}

define_asset_class!(MyAssetClass, b"abcabcababcabcababcabcababca");
