/// In memory implementation of KV for development usage.
use std::collections::BTreeMap;

use crate::wit::balius::app::kv as wit;
use std::ops::Bound;
use wit::{KvError, Payload};

pub struct MemoryKv {
    map: BTreeMap<String, Vec<u8>>,
}
impl MemoryKv {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }
}
impl Default for MemoryKv {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl wit::Host for MemoryKv {
    async fn get_value(&mut self, key: String) -> Result<Payload, KvError> {
        match self.map.get(&key) {
            Some(value) => Ok(value.clone()),
            None => Err(KvError::NotFound(key)),
        }
    }

    async fn set_value(&mut self, key: String, value: Payload) -> Result<(), KvError> {
        let _ = self.map.insert(key, value);
        Ok(())
    }

    async fn list_values(&mut self, prefix: String) -> Result<Vec<String>, KvError> {
        let range = self
            .map
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
