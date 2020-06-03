use std::collections::HashMap;

pub fn stringify_map_keys<T: ToString, U: Clone>(map: &HashMap<T, U>) -> HashMap<String, U> {
    map.into_iter()
        .fold(HashMap::new(), |mut acc, (key, value)| {
            acc.insert(key.to_string(), value.clone());

            acc
        })
}