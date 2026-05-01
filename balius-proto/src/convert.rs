//! Conversions from upstream `utxorpc-spec` 0.18.x types into the
//! Balius-owned schema (which mirrors the pre-BigInt 0.17.0 wire format).
//!
//! The runtime ingests u5c bytes from the chainsync / ledger streams and
//! must hand the WASM guest bytes that decode under the legacy schema.
//! These conversions are the single, hardcoded translation layer.

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
            ConvertError::Overflow => write!(f, "BigInt value out of range for legacy uint64/int64 target"),
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

fn unwrap_i64(b: Option<&upstream::BigInt>) -> Result<i64, ConvertError> {
    match b.and_then(|x| x.big_int.as_ref()) {
        None => Ok(0),
        Some(upstream::big_int::BigInt::Int(v)) => Ok(*v),
        _ => Err(ConvertError::Overflow),
    }
}

/// Re-encodes a wire-stable type via prost roundtrip into its legacy
/// counterpart. Only safe when the upstream and legacy types are
/// known to share an identical wire format.
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

fn try_map<U, L, F>(items: Vec<U>, f: F) -> Result<Vec<L>, ConvertError>
where
    F: Fn(U) -> Result<L, ConvertError>,
{
    items.into_iter().map(f).collect()
}

// ---------------------------------------------------------------------------
// Asset / Multiasset
// ---------------------------------------------------------------------------

impl TryFrom<upstream::Asset> for legacy::Asset {
    type Error = ConvertError;

    fn try_from(a: upstream::Asset) -> Result<Self, Self::Error> {
        let mut output_coin = 0u64;
        let mut mint_coin = 0i64;
        match a.quantity {
            None => {}
            Some(upstream::asset::Quantity::OutputCoin(b)) => {
                output_coin = unwrap_u64(Some(&b))?;
            }
            Some(upstream::asset::Quantity::MintCoin(b)) => {
                mint_coin = unwrap_i64(Some(&b))?;
            }
        }
        Ok(legacy::Asset {
            name: a.name,
            output_coin,
            mint_coin,
        })
    }
}

impl TryFrom<upstream::Multiasset> for legacy::Multiasset {
    type Error = ConvertError;

    fn try_from(m: upstream::Multiasset) -> Result<Self, Self::Error> {
        Ok(legacy::Multiasset {
            policy_id: m.policy_id,
            assets: try_map(m.assets, legacy::Asset::try_from)?,
            redeemer: roundtrip_opt(&m.redeemer)?,
        })
    }
}

// ---------------------------------------------------------------------------
// TxOutput / TxInput
// ---------------------------------------------------------------------------

impl TryFrom<upstream::TxOutput> for legacy::TxOutput {
    type Error = ConvertError;

    fn try_from(o: upstream::TxOutput) -> Result<Self, Self::Error> {
        Ok(legacy::TxOutput {
            address: o.address,
            coin: unwrap_u64(o.coin.as_ref())?,
            assets: try_map(o.assets, legacy::Multiasset::try_from)?,
            datum: roundtrip_opt(&o.datum)?,
            script: roundtrip_opt(&o.script)?,
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
            redeemer: roundtrip_opt(&i.redeemer)?,
        })
    }
}

// ---------------------------------------------------------------------------
// Tx-level pieces
// ---------------------------------------------------------------------------

impl TryFrom<upstream::Withdrawal> for legacy::Withdrawal {
    type Error = ConvertError;

    fn try_from(w: upstream::Withdrawal) -> Result<Self, Self::Error> {
        Ok(legacy::Withdrawal {
            reward_account: w.reward_account,
            coin: unwrap_u64(w.coin.as_ref())?,
            redeemer: roundtrip_opt(&w.redeemer)?,
        })
    }
}

impl TryFrom<upstream::Collateral> for legacy::Collateral {
    type Error = ConvertError;

    fn try_from(c: upstream::Collateral) -> Result<Self, Self::Error> {
        Ok(legacy::Collateral {
            collateral: try_map(c.collateral, legacy::TxInput::try_from)?,
            collateral_return: c
                .collateral_return
                .map(legacy::TxOutput::try_from)
                .transpose()?,
            total_collateral: unwrap_u64(c.total_collateral.as_ref())?,
        })
    }
}

// ---------------------------------------------------------------------------
// Certificates (the BigInt-bearing variants)
// ---------------------------------------------------------------------------

impl TryFrom<upstream::PoolRegistrationCert> for legacy::PoolRegistrationCert {
    type Error = ConvertError;

