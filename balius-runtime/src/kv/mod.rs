use crate::wit::balius::app::kv as wit;

#[derive(Clone)]
pub enum Kv {
    Mock,
}

#[async_trait::async_trait]
impl wit::Host for Kv {
    async fn get_value(&mut self, key: String) -> Result<wit::Payload, wit::KvError> {
        todo!()
    }

    async fn set_value(&mut self, key: String, value: wit::Payload) -> Result<(), wit::KvError> {
        println!("{}:{}", key, hex::encode(value));

        Ok(())
    }

    async fn list_values(&mut self, prefix: String) -> Result<Vec<String>, wit::KvError> {
        todo!()
    }
}
