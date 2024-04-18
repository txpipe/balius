const TOKEN_TO_BUY: &AssetClass = AssetClass {
    name: "tokenA",
    policy_id: "",
};
const TOKEN_TO_SELL: &AssetClass = AssetClass {
    name: "tokenB",
    policy_id: "",
};
const PRICE: &int = 504000;

#[on_chain(mints=ORDER_MINTING_POLICY)]
fn buyer_bot(tx: Tx) {
    let mut orders = vec![];
    for (output_index, output) in tx.outputs.iter().enumerate() {
	let order_value = output
	    .non_ada_assets
	    .iter()
	    .filter(|x| is_control_token(x) || is_token_to_buy(x))
	    .collect();

	if (is_valid_order_value(order_value)) {
	    let order_datum = output.datum_as::<OrderDatum>();
	    if (order_datum.receive_assetclass == TOKEN_TO_SELL && is_valid_price(order_value, order_datum.amount)) {
		orders.add(output_index);
	    }
	}
    }
    if (!orders.is_empty()) {
	// At the moment we only consume the first order.
        let order_index = orders.into_iter().nth(0);
	let order_to_resolve = OutputRef::new(tx.hash(), order_index);
        let unbalancedTx = build_tx_resolve(order_to_resolve, , wallet::address());

        // "crane::wallet" maybe?
        // Probably the balance function could have an Option argument, to specify some
        // balancing algorithm (https://hackage.haskell.org/package/cardano-coin-selection)
	let balancedTx = wallet::balance(unbalancedTx);
	let signedTx = wallet::sign(balancedTx);
	crane::submit(signedTx);
    }
}

fn is_valid_price(order_value: Vec<MultiEraPolicyAssets>, order_amount: u64) -> bool { todo!() }

/// The value of an order is correct if it has only two tokens besides the ADAs.
fn is_valid_order_value(value: Vec<MultiEraPolicyAssets>) -> bool {
    value.len() == 2;
}

fn is_control_token(v: MultiEraPolicyAssets) -> bool {
    v.assets.policy() == ORDER_MINTING_POLICY && v.assets.any_coin == 1
}

fn is_token_to_buy(v: MultiEraPolicyAssets) -> bool {
    v.assets.policy() == TOKEN_TO_BUY.policy_id && v.assets.to_ascii_name() == TOKEN_TO_BUY.name
}