    fn try_from(c: upstream::PoolRegistrationCert) -> Result<Self, Self::Error> {
        Ok(legacy::PoolRegistrationCert {
            operator: c.operator,
            vrf_keyhash: c.vrf_keyhash,
            pledge: unwrap_u64(c.pledge.as_ref())?,
            cost: unwrap_u64(c.cost.as_ref())?,
            margin: roundtrip_opt(&c.margin)?,
            reward_account: c.reward_account,
            pool_owners: c.pool_owners,
            relays: c
                .relays
                .iter()
                .map(|r| roundtrip::<_, legacy::Relay>(r))
                .collect::<Result<Vec<_>, _>>()?,
            pool_metadata: roundtrip_opt(&c.pool_metadata)?,
        })
    }
}

impl TryFrom<upstream::MirTarget> for legacy::MirTarget {
    type Error = ConvertError;

    fn try_from(t: upstream::MirTarget) -> Result<Self, Self::Error> {
        Ok(legacy::MirTarget {
            stake_credential: roundtrip_opt(&t.stake_credential)?,
            delta_coin: unwrap_i64(t.delta_coin.as_ref())?,
        })
    }
}

impl TryFrom<upstream::MirCert> for legacy::MirCert {
    type Error = ConvertError;

    fn try_from(c: upstream::MirCert) -> Result<Self, Self::Error> {
        Ok(legacy::MirCert {
            from: c.from,
            to: try_map(c.to, legacy::MirTarget::try_from)?,
            other_pot: c.other_pot,
        })
    }
}

impl TryFrom<upstream::RegCert> for legacy::RegCert {
    type Error = ConvertError;

    fn try_from(c: upstream::RegCert) -> Result<Self, Self::Error> {
        Ok(legacy::RegCert {
            stake_credential: roundtrip_opt(&c.stake_credential)?,
            coin: unwrap_u64(c.coin.as_ref())?,
        })
    }
}

impl TryFrom<upstream::UnRegCert> for legacy::UnRegCert {
    type Error = ConvertError;

    fn try_from(c: upstream::UnRegCert) -> Result<Self, Self::Error> {
        Ok(legacy::UnRegCert {
            stake_credential: roundtrip_opt(&c.stake_credential)?,
            coin: unwrap_u64(c.coin.as_ref())?,
        })
    }
}

impl TryFrom<upstream::StakeRegDelegCert> for legacy::StakeRegDelegCert {
    type Error = ConvertError;

    fn try_from(c: upstream::StakeRegDelegCert) -> Result<Self, Self::Error> {
        Ok(legacy::StakeRegDelegCert {
            stake_credential: roundtrip_opt(&c.stake_credential)?,
            pool_keyhash: c.pool_keyhash,
            coin: unwrap_u64(c.coin.as_ref())?,
        })
    }
}

impl TryFrom<upstream::VoteRegDelegCert> for legacy::VoteRegDelegCert {
    type Error = ConvertError;

    fn try_from(c: upstream::VoteRegDelegCert) -> Result<Self, Self::Error> {
        Ok(legacy::VoteRegDelegCert {
            stake_credential: roundtrip_opt(&c.stake_credential)?,
            drep: roundtrip_opt(&c.drep)?,
            coin: unwrap_u64(c.coin.as_ref())?,
        })
    }
}

impl TryFrom<upstream::StakeVoteRegDelegCert> for legacy::StakeVoteRegDelegCert {
    type Error = ConvertError;

    fn try_from(c: upstream::StakeVoteRegDelegCert) -> Result<Self, Self::Error> {
        Ok(legacy::StakeVoteRegDelegCert {
            stake_credential: roundtrip_opt(&c.stake_credential)?,
            pool_keyhash: c.pool_keyhash,
            drep: roundtrip_opt(&c.drep)?,
            coin: unwrap_u64(c.coin.as_ref())?,
        })
    }
}

impl TryFrom<upstream::RegDRepCert> for legacy::RegDRepCert {
    type Error = ConvertError;

    fn try_from(c: upstream::RegDRepCert) -> Result<Self, Self::Error> {
        Ok(legacy::RegDRepCert {
            drep_credential: roundtrip_opt(&c.drep_credential)?,
            coin: unwrap_u64(c.coin.as_ref())?,
            anchor: roundtrip_opt(&c.anchor)?,
        })
    }
}

impl TryFrom<upstream::UnRegDRepCert> for legacy::UnRegDRepCert {
    type Error = ConvertError;

    fn try_from(c: upstream::UnRegDRepCert) -> Result<Self, Self::Error> {
        Ok(legacy::UnRegDRepCert {
            drep_credential: roundtrip_opt(&c.drep_credential)?,
            coin: unwrap_u64(c.coin.as_ref())?,
        })
    }
}

impl TryFrom<upstream::Certificate> for legacy::Certificate {
    type Error = ConvertError;

