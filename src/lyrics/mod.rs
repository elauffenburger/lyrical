use serde::{Serialize, Deserialize};

mod musix_match;
mod cached;
mod logging;
mod failover;

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
    let fetcher = failover::FailoverLyricsFetcher::new(vec![
        Box::new(musix_match::MusixMatchLyricsFetcher::new())
    ]);

    let cache = cached::dev_cache::DevCache::new(
        // TODO: make this configurable via args.
        cached::dev_cache::DevCacheOptionsBuilder::default()
            .write_eagerly(false)
            .build()
            .unwrap()
    );

    let cached_fetcher = cached::CachedLyricsFetcher::new(
        fetcher, 
        cache, 
        // TODO: make this configurable via args.
        cached::CachedLyricsFetcherOptionsBuilder::default()
            .cache_failures(true)
            .build()
            .unwrap()
    );

    // TODO: make this configurable via args.
    logging::LoggingLyricsFetcher::new(cached_fetcher)
}