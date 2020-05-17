mod musix_match;

pub trait LyricsFetcher {
    fn fetch_lyrics(&self, song: &SongDescriptor) -> Result<String, String>;
}

#[derive(Debug)]
pub enum SongUri {
    MusixMatchUri(String)
}

#[derive(Debug)]
pub struct SongDescriptor {
    pub name: String,
    pub artist: String,
    pub uri: Option<SongUri>,
}

pub fn make_lyrics_fetcher() -> impl LyricsFetcher {
    musix_match::MusixMatchLyricsFetcher{}
}