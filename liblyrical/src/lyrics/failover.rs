use super::*;

#[derive(Debug)]
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
                // TODO: make printing this configurable via args.
                Err(err) => println!("Failed to fetch lyrics for song {:?} using {:?}: {}", song, fetcher, err)
            };
        }

        Err(format!("Failed to fetch lyrics for {:?}", song))
    }
}