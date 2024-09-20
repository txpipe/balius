mod ddk;
mod sundae;

use crate::ddk::*;

struct MyParams {
    sell_price: f32,
    sell_quantity: u64,
    max_slippage: f32,
}

struct MyDatum {}

fn on_some_utxo(utxo: Utxo<sundae::PoolDatum>, config: Config<MyParams>) {
    // This is where your logic goes. Do whatever you need with the
}

wit_bindgen::generate!({
    world: "bod",
});

struct MyBod;

impl Guest for MyBod {
    fn handle(evt: Event) -> Result<(), HandleError> {
        Router::on_utxo()
            .at_address("addr1xxx")
            .holding_token("asset1xxxx")
            .handle_with(on_some_utxo)
            .bind(evt)
    }
}

export!(MyBod);
