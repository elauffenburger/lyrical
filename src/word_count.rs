use std::collections::HashMap;

pub fn count_words<'a>(src: String) -> HashMap<String, i32> {
    src.split(" ")
        .fold(HashMap::new(), |mut acc, word| {
            let count = match acc.get(word) {
                Some(old_count) => old_count + 1,
                None => 1
            };

            acc.insert(word.to_string(), count);

            acc
        })
}

#[cfg(test)]
mod test {
    use maplit::hashmap;

    use super::*;

    #[test]
    fn can_count_simple_content() {
        assert_eq!(count_words("hello world i am a str and i am proud".to_string()), stringify_map(hashmap!{
            "hello" => 1,
            "world" => 1,
            "i" => 2,
            "am" => 2,
            "a" => 1,
            "str"=> 1,
            "and" => 1,
            "proud" => 1
        }));
    }

    fn stringify_map<'a>(map: HashMap<&'a str, i32>) -> HashMap<String, i32> {
        map.into_iter()
            .fold(HashMap::new(), |mut acc, (key, value)| {
                acc.insert(key.to_string(), value);

                acc
            })
    }
}