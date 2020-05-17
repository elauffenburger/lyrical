use serde::{Serialize, Deserialize};

mod musix_match;
mod cached;

pub trait LyricsFetcher {
    fn fetch_lyrics(&mut self, song: &SongDescriptor) -> Result<String, String>;
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum SongUri {
    MusixMatchUri(String)
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct SongDescriptor {
    pub name: String,
    pub artist: String,
    pub uri: Option<SongUri>,
}

impl ToString for SongDescriptor {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

pub fn make_lyrics_fetcher() -> impl LyricsFetcher {
    let fetcher = musix_match::MusixMatchLyricsFetcher::new();
    let cache = cached::dev_cache::DevCache::new(
        cached::dev_cache::DevCacheOptionsBuilder::default()
            .write_eagerly(false)
            .build()
            .unwrap()
    );

    cached::CachedLyricsFetcher::new(
        fetcher, 
        cache, 
        cached::CachedLyricsFetcherOptionsBuilder::default()
            .cache_failure_as_empty_string(true)
            .build()
            .unwrap()
    )
}