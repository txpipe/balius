use cardano/transaction.{OutputReference, Transaction}

validator free {
    mint(_redeemer: Data, _policy_id: ByteArray, _tx: Transaction) {
        True
    }
}