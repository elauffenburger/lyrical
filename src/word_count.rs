use std::collections::HashMap;

use regex;

pub fn count_words<'a>(src: String) -> HashMap<String, i32> {
    let allowed_word_regex = regex::Regex::new("[0-9a-zA-Z]+").unwrap();
    let punctuation_regex = regex::Regex::new("(\r?\n)|(,)").unwrap();

    punctuation_regex.replace_all(&src, " ")
        .split(" ")
        .fold(HashMap::new(), |mut acc, word| {
            let word = &word.to_lowercase();

            if !allowed_word_regex.is_match(word) {
                return acc;
            }

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
    use crate::utils::*;

    #[test]
    fn can_count_simple_content() {
        assert_eq!(count_words("hello world i am a str and i am proud".to_string()), stringify_map_keys(&hashmap!{
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

    #[test]
    fn strips_newlines_correctly() {
        assert_eq!(count_words("hello world\ni am a\nstr and i am proud".to_string()), stringify_map_keys(&hashmap!{
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

    #[test]
    fn lowercases_words() {
        assert_eq!(count_words("Hello World world hello".to_string()), stringify_map_keys(&hashmap!{
            "hello" => 2,
            "world" => 2,
        }));
    }

    #[test]
    fn strips_punctuation() {
        assert_eq!(count_words("Hello, - world hello,".to_string()), stringify_map_keys(&hashmap!{
            "hello" => 2,
            "world" => 1,
        }));
    }
}