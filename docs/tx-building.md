# Transaction building

## Creating an order
```rust
    let tx = Tx::new()
        .to_contract(
            order_data.sender_address, // from
            order_data.ORDER_BOOK,     // to
            order_data.sent,           // assets
            InlineDatum(datum),        // datum
        )
        .mint([(control_token_unit, 1)]) // assets
        .read_from([VALIDATOR_REF_UTXO]);
    tx.serialize()
```
- `Tx::new()` should create a "blank" transaction.

- The method `.to_contract()` should specify that the transaction sends from an address (1st arguemnt) to a script address (2nd argument) assets (3rd argument) and a datum (4th argument).

- The `.mint()` method specifies the minting that this transaction will do as a list of tuples `(units, amounts)`. In this example it means, that it will **mint** `1` token of unit `control_token_unit`. If the number was negative, it would **burn** the specified amount of tokens.

- The `.read_from()` method specifies the reference inputs this transaction takes in.

- The `.serialize()` method should actually create a balance transaction such that satisfies every specification set for the request and serialize it.


### Cancelling an order
```rust
    let tx = Tx::new()
        .collect_from([target_utxo], order_redeemer)
        .read_from([VALIDATOR_REF_UTXO])
        .add_signer_key(payment_cred_hash)
        .mint([(control_token_unit, -1)])
        .add_metadata(
            774,                         // label
            String::from("CancelOrder"), // data
        );
    tx.serialize()
```

- `.collect_from()` specifies what utxos are going to be spent by the transaction. // TODO: @ale complete here.
-  `.add_metadata()` specifies the metadata added to the transaction // TODO: @ale complete here.


### Resolving an order
```rust
    let tx = Tx::new()
        .collect_from([target_utxo], order_redeemer)
        .read_from([VALIDATOR_REF_UTXO])
        .add_signer_key(payment_cred_hash)
        .mint([(control_token_unit, -1)])
        .add_metadata(
            774,                         // label
            String::from("CancelOrder"), // data
        );
    tx.serialize()
```

- `.add_signer_key()` specifies a wallet that has to sign this transaction

## Important issues
- what errors does `tx.serialize()` throw?
- how to set the status of an http response?
