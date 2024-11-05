use pallas_codec::utils::KeyValuePairs;
use pallas_crypto::hash::Hash;
use pallas_primitives::babbage::{self, Value};
use std::collections::HashMap;

pub fn fold_assets<T>(
    acc: &mut HashMap<pallas_codec::utils::Bytes, T>,
    item: KeyValuePairs<pallas_codec::utils::Bytes, T>,
) where
    T: std::ops::AddAssign + Copy,
{
    for (key, value) in item.to_vec() {
        acc.entry(key).and_modify(|e| *e += value).or_insert(value);
    }
}

pub fn fold_multiassets<T>(
    acc: &mut HashMap<Hash<28>, HashMap<pallas_codec::utils::Bytes, T>>,
    item: KeyValuePairs<Hash<28>, KeyValuePairs<pallas_codec::utils::Bytes, T>>,
) where
    T: std::ops::AddAssign + Copy,
{
    for (key, value) in item.to_vec() {
        let mut map = acc.remove(&key).unwrap_or_default();
        fold_assets(&mut map, value);
        acc.insert(key, map);
    }
}

pub fn aggregate_assets<T>(
    items: impl IntoIterator<Item = babbage::Multiasset<T>>,
) -> babbage::Multiasset<T>
where
    T: std::ops::AddAssign + Copy,
{
    let mut total_assets = HashMap::new();

    for assets in items {
        fold_multiassets(&mut total_assets, assets);
    }

    total_assets
        .into_iter()
        .map(|(policy_id, assets)| (policy_id, assets.into()))
        .collect()
}

pub fn aggregate_values(items: impl IntoIterator<Item = Value>) -> Value {
    let mut total_coin = 0;
    let mut total_assets = KeyValuePairs::Def(vec![]);

    for value in items {
        let (coin, assets) = match value {
            Value::Coin(x) => (x, KeyValuePairs::Def(vec![])),
            Value::Multiasset(x, y) => (x, y),
        };

        total_coin += coin;
        total_assets = aggregate_assets(vec![total_assets, assets]);
    }

    if total_assets.is_empty() {
        Value::Coin(total_coin)
    } else {
        Value::Multiasset(total_coin, total_assets)
    }
}

pub fn multiasset_coin_to_mint(assets: babbage::Multiasset<babbage::Coin>) -> babbage::Mint {
    assets
        .to_vec()
        .into_iter()
        .map(|(policy, assets)| {
            let assets: KeyValuePairs<_, _> = assets
                .to_vec()
                .into_iter()
                .map(|(name, quantity)| (name, quantity as i64))
                .collect();

            (policy, KeyValuePairs::from(assets))
        })
        .collect()
}

pub fn multiasset_coin_to_burn(assets: babbage::Multiasset<babbage::Coin>) -> babbage::Mint {
    assets
        .to_vec()
        .into_iter()
        .map(|(policy, assets)| {
            let assets: KeyValuePairs<_, _> = assets
                .to_vec()
                .into_iter()
                .map(|(name, quantity)| (name, -(quantity as i64)))
                .collect();

            (policy, KeyValuePairs::from(assets))
        })
        .collect()
}

pub fn value_saturating_add_coin(value: Value, coin: i64) -> Value {
    match value {
        Value::Coin(x) => Value::Coin(x.saturating_add_signed(coin)),
        Value::Multiasset(x, assets) => Value::Multiasset(x.saturating_add_signed(coin), assets),
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use super::*;
    use pallas_primitives::alonzo::Value;

    #[test]
    fn test_add_values_coin_only() {
        let value_a = Value::Coin(100);
        let value_b = Value::Coin(200);

        let result = aggregate_values(vec![value_a, value_b]);

        assert_eq!(result, Value::Coin(300));
    }

    #[test]
    fn test_add_values_same_asset() {
        let policy_id =
            Hash::<28>::from_str("bb4bc871e84078de932d392186dd3093b8de93505178d88d89b7ac98")
                .unwrap();

        let asset_name = "pepe".as_bytes().to_vec();

        let value_a = Value::Multiasset(
            100,
            KeyValuePairs::Def(vec![(
                policy_id,
                KeyValuePairs::Def(vec![(asset_name.clone().into(), 50)]),
            )]),
        );
        let value_b = Value::Multiasset(
            200,
            KeyValuePairs::Def(vec![(
                policy_id,
                KeyValuePairs::Def(vec![(asset_name.clone().into(), 30)]),
            )]),
        );

        let result = aggregate_values(vec![value_a, value_b]);

        assert_eq!(
            result,
            Value::Multiasset(
                300,
                KeyValuePairs::Def(vec![(
                    policy_id,
                    KeyValuePairs::Def(vec![(asset_name.clone().into(), 80)]),
                )]),
            )
        );
    }
}
