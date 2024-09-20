use balius_ddk::prelude::*;
use serde::Serialize;

pub type Bytes = Vec<u8>;

#[derive(Serialize, Clone, PartialEq)]
pub struct AssetClass {
    pub policy: Bytes,
    pub name: Bytes,
}

#[derive(Serialize, Clone)]
pub enum Credential {
    VKeyCredential(Bytes),
    SCredential(Bytes),
}

impl Into<Cbor> for &Credential {
    fn into(self) -> crate::submit::Cbor {
        todo!()
    }
}

#[derive(Serialize)]
pub struct MultiSigScriptSchema {
    pub signature: Bytes,
}

#[derive(Serialize)]
pub struct SingletonValue {
    pub policy: Bytes,
    pub name: Bytes,
    pub amount: u64,
}

#[derive(Serialize)]
pub enum Extension {
    NoExtension,
    Foo,
}

#[derive(Serialize, Clone)]
pub struct Address {
    pub payment_credential: Credential,
    pub stake_credential: Option<Credential>,
}

#[derive(Serialize)]
pub struct Destination {
    pub address: Credential,
    pub datum: Option<Credential>,
}

#[derive(Serialize)]
pub struct PoolDatum {
    pub identifier: Bytes,
    pub assets: (AssetClass, AssetClass),
    pub circulating_lp: u64,
    pub fees_per10_thousand: (u64, u64),
    pub market_open: u64,
    pub fee_finalized: u64,
    pub protocol_fees: u64,
}

#[derive(Serialize)]
pub struct SwapOrderSpec {
    pub offer: SingletonValue,
    pub min_received: SingletonValue,
}

#[derive(Serialize)]
pub struct OrderDatum {
    pub pool_ident: Bytes,
    pub owner: MultiSigScriptSchema,
    pub scooper_fee: u64,
    pub destination: Destination,
    pub order: SwapOrderSpec,
    pub extension: Extension,
}

pub fn compute_abl_price(pool: &Utxo<PoolDatum>, asset: &AssetClass, gives: u64) -> f32 {
    todo!()
}

pub fn pool_has_assets(asset_a: &AssetClass, asset_b: &AssetClass) -> bool {
    todo!()
}

pub fn compute_received_with_slippage(current_price: f32, gives: u64, max_slippage: f32) -> u64 {
    //current_price * gives as f32 * (1f32 - max_slippage)
    todo!()
}

pub struct SwapOrderTx {
    pub pool: Utxo<PoolDatum>,
    pub desination: Address,
    pub order: SwapOrderSpec,
}

impl SwapOrderTx {
    pub fn finalize(self) {
        todo!()
    }
}
