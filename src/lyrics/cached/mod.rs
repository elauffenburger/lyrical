use super::*;

pub mod dev_cache;

pub trait Cache {
    fn save(&mut self, song: &SongDescriptor, lyrics: String) -> Result<(), String>;
    fn load(&self, song: &SongDescriptor) -> Result<Option<String>, String>;
    fn write_back(&mut self) -> Result<(), String>;
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
    fn fetch_lyrics(&mut self, song: &SongDescriptor) -> Result<String, String> {
        match self.cache.load(song)? {
            Some(lyrics) => Ok(lyrics),
            None => {
                let lyrics = self.fallback.fetch_lyrics(song)?;
                self.cache.save(song, lyrics.clone())?;

                Ok(lyrics)
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