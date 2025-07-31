/// In memory implementation of KV for development usage.
use std::collections::BTreeMap;

use crate::wit::balius::app::kv as wit;
use std::ops::Bound;
use wit::{KvError, Payload};

use super::KvProvider;

#[derive(Default, Clone)]
pub struct MemoryKv {
    map: BTreeMap<String, BTreeMap<String, Vec<u8>>>,
}

impl From<BTreeMap<String, BTreeMap<String, Vec<u8>>>> for MemoryKv {
    fn from(value: BTreeMap<String, BTreeMap<String, Vec<u8>>>) -> Self {
        Self { map: value }
    }
}

#[async_trait::async_trait]
impl KvProvider for MemoryKv {
    async fn get_value(&mut self, worker_id: &str, key: String) -> Result<Payload, KvError> {
        match self.map.entry(worker_id.to_string()).or_default().get(&key) {
            Some(value) => Ok(value.clone()),
            None => Err(KvError::NotFound(key)),
        }
    }

    async fn set_value(
        &mut self,
        worker_id: &str,
        key: String,
        value: Payload,
    ) -> Result<(), KvError> {
        let _ = self
            .map
            .entry(worker_id.to_string())
            .or_default()
            .insert(key, value);
        Ok(())
    }

    async fn list_values(
        &mut self,
        worker_id: &str,
        prefix: String,
    ) -> Result<Vec<String>, KvError> {
        let range = self
            .map
            .entry(worker_id.to_string())
            .or_default()
            .range((Bound::Included(prefix.clone()), Bound::Unbounded));
        let mut result = vec![];
        for (k, _) in range {
            if k.starts_with(&prefix) {
                result.push(k.to_string());
            } else {
                // Sorted, if prefix doesn't match then we break
                break;
            }
        }
        Ok(result)
    }
}
