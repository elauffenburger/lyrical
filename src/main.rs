#[macro_use]
extern crate maplit;
extern crate regex;
extern crate reqwest;
extern crate scraper;
extern crate serde;
extern crate tokio;

mod lyrics;
mod utils;
mod word_count;

use std::collections::HashMap;

fn main() {
    // TODO: make this an input.
    let songs = vec![];

    print_word_counts_for_songs(songs);
}

fn print_word_counts_for_songs(songs: Vec<lyrics::SongDescriptor>) {
    let fetcher = lyrics::make_lyrics_fetcher();

    let results = songs.into_iter()
        .map(|song| {
            let count = get_word_count_for_song(&fetcher, &song);

            (song, count)
        });

    for result in results {
        match &result.1 {
            Ok(count) => println!("song: {:?}\nlyrics: {:?}\n\n", result.0, count),
            Err(err) => println!("error fetching song {:?}: {:?}\n\n", result.0, err)
        }
    }
}

fn get_word_count_for_song(fetcher: &dyn lyrics::LyricsFetcher, song: &lyrics::SongDescriptor) -> Result<HashMap<String, i32>, String> {
    fetcher.fetch_lyrics(song)
        .map(|lyrics| word_count::count_words(lyrics))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::*;

    struct MockLyricsFetcher {
        pub lyrics: String,
    }

    impl lyrics::LyricsFetcher for MockLyricsFetcher {
        fn fetch_lyrics(&self, _song: &lyrics::SongDescriptor) -> Result<String, String> {
            Ok(self.lyrics.clone())
        }
    }

    #[test]
    fn can_get_word_count_for_song() {
        let fetcher = MockLyricsFetcher { lyrics: include_str!("../test_data/songs/house_of_fire.txt").to_string() };

        let song = lyrics::SongDescriptor {
            name: "House of Fire".to_string(),
            artist: "Dave Rodgers".to_string(),
            uri: None
        };

        let count = get_word_count_for_song(&fetcher, &song);

        assert_eq!(count, Ok(stringify_map(hashmap!{
            "'cause" => 2,
            "i" => 16,
            "i'm" => 2,
            "all" => 6,
            "be" => 4,
            "best" => 2,
            "by" => 2,
            "can" => 2,
            "do" => 1,
            "don't" => 2,
            "end" => 2,
            "fire" => 11,
            "fly" => 2,
            "free" => 2,
            "from" => 2,
            "go" => 12,
            "gonna" => 2,
            "hear" => 2,
            "heart" => 1,
            "house" => 8,
            "in" => 2,
            "into" => 1,
            "is" => 2,
            "just" => 4,
            "keep" => 1,
            "know" => 2,
            "let" => 12,
            "lies" => 2,
            "life" => 5,
            "me" => 14,
            "more" => 2,
            "my" => 8,
            "now" => 6,
            "of" => 12,
            "on" => 2,
            "play" => 1,
            "rays" => 2,
            "reach" => 2,
            "real" => 1,
            "really" => 1,
            "satellite" => 2,
            "see" => 2,
            "side" => 2,
            "silence" => 2,
            "sky" => 4,
            "spend" => 4,
            "star" => 2,
            "stay" => 2,
            "survive" => 1,
            "take" => 4,
            "the" => 21,
            "time" => 4,
            "to" => 12,
            "tonight" => 2,
            "until" => 2,
            "wanna" => 12,
            "welcome" => 4,
            "we'll" => 2,
            "when" => 2,
            "will" => 2,
            "you" => 6,
            "your" => 4,
        })));
    }
}