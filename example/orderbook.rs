// TODO: add some data validation (research warp)
const ORDER_MINTING_POLICY: &string = "";
const CONTROL_TOKEN_NAME: &string = "";
const ORDER_BOOK: &string = "";
const VALIDATOR_REF_UTXO: &string = "";

struct AssetClass {
    name: String,
    policy_id: String,
}

struct OrderStartBody {
    sender_address: String,
    sent: (AssetClass, u64),
    receive: (AssetClass, u64),
}

struct AddressDetails {
    address_type: AddressType,
    network: Network,
    address_bech32: String,
    address_hex: String,
    payment_credential: Option<Credential>,
    stake_credential: Option<Credential>,
}

struct ResolveOrderBody {
    receiver_address: String,
}

struct CancelOrderBody {
    sender_address: String,
}

#[on_http(method = "POST", path = "/order")]
fn new(order_data: OrderStartBody) {
    // returns transaction to be signed by sender
    let datum = OrderDatum {
        // make ti so that the order needs to receive orderData.receive
    };
    let control_token_unit = format!("{}{}", ORDER_MINTING_POLICY, CONTROL_TOKEN_NAME);

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
}

#[on_http(method = "GET", path = "/order")]
// TODO: think how to cleanly validate query input
fn list(cursor: String, limit: u64) {
    let db = use_extension::<Database>();
    let orders = db.execute("SELECT * FROM orders LIMIT {} WHERE hash<{}", limit, cursor);
    orders
}

#[on_http(method="POST", path=("/order"/ String))]
// TODO: think of a clean way to indicate the dynamic path
fn resolve(tx_out_ref: String, resolve_data: ResolveOrderBody) {
    let payment_cred_hash = Address::payment_credential(resolve_data.receiver_address);
    let utxos = UTxO::by_ref([tx_out_ref]);
    let datum = utxos[0].datum_as::<OrderDatum>();
    let sender_address = datum.sender_address;
    let order_redeemer = Redeemer {};
    let control_token_unit = format!("{}{}", ORDER_MINTING_POLICY, CONTROL_TOKEN_NAME);
    let tx = Tx::new()
        .collect_from(utxos, order_redeemer)
        .read_from([VALIDATOR_REF_UTXO])
        .add_signer_key(payment_cred_hash)
        .mint([(control_token_unit, 1)])
        .pay_to_address(
            sender_address,                                 // address
            [(datum.receiver_unit, datum.receiver_amount)], // assets
        )
        .add_metadata(
            674,                          // label
            String::from("ResolveOrder"), // data
        );

    tx.serialize()
}

#[on_http(method="POST", path=("/order"/ String))]
// TODO: think of a clean way to indicate the dynamic path
fn cancel(tx_out_ref: String, cancel_data: CancelOrderBody) {
    // returns a transaction to be signed by the creator of the order
    let payment_cred_hash = Address::payment_credential(cancel_data.sender_address);
    // utxos at address ORDER_BOOK
    let utxos = UTxO::at(ORDER_BOOK);
    let tx_out_hash = tx_out_ref.split("#").next().unwrap();
    let target_utxo = utxos
        .iter()
        .find(|utxo| utxo.tx_hash == tx_out_hash)
        .unwrap();
    let datum = target_utxo.datum_as::<OrderDatum>();
    let order_redeemer = Redeemer {};
    let control_token_unit = format!("{}{}", ORDER_MINTING_POLICY, CONTROL_TOKEN_NAME);
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
}

enum OrderChange {
    New(Order),
    Cancel(Order),
    Resolve(Order),
    Other,
}

#[match_tx_variant(type="address_eq", value=ORDER_BOOK, details=true)]
fn on_order_book_change(tx: Transaction) {
    let inputs = tx.inputs;
    let outputs = tx.outputs;
    let change = match_tx_purpose(transaction);

    match change {
        OrderChange::New(order) => on_new_order(order),
        OrderChange::Cancel(order) => on_order_cancellation(order),
        OrderChange::Resolve(order) => on_order_resolution(order),
        _ => (),
    }
}

fn on_new_order(order: Order) {
    let (sent_unit, sent_amount) = order.send;
    let (receive_unit, receive_amount) = order.receive;
    let db = use_extension::<Database>();
    let utxo_ref = db.execute(
        "INSERT INTO orders (utxo_ref, s_address, s_unit, s_amount, r_unit, r_amount) VALUES, ({}, {}, {}, {}, {}) RETURNING utxo_ref",
        order.utxo_ref,
        order.sender_address,
        sent_unit,
        sent_amount,
        receive_unit,
        receive_amount
        );
}

fn on_order_cancellation(order: Order) {
    // update db
    let db = use_extension::<Database>();
    db.execute("DELETE FROM orders WHERE utxo_ref={}", order.utxo_ref);
}

fn on_order_resolution(order: Order) {
    // update db
    let db = use_extension::<Database>();
    db.execute(
        "UPDATE orders SET r_address={} resolved=TRUE",
        order.receiver_address,
    );
}
