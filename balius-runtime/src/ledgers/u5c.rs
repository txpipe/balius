use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use utxorpc::CardanoQueryClient;

use crate::wit::balius::app::ledger as wit;

impl From<utxorpc::Error> for crate::Error {
    fn from(error: utxorpc::Error) -> Self {
        crate::Error::Ledger(error.to_string())
    }
}

impl From<utxorpc::Error> for wit::LedgerError {
    fn from(error: utxorpc::Error) -> Self {
        wit::LedgerError::Upstream(error.to_string())
    }
}

impl From<wit::TxoRef> for utxorpc::spec::query::TxoRef {
    fn from(value: wit::TxoRef) -> Self {
        utxorpc::spec::query::TxoRef {
            hash: value.tx_hash.into(),
            index: value.tx_index,
        }
    }
}

impl From<utxorpc::spec::query::TxoRef> for wit::TxoRef {
    fn from(value: utxorpc::spec::query::TxoRef) -> Self {
        wit::TxoRef {
            tx_hash: value.hash.into(),
            tx_index: value.index,
        }
    }
}

fn chain_utxo_to_wit(
    value: utxorpc::ChainUtxo<utxorpc::spec::cardano::TxOutput>,
) -> Result<wit::Utxo, wit::LedgerError> {
    use prost::Message;

    let bytes = value
        .parsed
        .map(convert_tx_output)
        .transpose()
        .map_err(|e| wit::LedgerError::Upstream(format!("u5c -> balius_proto conversion: {e}")))?
        .map(|t| t.encode_to_vec())
        .unwrap_or_default();

    Ok(wit::Utxo {
        body: bytes,
        ref_: value.txo_ref.unwrap_or_default().into(),
    })
}

impl From<wit::AddressPattern> for utxorpc::spec::cardano::AddressPattern {
    fn from(value: wit::AddressPattern) -> Self {
        utxorpc::spec::cardano::AddressPattern {
            exact_address: value.exact_address.into(),
            ..Default::default()
        }
    }
}

impl From<wit::AssetPattern> for utxorpc::spec::cardano::AssetPattern {
    fn from(value: wit::AssetPattern) -> Self {
        utxorpc::spec::cardano::AssetPattern {
            policy_id: value.policy.into(),
            asset_name: value.name.map(|n| n.into()).unwrap_or_default(),
        }
    }
}

impl From<wit::UtxoPattern> for utxorpc::spec::cardano::TxOutputPattern {
    fn from(value: wit::UtxoPattern) -> Self {
        utxorpc::spec::cardano::TxOutputPattern {
            address: value.address.map(|a| a.into()),
            asset: value.asset.map(|a| a.into()),
        }
    }
}

fn utxo_page_to_wit(
    value: utxorpc::UtxoPage<utxorpc::Cardano>,
) -> Result<wit::UtxoPage, wit::LedgerError> {
    Ok(wit::UtxoPage {
        utxos: value
            .items
            .into_iter()
            .map(chain_utxo_to_wit)
            .collect::<Result<Vec<_>, _>>()?,
        next_token: value.next,
    })
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    pub endpoint_url: String,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Clone)]
pub struct Ledger {
    queries: utxorpc::CardanoQueryClient,
}

impl Ledger {
    pub async fn new(config: &Config) -> Result<Self, crate::Error> {
        let mut builder = utxorpc::ClientBuilder::new().uri(&config.endpoint_url)?;
        if let Some(headers) = &config.headers {
            for (k, v) in headers.iter() {
                builder = builder.metadata(k, v)?;
            }
        }

        let queries = builder.build::<CardanoQueryClient>().await;

        Ok(Self { queries })
    }

    pub async fn read_utxos(
        &mut self,
        refs: Vec<wit::TxoRef>,
    ) -> Result<Vec<wit::Utxo>, wit::LedgerError> {
        let refs = refs.into_iter().map(|r| r.into()).collect();
        let utxos = self.queries.read_utxos(refs).await?;
        utxos.into_iter().map(chain_utxo_to_wit).collect()
    }

    pub async fn search_utxos(
        &mut self,
        pattern: wit::UtxoPattern,
        start: Option<String>,
        max_items: u32,
    ) -> Result<wit::UtxoPage, wit::LedgerError> {
        let pattern = pattern.into();
        let utxos = self.queries.match_utxos(pattern, start, max_items).await?;
        utxo_page_to_wit(utxos)
    }

    pub async fn read_params(&mut self) -> Result<wit::Json, wit::LedgerError> {
        let res = self
            .queries
            .read_params()
            .await
            .map_err(|err| wit::LedgerError::Upstream(format!("{err:?}")))?;

        let params = res.params.ok_or(wit::LedgerError::Upstream(
            "unexpected response from read_params".to_string(),
        ))?;
        match params {
            utxorpc::spec::query::any_chain_params::Params::Cardano(params) => {
                let json = pparams_to_legacy_json(&params).map_err(|e| {
                    wit::LedgerError::Upstream(format!("u5c -> balius pparams conversion: {e}"))
                })?;
                Ok(serde_json::to_vec(&json).unwrap())
            }
            #[allow(unreachable_patterns)]
            _ => Err(wit::LedgerError::Upstream(
                "unexpected response from read_params".to_string(),
            )),
        }
    }
}

// ────────────────────────────────────────────────────────────────────────
// u5c -> balius_proto conversion
//
// Used at two places: chainsync ingress (lib.rs::Block::txs) and ledger
// reads (chain_utxo_to_wit / read_params above). Lives here, not in
// balius-proto, so the schema crate stays decoupled from any upstream
// service.
// ────────────────────────────────────────────────────────────────────────

use balius_proto::cardano as legacy;
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