    fn try_from(c: upstream::Certificate) -> Result<Self, Self::Error> {
        use legacy::certificate::Certificate as L;
        use upstream::certificate::Certificate as U;
        let cert = match c.certificate {
            None => None,
            Some(U::StakeRegistration(s)) => Some(L::StakeRegistration(roundtrip(&s)?)),
            Some(U::StakeDeregistration(s)) => Some(L::StakeDeregistration(roundtrip(&s)?)),
            Some(U::StakeDelegation(s)) => Some(L::StakeDelegation(roundtrip(&s)?)),
            Some(U::PoolRegistration(p)) => Some(L::PoolRegistration(p.try_into()?)),
            Some(U::PoolRetirement(p)) => Some(L::PoolRetirement(roundtrip(&p)?)),
            Some(U::GenesisKeyDelegation(g)) => Some(L::GenesisKeyDelegation(roundtrip(&g)?)),
            Some(U::MirCert(m)) => Some(L::MirCert(m.try_into()?)),
            Some(U::RegCert(r)) => Some(L::RegCert(r.try_into()?)),
            Some(U::UnregCert(r)) => Some(L::UnregCert(r.try_into()?)),
            Some(U::VoteDelegCert(v)) => Some(L::VoteDelegCert(roundtrip(&v)?)),
            Some(U::StakeVoteDelegCert(v)) => Some(L::StakeVoteDelegCert(roundtrip(&v)?)),
            Some(U::StakeRegDelegCert(v)) => Some(L::StakeRegDelegCert(v.try_into()?)),
            Some(U::VoteRegDelegCert(v)) => Some(L::VoteRegDelegCert(v.try_into()?)),
            Some(U::StakeVoteRegDelegCert(v)) => Some(L::StakeVoteRegDelegCert(v.try_into()?)),
            Some(U::AuthCommitteeHotCert(v)) => Some(L::AuthCommitteeHotCert(roundtrip(&v)?)),
            Some(U::ResignCommitteeColdCert(v)) => Some(L::ResignCommitteeColdCert(roundtrip(&v)?)),
            Some(U::RegDrepCert(v)) => Some(L::RegDrepCert(v.try_into()?)),
            Some(U::UnregDrepCert(v)) => Some(L::UnregDrepCert(v.try_into()?)),
            Some(U::UpdateDrepCert(v)) => Some(L::UpdateDrepCert(roundtrip(&v)?)),
        };
        Ok(legacy::Certificate {
            redeemer: roundtrip_opt(&c.redeemer)?,
            certificate: cert,
        })
    }
}

// ---------------------------------------------------------------------------
// PParams + ParameterChangeAction
// ---------------------------------------------------------------------------

impl TryFrom<upstream::PParams> for legacy::PParams {
    type Error = ConvertError;

    fn try_from(p: upstream::PParams) -> Result<Self, Self::Error> {
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
            pool_influence: roundtrip_opt(&p.pool_influence)?,
            monetary_expansion: roundtrip_opt(&p.monetary_expansion)?,
            treasury_expansion: roundtrip_opt(&p.treasury_expansion)?,
            min_pool_cost: unwrap_u64(p.min_pool_cost.as_ref())?,
            protocol_version: roundtrip_opt(&p.protocol_version)?,
            max_value_size: p.max_value_size,
            collateral_percentage: p.collateral_percentage,
            max_collateral_inputs: p.max_collateral_inputs,
            cost_models: roundtrip_opt(&p.cost_models)?,
            prices: roundtrip_opt(&p.prices)?,
            max_execution_units_per_transaction: roundtrip_opt(&p.max_execution_units_per_transaction)?,
            max_execution_units_per_block: roundtrip_opt(&p.max_execution_units_per_block)?,
            min_fee_script_ref_cost_per_byte: roundtrip_opt(&p.min_fee_script_ref_cost_per_byte)?,
            pool_voting_thresholds: roundtrip_opt(&p.pool_voting_thresholds)?,
            drep_voting_thresholds: roundtrip_opt(&p.drep_voting_thresholds)?,
            min_committee_size: p.min_committee_size,
            committee_term_limit: p.committee_term_limit,
            governance_action_validity_period: p.governance_action_validity_period,
            governance_action_deposit: unwrap_u64(p.governance_action_deposit.as_ref())?,
            drep_deposit: unwrap_u64(p.drep_deposit.as_ref())?,
            drep_inactivity_period: p.drep_inactivity_period,
        })
    }
}

impl TryFrom<upstream::ParameterChangeAction> for legacy::ParameterChangeAction {
    type Error = ConvertError;

    fn try_from(a: upstream::ParameterChangeAction) -> Result<Self, Self::Error> {
        Ok(legacy::ParameterChangeAction {
            gov_action_id: roundtrip_opt(&a.gov_action_id)?,
            protocol_param_update: a.protocol_param_update.map(legacy::PParams::try_from).transpose()?,
            policy_hash: a.policy_hash,
        })
    }
}

// ---------------------------------------------------------------------------
// Governance
// ---------------------------------------------------------------------------

