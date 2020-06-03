use super::*;

pub type BeforeFn = dyn Fn(&SongDescriptor) -> ();
pub type AfterFn = dyn Fn(&SongDescriptor, &Result<String, String>) -> ();

pub struct DecoratingLyricsFetcher<T: LyricsFetcher> {
    fetcher: T,
    before: Box<BeforeFn>,
    after: Box<AfterFn>,
}

impl<T: LyricsFetcher> DecoratingLyricsFetcher<T> {
    pub fn new(fetcher: T, before: Box<BeforeFn>, after: Box<AfterFn>) -> Self {
        DecoratingLyricsFetcher {
            fetcher,
            before,
            after,
        }
    }
}

impl<T: LyricsFetcher> LyricsFetcher for DecoratingLyricsFetcher<T> {
    fn fetch_lyrics(&mut self, song: &SongDescriptor) -> Result<String, String> {
        (self.before)(song);

        let result = self.fetcher.fetch_lyrics(song);

        (self.after)(song, &result);

        result
    }
}

impl<T: LyricsFetcher> Debug for DecoratingLyricsFetcher<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        formatter.write_str("DecoratingLyricsFetcher { }")
    }
}