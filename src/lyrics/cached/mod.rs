use std::fmt::Debug;

use super::*;

pub mod dev_cache;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum CacheEntry {
    Success(String),
    Failure(String),
}

pub trait Cache: Debug {
    fn save(&mut self, song: &SongDescriptor, entry: CacheEntry) -> Result<(), String>;
    fn load(&self, song: &SongDescriptor) -> Result<Option<CacheEntry>, String>;
    fn write_back(&mut self) -> Result<(), String>;
}

#[derive(Default, Builder, Debug)]
#[builder(setter(into))]
pub struct CachedLyricsFetcherOptions {
    cache_failures: bool,
    retry_cached_failures: bool,
}

#[derive(Debug)]
pub struct CachedLyricsFetcher<T: LyricsFetcher, C: Cache> {
    cache: C, 
    fallback: T,
    options: CachedLyricsFetcherOptions
}

impl<T: LyricsFetcher, C: Cache> CachedLyricsFetcher<T, C> {
    pub fn new(fallback: T, cache: C, options: CachedLyricsFetcherOptions) -> Self {
        CachedLyricsFetcher { cache, fallback, options }
    }

    fn fetch_lyrics_using_fallback(&mut self, song: &SongDescriptor) -> Result<String, String> {
        match self.fallback.fetch_lyrics(song) {
            // We found some lyrics; save the lyrics to cache and
            // return the result.
            Ok(lyrics) => {
                let entry = CacheEntry::Success(lyrics.clone());
                self.cache.save(song, entry)?;

                Ok(lyrics)
            },
            // We didn't find any lyrics :(
            Err(err) => {
                // If we should cache failures, do so.
                if self.options.cache_failures {
                    let entry = CacheEntry::Failure(err.clone());

                    self.cache.save(song, entry)?;
                }

                // Return the error.
                Err(err)
            }
        }
    }
}

impl<T: LyricsFetcher, C: Cache> LyricsFetcher for CachedLyricsFetcher<T, C> {
    fn fetch_lyrics(&mut self, song: &SongDescriptor) -> Result<String, String> {
        // Try to load the lyrics from cache.
        match self.cache.load(song)? {
            // We found a result in the cache.
            Some(entry) => match entry {
                CacheEntry::Success(lyrics) => Ok(lyrics),
                CacheEntry::Failure(err) => {
                    // Check if we should retry a cached failure.
                    // If so, do it; otherwise, just return the err directly.
                    match self.options.retry_cached_failures {
                        true => self.fetch_lyrics_using_fallback(song),
                        false => Err(err)
                    }
                } 
            },

            // We didn't find a result in the cache, so we'll need to use our
            // fallback fetcher.
            None => self.fetch_lyrics_using_fallback(song)
        }
    }
}

impl<T: LyricsFetcher, C: Cache> Drop for CachedLyricsFetcher<T, C> {
    fn drop(&mut self) {
        // Make sure we write the cache back on drop.
        match self.cache.write_back() {
            Err(err) => println!("Something went wrong while writing cache on CachedLyricsFetcher drop: {}", err),
            _ => {}
        }
    }
}