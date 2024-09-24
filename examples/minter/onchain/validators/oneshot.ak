use cardano/transaction.{OutputReference, Transaction}

pub type Emit {
    utxo_ref: OutputReference,
    token_name: ByteArray,
}

validator oneshot {
    mint(redeemer: Emit, _policy_id: ByteArray, _tx: Transaction) {
        True
    }
}