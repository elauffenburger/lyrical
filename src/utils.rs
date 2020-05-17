use std::collections::HashMap;

pub fn stringify_map<'a>(map: HashMap<&'a str, i32>) -> HashMap<String, i32> {
    map.into_iter()
        .fold(HashMap::new(), |mut acc, (key, value)| {
            acc.insert(key.to_string(), value);

            acc
        })
}