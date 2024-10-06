use crate::wit::balius::app::submit as wit;

#[derive(Clone)]
pub enum Submit {
    Mock,
}

#[async_trait::async_trait]
impl wit::Host for Submit {
    async fn submit_tx(&mut self, tx: wit::Cbor) -> Result<(), wit::SubmitError> {
        println!("{}", hex::encode(tx));

        Ok(())
    }
}
