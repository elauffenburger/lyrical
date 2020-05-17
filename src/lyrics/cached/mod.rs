use super::*;

pub mod dev_cache;

pub trait Cache {
    fn save(&mut self, song: &SongDescriptor, lyrics: String) -> Result<(), String>;
    fn load(&self, song: &SongDescriptor) -> Result<Option<String>, String>;
    fn write_back(&mut self) -> Result<(), String>;
}

#[derive(Default, Builder)]
#[builder(setter(into))]
pub struct CachedLyricsFetcherOptions {
    cache_failure_as_empty_string: bool,
}

pub struct CachedLyricsFetcher<T: LyricsFetcher, C: Cache> {
    cache: C, 
    fallback: T,
    options: CachedLyricsFetcherOptions
}

impl<T: LyricsFetcher, C: Cache> CachedLyricsFetcher<T, C> {
    pub fn new(fallback: T, cache: C, options: CachedLyricsFetcherOptions) -> Self {
        CachedLyricsFetcher { cache, fallback, options }
    }
}

impl<T: LyricsFetcher, C: Cache> LyricsFetcher for CachedLyricsFetcher<T, C> {
    fn fetch_lyrics(&mut self, song: &SongDescriptor) -> Result<String, String> {
        match self.cache.load(song)? {
            Some(lyrics) => Ok(lyrics),
            None => {
                match self.fallback.fetch_lyrics(song) {
                    Ok(lyrics) => {
                        self.cache.save(song, lyrics.clone())?;

                        Ok(lyrics)
                    },
                    err @ Err(_) => {
                        if self.options.cache_failure_as_empty_string {
                            self.cache.save(song, String::new())?;
                        }

                        err
                    }
                }
            }
        }
    }
}

impl<T: LyricsFetcher, C: Cache> Drop for CachedLyricsFetcher<T, C> {
    fn drop(&mut self) {
        match self.cache.write_back() {
            Err(err) => println!("Something went wrong while writing cache on CachedLyricsFetcher drop: {}", err),
            _ => {}
        }
    }
}