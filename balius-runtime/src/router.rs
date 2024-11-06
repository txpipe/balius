use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, RwLock},
};

use pallas::ledger::traverse::MultiEraOutput;

use crate::wit::balius::app::driver::{Event, EventPattern, UtxoPattern};

type WorkerId = String;
type ChannelId = u32;
type Method = String;
type AddressBytes = Vec<u8>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum MatchKey {
    RequestMethod(Method),
    EveryUtxo,
    UtxoAddress(AddressBytes),
}

fn infer_match_keys(pattern: &EventPattern) -> Vec<MatchKey> {
    match pattern {
        EventPattern::Request(x) => vec![MatchKey::RequestMethod(x.to_owned())],
        EventPattern::Utxo(UtxoPattern { address, token }) => match (address, token) {
            (None, None) => vec![MatchKey::EveryUtxo],
            (Some(address), None) => vec![MatchKey::UtxoAddress(address.to_vec())],
            _ => todo!(),
        },
        EventPattern::UtxoUndo(_) => todo!(),
        EventPattern::Timer(_) => todo!(),
        EventPattern::Message(_) => todo!(),
    }
}

type RouteMap = HashMap<MatchKey, HashSet<ChannelId>>;

#[derive(Default, Clone)]
pub struct Router {
    routes: RouteMap,
}

impl Router {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn register_channel(&mut self, channel: u32, pattern: &EventPattern) {
        let keys = infer_match_keys(pattern);

        for key in keys {
            let targets = self.routes.entry(key).or_default();

            targets.insert(channel);
        }
    }

    pub fn find_utxo_targets(
        &self,
        utxo: &MultiEraOutput,
    ) -> Result<HashSet<ChannelId>, super::Error> {
        let key = MatchKey::UtxoAddress(utxo.address()?.to_vec());
        let targets: HashSet<_> = self
            .routes
            .get(&key)
            .iter()
            .flat_map(|x| x.iter())
            .cloned()
            .collect();

        // TODO: match by policy / asset

        Ok(targets)
    }

    pub fn find_request_target(&self, method: &str) -> Result<ChannelId, super::Error> {
        let key = MatchKey::RequestMethod(method.to_owned());

        let targets = self.routes.get(&key).ok_or(super::Error::NoTarget)?;

        if targets.is_empty() {
            return Err(super::Error::NoTarget);
        }

        if targets.len() > 1 {
            return Err(super::Error::AmbiguousTarget);
        }

        let target = targets.iter().next().unwrap();

        Ok(target.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_channel() {
        let mut router = Router::new();

        let method = "test_method";
        let channel = 1;

        router.register_channel(channel, &EventPattern::Request(method.to_string()));

        let channel = router.find_request_target(method).unwrap();

        assert_eq!(channel, channel);
    }
}
