use crate::*;
use liblyrical::utils::{stringify_map_keys};

#[derive(Debug)]
struct MockLyricsFetcher {
    pub lyrics: String,
}

impl lyrics::LyricsFetcher for MockLyricsFetcher {
    fn fetch_lyrics(&mut self, _song: &lyrics::SongDescriptor) -> Result<String, String> {
        Ok(self.lyrics.clone())
    }
}

#[test]
fn can_get_word_count_for_song() {
    let mut fetcher = MockLyricsFetcher { lyrics: include_str!("../../test_data/songs/house_of_fire.txt").to_string() };

    let song = lyrics::SongDescriptor {
        name: "House of Fire".to_string(),
        artist: "Dave Rodgers".to_string(),
        uri: None
    };

    let songs = vec![song];
    let counts = get_word_counts_for_songs(&mut fetcher, &songs);

    assert_eq!(counts.get(0).unwrap().1, Ok(stringify_map_keys(&hashmap!{
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