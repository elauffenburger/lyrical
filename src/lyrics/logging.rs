use super::*;

pub struct LoggingLyricsFetcher<T: LyricsFetcher> {
    fetcher: T
}

impl<T: LyricsFetcher> LoggingLyricsFetcher<T> {
    pub fn new(fetcher: T) -> Self {
        LoggingLyricsFetcher { fetcher }
    }
}

impl<T: LyricsFetcher> LyricsFetcher for LoggingLyricsFetcher<T> {
    fn fetch_lyrics(&mut self, song: &SongDescriptor) -> Result<String, String> {
        println!("Fetching song {:?}...", &song);

        self.fetcher.fetch_lyrics(song)
    }
}