use crate::{
    router::Router,
    wit::balius::app::{driver, kv, ledger, submit},
};

#[derive(Clone)]
pub struct Adapter {
    worker_id: String,
    router: Router,
}

impl Adapter {
    pub fn new(worker_id: String, router: Router) -> Self {
        Self { worker_id, router }
    }
}

impl kv::Host for Adapter {
    fn get_value(&mut self, key: String) -> Result<kv::Payload, kv::KvError> {
        todo!()
    }

    fn set_value(&mut self, key: String, value: kv::Payload) -> Result<(), kv::KvError> {
        println!("{}:{}", key, hex::encode(value));

        Ok(())
    }

    fn list_values(&mut self, prefix: String) -> Result<Vec<String>, kv::KvError> {
        todo!()
    }
}

impl submit::Host for Adapter {
    fn submit_tx(&mut self, tx: submit::Cbor) -> Result<(), submit::SubmitError> {
        println!("{}", hex::encode(tx));

        Ok(())
    }
}

impl driver::Host for Adapter {
    fn register_channel(&mut self, id: u32, pattern: driver::EventPattern) -> () {
        self.router.register_channel(&self.worker_id, id, &pattern);
    }
}

impl ledger::Host for Adapter {
    fn read_utxos(
        &mut self,
        refs: Vec<ledger::TxoRef>,
    ) -> Result<Vec<ledger::Utxo>, ledger::LedgerError> {
        let output = pallas::ledger::primitives::babbage::MintedTransactionOutput::PostAlonzo(pallas::ledger::primitives::babbage::MintedPostAlonzoTransactionOutput {
            address: pallas::ledger::addresses::Address::from_bech32("addr1qx2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer3n0d3vllmyqwsx5wktcd8cc3sq835lu7drv2xwl2wywfgse35a3x").unwrap().to_vec().into(),
            value: pallas::ledger::primitives::babbage::Value::Coin(5_000_000),
            datum_option: None,
            script_ref: None,
        });

        let cbor = pallas::codec::minicbor::to_vec(&output).unwrap();

        Ok(vec![ledger::Utxo {
            ref_: ledger::TxoRef {
                tx_hash: hex::decode(
                    "f7d3837715680f3a170e99cd202b726842d97f82c05af8fcd18053c64e33ec4f",
                )
                .unwrap(),
                tx_index: 0,
            },
            body: cbor,
        }])
    }

    fn search_utxos(
        &mut self,
        pattern: ledger::UtxoPattern,
    ) -> Result<Vec<ledger::Utxo>, ledger::LedgerError> {
        todo!()
    }
}
