use super::{
    primitives, BuildContext, BuildError, Ledger, PParams, TxExpr, TxoRef, UtxoPattern, UtxoSet,
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
        todo!()
    }

    fn read_params(&self) -> Result<PParams, BuildError> {
        let bytes = crate::wit::balius::app::ledger::read_params()?;

        serde_json::from_slice(&bytes)
            .map_err(|_| BuildError::LedgerError("failed to parse params json".to_string()))
    }
}

pub fn build<T, L>(tx: T, ledger: L) -> Result<primitives::Tx, BuildError>
where
    T: TxExpr,
    L: Ledger + 'static,
{
    let mut ctx = BuildContext {
        network: primitives::NetworkId::Testnet,
        pparams: ledger.read_params()?,
        estimated_fee: 1,
        ledger: Box::new(ledger),
        tx_body: None,
    };

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
