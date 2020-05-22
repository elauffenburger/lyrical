use std::fmt::Debug;

use serde::{Serialize, Deserialize};

mod musix_match;
mod cached;
mod decorating;
mod failover;
mod simplifying;

pub trait LyricsFetcher: Debug {
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
    // The main fetcher that will be used for retrieiving song lyrics we don't
    // have cached yet.
    let main_fetcher = failover::FailoverLyricsFetcher::new(vec![
        Box::new(musix_match::MusixMatchLyricsFetcher::new()),
        Box::new(simplifying::SimplifyingLyricsFetcher::new(
            musix_match::MusixMatchLyricsFetcher::new()
        ))
    ]);

    // The cache to use by the [CachedLyricsFetcher] we're going to construct.
    let cache = cached::dev_cache::DevCache::new(
        // TODO: make this configurable via args.
        cached::dev_cache::DevCacheOptionsBuilder::default()
            .write_eagerly(false)
            .build()
            .unwrap()
    );

    // A fetcher that will use [main_fetcher] as a fallback when it can't find
    // the lyrics for a song in the cache.
    let cached_fetcher = cached::CachedLyricsFetcher::new(
        main_fetcher, 
        cache, 
        // TODO: make this configurable via args.
        cached::CachedLyricsFetcherOptionsBuilder::default()
            .cache_failures(true)
            .retry_cached_failures(true)
            .build()
            .unwrap()
    );

    // TODO: make use of this fetcher configurable via args.
    // A fetcher that will log periodic results to stdout.
    let logging_fetcher = decorating::DecoratingLyricsFetcher::new(
        cached_fetcher,
        Box::new(|song| println!("Fetching song {:?}", song)),
        Box::new(|song, result|
            println!("Fetched song {:?}; success: {}", song, result.is_ok()))
    );

    logging_fetcher
}