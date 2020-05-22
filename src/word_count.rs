use std::collections::HashMap;

use regex;

pub type WordCounts = HashMap<String, i32>;

pub enum SortOrder {
    Ascending,
    Descending
}

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

pub fn sort_word_counts(word_counts: &WordCounts, sort_order: SortOrder) -> Vec<(&String, &i32)> {
    let mut kvps = word_counts
        .iter()
        .collect::<Vec<_>>();

    kvps.sort_by(|a, b| match sort_order {
        SortOrder::Ascending => a.1.partial_cmp(b.1).unwrap(),
        SortOrder::Descending => b.1.partial_cmp(a.1).unwrap(),
    });

    kvps
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
    fn count_words_can_count_simple_content() {
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
    fn count_words_strips_newlines_correctly() {
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
    fn count_words_lowercases_words() {
        count_test!("Hello World world hello", {
            "hello" => 2,
            "world" => 2,
        });
    }

    #[test]
    fn count_words_strips_commas() {
        count_test!("Hello, world hello,", {
            "hello" => 2,
            "world" => 1,
        });
    }

    #[test]
    fn count_words_strips_dashes() {
        count_test!("Hello - world -hello", { "hello" => 2, "world" => 1, });
    }

    #[test]
    fn count_words_strips_periods() {
        count_test!("Hello . world .hello", { "hello" => 2, "world" => 1, });
    }

    #[test]
    fn count_words_strips_parens() {
        count_test!("Hello ( ) (world )hello", { "hello" => 2, "world" => 1, });
    }
    
    #[test]
    fn count_words_strips_bangs() {
        count_test!("Hello ! !world hello!", { "hello" => 2, "world" => 1, });
    }

    #[test]
    fn count_words_strips_questions() {
        count_test!("Hello ? ?world hello?", { "hello" => 2, "world" => 1, });
    }

    #[test]
    fn sort_word_counts_descending_sorts_descending() {
        let word_counts = stringify_map_keys(&hashmap!{ "hello" => 1, "world" => 2 });
        let result = sort_word_counts(&word_counts, SortOrder::Descending);

        assert_eq!(result, vec![(&"world".to_string(), &2i32), (&"hello".to_string(), &1i32)])
    }

    #[test]
    fn sort_word_counts_ascending_sorts_ascending() {
        let word_counts = stringify_map_keys(&hashmap!{ "hello" => 2, "world" => 1 });
        let result = sort_word_counts(&word_counts, SortOrder::Ascending);

        assert_eq!(result, vec![(&"world".to_string(), &1i32), (&"hello".to_string(), &2i32)])
    }
}