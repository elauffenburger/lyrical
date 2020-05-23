mod caching;
mod decorating;
mod failover;
mod musixmatch;
mod simplifying;
mod song;

use std::fmt::Debug;

pub use song::*;

use caching::*;
use decorating::*;
use failover::*;
use musixmatch::*;
use simplifying::*;

pub trait LyricsFetcher: Debug {
    fn fetch_lyrics(&mut self, song: &SongDescriptor) -> Result<String, String>;
}

pub fn make_lyrics_fetcher() -> impl LyricsFetcher {
    // TODO: make this configurable via args.
    let proxies = vec![
        None,
        Some("https://103.83.116.210:55443"),
        Some("https://103.241.227.105:50313"),
    ];

    // The main fetcher that will be used for retrieiving song lyrics we don't
    // have cached yet.
    let main_fetcher = FailoverLyricsFetcher::new({
        // First, try musixmatch directly with each proxy.
        let mut fetchers = make_musixmatch_fetchers(proxies.clone());

        // Next, try simplifying the song name and hitting musixmatch again.
        fetchers.push(Box::new(
            SimplifyingLyricsFetcher::new(
                FailoverLyricsFetcher::new(
                    make_musixmatch_fetchers(proxies.clone())))));

        fetchers
    });

    // The cache to use by the [CachingLyricsFetcher] we're going to construct.
    let cache = DevCache::new(
        // TODO: make this configurable via args.
        DevCacheOptionsBuilder::default()
            .write_eagerly(false)
            .build()
            .unwrap()
    );

    // A fetcher that will use [main_fetcher] as a fallback when it can't find
    // the lyrics for a song in the cache.
    let caching_fetcher = CachingLyricsFetcher::new(
        main_fetcher, 
        cache, 
        // TODO: make this configurable via args.
        CachingLyricsFetcherOptionsBuilder::default()
            .cache_failures(true)
            .retry_cached_failures(true)
            .build()
            .unwrap()
    );

    // TODO: make use of this fetcher configurable via args.
    // A fetcher that will log periodic results to stdout.
    let logging_fetcher = DecoratingLyricsFetcher::new(
        caching_fetcher,
        Box::new(|song| println!("Fetching song {:?}", song)),
        Box::new(|song, result|
            println!("Fetched song {:?}; success: {}", song, result.is_ok()))
    );

    logging_fetcher
}

fn make_musixmatch_fetchers<'a>(proxies: Vec<Option<&'a str>>) -> Vec<Box<dyn LyricsFetcher>> {
    proxies.iter()
        .map(|proxy| {
            let options = MusixMatchLyricsFetcherOptionsBuilder::default()
                .proxy(match proxy {
                    Some(proxy) => Some(proxy.to_string()),
                    None => None
                })
                .build()
                .unwrap();

            Box::new(MusixMatchLyricsFetcher::new(options)) as Box<dyn LyricsFetcher>
        })
        .collect()
}