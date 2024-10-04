use pallas_primitives::babbage;

use super::{BuildContext, BuildError, Ledger, PParams, TxExpr};

pub fn build<T, L>(tx: T, ledger: L) -> Result<babbage::Tx, BuildError>
where
    T: TxExpr,
    L: Ledger + 'static,
{
    let mut ctx = BuildContext {
        network: babbage::NetworkId::One,
        pparams: PParams {
            min_fee_a: 4,
            min_fee_b: 3,
            min_utxo_value: 2,
        },
        estimated_fee: 1,
        ledger: Box::new(ledger),
        tx_body: None,
    };

    let body = tx.eval_body(&ctx)?;

    ctx.tx_body = Some(body);

    let wit = tx.eval_witness_set(&ctx).unwrap();

    let tx = babbage::Tx {
        transaction_body: ctx.tx_body.take().unwrap(),
        transaction_witness_set: wit,
        auxiliary_data: pallas_codec::utils::Nullable::Null,
        success: true,
    };

    Ok(tx)
}
