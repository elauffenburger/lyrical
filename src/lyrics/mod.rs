use serde::{Serialize, Deserialize};

mod musix_match;
mod cached;

pub trait LyricsFetcher {
    fn fetch_lyrics(&self, song: &SongDescriptor) -> Result<String, String>;
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum SongUri {
    MusixMatchUri(String)
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct SongDescriptor {
    pub name: String,
    pub artist: String,
    pub uri: Option<SongUri>,
}

pub fn make_lyrics_fetcher() -> impl LyricsFetcher {
    let fetcher = musix_match::MusixMatchLyricsFetcher{};
    let cache = cached::dev_cache::DevCache::new();

    cached::CachedLyricsFetcher::new(fetcher, cache)
}