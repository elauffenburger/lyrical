use super::*;

pub struct FailoverLyricsFetcher {
    fetchers: Vec<Box<dyn LyricsFetcher>>
}

impl FailoverLyricsFetcher {
    pub fn new(fetchers: Vec<Box<dyn LyricsFetcher>>) -> Self {
        FailoverLyricsFetcher { fetchers }
    }
}

impl LyricsFetcher for FailoverLyricsFetcher {
    fn fetch_lyrics(&mut self, song: &SongDescriptor) -> Result<String, String> {
        for fetcher in &mut self.fetchers {
            match fetcher.fetch_lyrics(song) {
                res @ Ok(_) => return res,
                _ => {}
            };
        }

        Err(format!("Failed to fetch lyrics for {:?}", &song))
    }
}