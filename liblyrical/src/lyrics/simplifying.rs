use super::*;

use regex;

#[derive(Debug)]
pub struct SimplifyingLyricsFetcher<T: LyricsFetcher> {
    fetcher: T
}

impl<T: LyricsFetcher> SimplifyingLyricsFetcher<T> {
    pub fn new(fetcher: T) -> Self {
        SimplifyingLyricsFetcher { fetcher }
    }
}

impl<T: LyricsFetcher> LyricsFetcher for SimplifyingLyricsFetcher<T> {
    fn fetch_lyrics(&mut self, song: &SongDescriptor) -> Result<String, String> {
        let song = SongDescriptor {
            name: simplify_name(&song.name),
            artist: song.artist.clone(),
            uri: None
        };

        self.fetcher.fetch_lyrics(&song)
    }
}

fn simplify_name<'a>(name: &'a str) -> String {
    let remove_after_hyphen_regex = regex::Regex::new(r" - (?:.*?)$").unwrap();

    remove_after_hyphen_regex
        .replace_all(name, "")
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    struct FakeLyricsFetcher {
        on_fetch_lyrics: Box<dyn FnMut(&SongDescriptor) -> ()>,
    }

    impl LyricsFetcher for FakeLyricsFetcher {
        fn fetch_lyrics(&mut self, song: &SongDescriptor) -> Result<String, String> {
            (self.on_fetch_lyrics)(song);

            Ok("".to_string())
        }
    }

    impl Debug for FakeLyricsFetcher {
        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
            formatter.write_str("FakeLyricsFetcher { }")
        }
    }

    #[test]
    fn simplify_name_removes_content_after_hyphen() {
        assert_eq!(simplify_name("foo bar - baz qux quux"), "foo bar".to_string())
    }

    #[test]
    fn simplifying_lyrics_fetcher_simplifies_song_name() {
        let mut fetched_song = None;
        let fetched_song_ptr = &mut fetched_song as *mut Option<SongDescriptor>;

        let fake_fetcher = FakeLyricsFetcher {
            on_fetch_lyrics: Box::new(move |song| {
                unsafe {
                    // Hack to go around borrow checker for testing.
                    *fetched_song_ptr = Some(song.clone());
                }
            })
        };

        let mut fetcher = SimplifyingLyricsFetcher::new(fake_fetcher);

        let song = SongDescriptor { 
            name: String::from("foo bar - baz qux quux"), 
            artist: String::from("mr. foo"), 
            uri: None,
        };

        fetcher.fetch_lyrics(&song).unwrap();

        let expected_song = SongDescriptor {
            name: String::from("foo bar"),
            artist: String::from("mr. foo"),
            uri: None
        };

        assert_eq!(fetched_song, Some(expected_song));
    }
}