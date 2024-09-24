use balius::*;

use pallas_primitives::conway::{Constr, PlutusData};
use pallas_txbuilder::{Address, AssetName, BuildBabbage, ExUnits, ReferenceScript};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Serialize, Deserialize, Clone)]
struct Config {
    free_validator: ReferenceScript,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
struct FreeMintRequest {
    token: AssetName,
    quantity: u64,
    #[serde_as(as = "DisplayFromStr")]
    recipient: Address,
    //fuel: UtxoSource,
}

fn free_mint(config: Env<Config>, params: Json<FreeMintRequest>) -> Result<UnsignedTx> {
    let tx = pallas_txbuilder::StagingTransaction::new()
        .reference_input(config.free_validator.clone())
        .mint_asset(
            config.free_validator.clone(),
            params.token.clone(),
            params.quantity as i64,
        )
        .output(
            pallas_txbuilder::OutputBuilder::new()
                .to_address(params.recipient.clone())
                .with_asset(
                    config.free_validator.clone(),
                    params.token.clone(),
                    params.quantity,
                )
                .with_min_lovelace()
                .build()
                .unwrap(),
        )
        .add_mint_redeemer(
            config.free_validator.clone(),
            (),
            Some(ExUnits { mem: 0, steps: 0 }),
        );

    let tx = tx.build_babbage_raw().unwrap();

    Ok(UnsignedTx(tx.tx_bytes.as_ref().to_vec()))
}

//#[balius::main]
fn main() -> balius::Worker {
    balius::Worker::new().handle_request("free-mint", free_mint)
}

balius::entrypoint!(main);

#[cfg(test)]
mod tests {
    use super::*;
    use pallas_txbuilder::{Address, Hash, TxoRef};
    use std::str::FromStr as _;

    #[test]
    fn test_free_mint() {
        let config = Config {
            free_validator: ReferenceScript {
                hash: Hash::from_str("ef7a1cebb2dc7de884ddf82f8fcbc91fe9750dcd8c12ec7643a99bbe").unwrap(),
                address: Address::from_bech32("addr1qx2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer3n0d3vllmyqwsx5wktcd8cc3sq835lu7drv2xwl2wywfgse35a3x").unwrap(),
                ref_txo: TxoRef {
                    hash: Hash::from_str(
                        "f7d3837715680f3a170e99cd202b726842d97f82c05af8fcd18053c64e33ec4f",
                    )
                    .unwrap(),
                    index: 0,
                },
            },
        };

        let request = FreeMintRequest {
            token: AssetName::new("TEST".as_bytes().to_vec()).unwrap(),
            quantity: 1,
            recipient: Address::from_bech32("addr1qx2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer3n0d3vllmyqwsx5wktcd8cc3sq835lu7drv2xwl2wywfgse35a3x").unwrap(),
        };

        free_mint(Env(config), Json(request)).unwrap();
    }
}