impl TryFrom<upstream::WithdrawalAmount> for legacy::WithdrawalAmount {
    type Error = ConvertError;

    fn try_from(w: upstream::WithdrawalAmount) -> Result<Self, Self::Error> {
        Ok(legacy::WithdrawalAmount {
            reward_account: w.reward_account,
            coin: unwrap_u64(w.coin.as_ref())?,
        })
    }
}

impl TryFrom<upstream::TreasuryWithdrawalsAction> for legacy::TreasuryWithdrawalsAction {
    type Error = ConvertError;

    fn try_from(a: upstream::TreasuryWithdrawalsAction) -> Result<Self, Self::Error> {
        Ok(legacy::TreasuryWithdrawalsAction {
            withdrawals: try_map(a.withdrawals, legacy::WithdrawalAmount::try_from)?,
            policy_hash: a.policy_hash,
        })
    }
}

impl TryFrom<upstream::GovernanceAction> for legacy::GovernanceAction {
    type Error = ConvertError;

    fn try_from(g: upstream::GovernanceAction) -> Result<Self, Self::Error> {
        use legacy::governance_action::GovernanceAction as L;
        use upstream::governance_action::GovernanceAction as U;
        let inner = match g.governance_action {
            None => None,
            Some(U::ParameterChangeAction(p)) => Some(L::ParameterChangeAction(p.try_into()?)),
            Some(U::HardForkInitiationAction(h)) => Some(L::HardForkInitiationAction(roundtrip(&h)?)),
            Some(U::TreasuryWithdrawalsAction(t)) => Some(L::TreasuryWithdrawalsAction(t.try_into()?)),
            Some(U::NoConfidenceAction(n)) => Some(L::NoConfidenceAction(roundtrip(&n)?)),
            Some(U::UpdateCommitteeAction(u)) => Some(L::UpdateCommitteeAction(roundtrip(&u)?)),
            Some(U::NewConstitutionAction(c)) => Some(L::NewConstitutionAction(roundtrip(&c)?)),
            Some(U::InfoAction(b)) => Some(L::InfoAction(b)),
        };
        Ok(legacy::GovernanceAction {
            governance_action: inner,
        })
    }
}

impl TryFrom<upstream::GovernanceActionProposal> for legacy::GovernanceActionProposal {
    type Error = ConvertError;

    fn try_from(p: upstream::GovernanceActionProposal) -> Result<Self, Self::Error> {
        Ok(legacy::GovernanceActionProposal {
            deposit: unwrap_u64(p.deposit.as_ref())?,
            reward_account: p.reward_account,
            gov_action: p.gov_action.map(legacy::GovernanceAction::try_from).transpose()?,
            anchor: roundtrip_opt(&p.anchor)?,
        })
    }
}

// ---------------------------------------------------------------------------
// Tx
// ---------------------------------------------------------------------------

impl TryFrom<upstream::Tx> for legacy::Tx {
    type Error = ConvertError;

    fn try_from(t: upstream::Tx) -> Result<Self, Self::Error> {
        Ok(legacy::Tx {
            inputs: try_map(t.inputs, legacy::TxInput::try_from)?,
            outputs: try_map(t.outputs, legacy::TxOutput::try_from)?,
            certificates: try_map(t.certificates, legacy::Certificate::try_from)?,
            withdrawals: try_map(t.withdrawals, legacy::Withdrawal::try_from)?,
            mint: try_map(t.mint, legacy::Multiasset::try_from)?,
            reference_inputs: try_map(t.reference_inputs, legacy::TxInput::try_from)?,
            witnesses: roundtrip_opt(&t.witnesses)?,
            collateral: t.collateral.map(legacy::Collateral::try_from).transpose()?,
            fee: unwrap_u64(t.fee.as_ref())?,
            validity: roundtrip_opt(&t.validity)?,
            successful: t.successful,
            auxiliary: roundtrip_opt(&t.auxiliary)?,
            hash: t.hash,
            proposals: try_map(t.proposals, legacy::GovernanceActionProposal::try_from)?,
        })
    }
}

// ---------------------------------------------------------------------------
// Block envelope
// ---------------------------------------------------------------------------

impl TryFrom<upstream::BlockBody> for legacy::BlockBody {
    type Error = ConvertError;

    fn try_from(b: upstream::BlockBody) -> Result<Self, Self::Error> {
        Ok(legacy::BlockBody {
            tx: try_map(b.tx, legacy::Tx::try_from)?,
        })
    }
}

impl TryFrom<upstream::Block> for legacy::Block {
    type Error = ConvertError;

    fn try_from(b: upstream::Block) -> Result<Self, Self::Error> {
        Ok(legacy::Block {
            header: roundtrip_opt(&b.header)?,
            body: b.body.map(legacy::BlockBody::try_from).transpose()?,
            timestamp: b.timestamp,
        })
    }
}
