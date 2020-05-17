use super::*;

pub mod dev_cache;

pub trait Cache {
    fn save(&mut self, song: &SongDescriptor, lyrics: String) -> Result<(), String>;
    fn load(&self, song: &SongDescriptor) -> Result<Option<String>, String>;
}

pub struct CachedLyricsFetcher<T: LyricsFetcher, C: Cache> {
    cache: C, 
    fallback: T,
}

impl<T: LyricsFetcher, C: Cache> CachedLyricsFetcher<T, C> {
    pub fn new(fallback: T, cache: C) -> Self {
        CachedLyricsFetcher { cache, fallback }
    }
}

impl<T: LyricsFetcher, C: Cache> LyricsFetcher for CachedLyricsFetcher<T, C> {
    fn fetch_lyrics(&self, song: &SongDescriptor) -> Result<String, String> {
        match self.cache.load(song)? {
            Some(lyrics) => Ok(lyrics),
            None => self.fallback.fetch_lyrics(song)
        }
    }
}
