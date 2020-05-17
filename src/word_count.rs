use std::collections::HashMap;

use regex;

pub type WordCounts = HashMap<String, i32>;

// TODO: look through word count results and see if there are some anomalies.
pub fn count_words<'a>(src: String) -> WordCounts {
    let allowed_word_regex = regex::Regex::new(r"[0-9a-zA-Z]+").unwrap();
    let punctuation_regex = regex::Regex::new(r"(\r?\n)|([-.!?,()])").unwrap();

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

    macro_rules! count_test {
        ($input:expr, {$($key:expr => $value:expr),* $(,)?}) => {
            assert_eq!(count_words($input.to_string()), stringify_map_keys(&hashmap!{
                $($key => $value),*
            }));
        };
    }

    #[test]
    fn can_count_simple_content() {
        count_test!("hello world i am a str and i am proud", {
            "hello" => 1,
            "world" => 1,
            "i" => 2,
            "am" => 2,
            "a" => 1,
            "str"=> 1,
            "and" => 1,
            "proud" => 1
        })
    }

    #[test]
    fn strips_newlines_correctly() {
        count_test!("hello world\ni am a\nstr and i am proud", {
            "hello" => 1,
            "world" => 1,
            "i" => 2,
            "am" => 2,
            "a" => 1,
            "str"=> 1,
            "and" => 1,
            "proud" => 1
        });
    }

    #[test]
    fn lowercases_words() {
        count_test!("Hello World world hello", {
            "hello" => 2,
            "world" => 2,
        });
    }

    #[test]
    fn strips_commas() {
        count_test!("Hello, world hello,", {
            "hello" => 2,
            "world" => 1,
        });
    }

    #[test]
    fn strips_dashes() {
        count_test!("Hello - world -hello", { "hello" => 2, "world" => 1, });
    }

    #[test]
    fn strips_periods() {
        count_test!("Hello . world .hello", { "hello" => 2, "world" => 1, });
    }

    #[test]
    fn strips_parens() {
        count_test!("Hello ( ) (world )hello", { "hello" => 2, "world" => 1, });
    }
    
    #[test]
    fn strips_bangs() {
        count_test!("Hello ! !world hello!", { "hello" => 2, "world" => 1, });
    }

    #[test]
    fn strips_questions() {
        count_test!("Hello ? ?world hello?", { "hello" => 2, "world" => 1, });
    }
}