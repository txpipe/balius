use std::{collections::HashMap, ops::Deref as _};

use pallas_traverse::MultiEraValue;

use super::{
    asset_math, primitives, BuildContext, BuildError, Ledger, PParams, TxExpr, TxoRef, UtxoPattern,
    UtxoSet,
};

impl BuildContext {
    pub fn mint_redeemer_index(&self, policy: primitives::ScriptHash) -> Result<u32, BuildError> {
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
        _script: primitives::ScriptHash,
        _data: &primitives::PlutusData,
    ) -> primitives::ExUnits {
        // TODO
        primitives::ExUnits { mem: 8, steps: 8 }
    }
}

pub(crate) struct ExtLedgerFacade;

impl crate::txbuilder::Ledger for ExtLedgerFacade {
    fn read_utxos(&self, refs: &[TxoRef]) -> Result<UtxoSet, BuildError> {
        let refs: Vec<_> = refs.iter().cloned().map(Into::into).collect();
        let x = crate::wit::balius::app::ledger::read_utxos(&refs)?;

        let x: Vec<_> = x
            .into_iter()
            .map(|x| (TxoRef::from(x.ref_), x.body.to_vec()))
            .collect();

        Ok(UtxoSet::from_iter(x))
    }

    fn search_utxos(&self, pattern: &UtxoPattern) -> Result<UtxoSet, BuildError> {
        let pattern = pattern.clone().into();
        let mut utxos = HashMap::new();
        let max_items = 32;
        let mut utxo_page = Some(crate::wit::balius::app::ledger::search_utxos(
            &pattern, None, max_items,
        )?);
        while let Some(page) = utxo_page.take() {
            for utxo in page.utxos {
                utxos.insert(utxo.ref_.into(), utxo.body);
            }
            if let Some(next) = page.next_token {
                utxo_page = Some(crate::wit::balius::app::ledger::search_utxos(
                    &pattern,
                    Some(&next),
                    max_items,
                )?);
            }
        }
        Ok(utxos.into())
    }
}

pub fn build<T, L>(tx: T, ledger: L) -> Result<primitives::Tx, BuildError>
where
    T: TxExpr,
    L: Ledger + 'static,
{
    let mut ctx = BuildContext {
        network: primitives::NetworkId::Testnet,
        pparams: PParams {
            min_fee_a: 4,
            min_fee_b: 3,
            min_utxo_value: 2,
        },
        total_input: primitives::Value::Coin(0),
        spent_output: primitives::Value::Coin(0),
        estimated_fee: 0,
        ledger: Box::new(ledger),
        tx_body: None,
    };

    // Build the raw transaction, so we have the info needed to estimate fees and
    // compute change.
    let body = tx.eval_body(&ctx)?;

    let input_refs: Vec<_> = body
        .inputs
        .iter()
        .map(|i| TxoRef {
            hash: i.transaction_id,
            index: i.index,
        })
        .collect();
    let utxos = ctx.ledger.read_utxos(&input_refs)?;
    ctx.total_input =
        asset_math::aggregate_values(utxos.txos().map(|txo| input_into_conway(&txo.value())));
    ctx.spent_output = asset_math::aggregate_values(body.outputs.iter().map(output_into_conway));
    // TODO: estimate the fee
    ctx.estimated_fee = 2_000_000;

    // Now that we know the inputs/outputs/fee, build the "final" (unsigned)tx
    let body = tx.eval_body(&ctx)?;
    ctx.tx_body = Some(body);

    let wit = tx.eval_witness_set(&ctx).unwrap();

    let tx = primitives::Tx {
        transaction_body: ctx.tx_body.take().unwrap(),
        transaction_witness_set: wit,
        auxiliary_data: pallas_codec::utils::Nullable::Null,
        success: true,
    };

    Ok(tx)
}

// TODO: this belongs in pallas-traverse
// https://github.com/txpipe/pallas/pull/545
fn input_into_conway(value: &MultiEraValue) -> primitives::Value {
    use pallas_primitives::{alonzo, conway};
    match value {
        MultiEraValue::Byron(x) => conway::Value::Coin(*x),
        MultiEraValue::AlonzoCompatible(x) => match x.deref() {
            alonzo::Value::Coin(x) => conway::Value::Coin(*x),
            alonzo::Value::Multiasset(x, assets) => {
                let coin = *x;
                let assets = assets
                    .iter()
                    .filter_map(|(k, v)| {
                        let v: Vec<(conway::Bytes, conway::PositiveCoin)> = v
                            .iter()
                            .filter_map(|(k, v)| Some((k.clone(), (*v).try_into().ok()?)))
                            .collect();
                        Some((k.clone(), conway::NonEmptyKeyValuePairs::from_vec(v)?))
                    })
                    .collect();
                if let Some(assets) = conway::NonEmptyKeyValuePairs::from_vec(assets) {
                    conway::Value::Multiasset(coin, assets)
                } else {
                    conway::Value::Coin(coin)
                }
            }
        },
        MultiEraValue::Conway(x) => x.deref().clone(),
        _ => panic!("unrecognized value"),
    }
}

fn output_into_conway(output: &primitives::TransactionOutput) -> primitives::Value {
    use pallas_primitives::{alonzo, conway};
    match output {
        primitives::TransactionOutput::Legacy(o) => match &o.amount {
            alonzo::Value::Coin(c) => primitives::Value::Coin(*c),
            alonzo::Value::Multiasset(c, assets) => {
                let assets = assets
                    .iter()
                    .filter_map(|(k, v)| {
                        let v: Vec<(conway::Bytes, conway::PositiveCoin)> = v
                            .iter()
                            .filter_map(|(k, v)| Some((k.clone(), (*v).try_into().ok()?)))
                            .collect();
                        Some((k.clone(), conway::NonEmptyKeyValuePairs::from_vec(v)?))
                    })
                    .collect();
                if let Some(assets) = conway::NonEmptyKeyValuePairs::from_vec(assets) {
                    primitives::Value::Multiasset(*c, assets)
                } else {
                    primitives::Value::Coin(*c)
                }
            }
        },
        primitives::TransactionOutput::PostAlonzo(o) => o.value.clone(),
    }
}
