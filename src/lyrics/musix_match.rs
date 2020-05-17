use reqwest;
use scraper::{Html, Selector};

use super::*;

const MUSIX_MATCH_URI: &'static str = "https://www.musixmatch.com";
const MUSIX_MATCH_SEARCH_URI: &'static str = "https://www.musixmatch.com/search";
const MUSIX_MATCH_SEARCH_TRACK_URI_SELECTOR: &'static str = "#search-all-results > .main-panel > .box > .box-content .track-card > meta[itemprop=\"url\"]";
const MUSIX_MATCH_LYRICS_SEGMENT_SELECTOR: &'static str = ".mxm-lyrics__content > span";

#[derive(Debug)]
pub struct MusixMatchLyricsFetcher {}

impl MusixMatchLyricsFetcher {
    pub fn new() -> Self {
        MusixMatchLyricsFetcher{}
    }

    fn get_song_uri(&self, song: &SongDescriptor) -> Result<String, String> {
        match &song.uri {
            // If we have the uri available, just use it directly.
            Some(SongUri::MusixMatchUri(uri)) => Ok(uri.clone()),

            // Otherwise, we need to derive it from a search.
            _ => {
                let search_uri = format!("{}/{} {}", MUSIX_MATCH_SEARCH_URI, song.name, song.artist);

                let search_result = reqwest::blocking::get(&search_uri)
                    .unwrap()
                    .text()
                    .map(|result| Html::parse_document(&result))
                    .unwrap();

                let result_url_meta_selector = Selector::parse(MUSIX_MATCH_SEARCH_TRACK_URI_SELECTOR)
                    .unwrap();

                let result_url = search_result
                    .select(&result_url_meta_selector)
                    .next()
                    .map(|element_ref| element_ref.value())
                    .unwrap()
                    .attr("content")
                    .map(|uri| format!("{}{}", MUSIX_MATCH_URI, uri))
                    .unwrap();

                Ok(result_url)
            }
        }
    }
}

impl LyricsFetcher for MusixMatchLyricsFetcher {
    fn fetch_lyrics(&self, song: &SongDescriptor) -> Result<String, String> {
        let uri = self.get_song_uri(song)?;

        let content = reqwest::blocking::get(&uri)
            .unwrap()
            .text()
            .map(|html| Html::parse_document(&html))
            .unwrap();

        let segments_selector = Selector::parse(MUSIX_MATCH_LYRICS_SEGMENT_SELECTOR)
            .unwrap();

        let lyrics = content.select(&segments_selector)
            .map(|element_ref| element_ref.inner_html())
            .fold(None, |acc, lyrics_segment| match acc {
                None => Some(lyrics_segment),
                Some(acc) => Some(format!("{}\n{}", acc, lyrics_segment)) 
            })
            .unwrap();

        Ok(lyrics.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    pub fn integration_can_fetch_music_match_song_uri_without_explicit_uri() {
        let result = MusixMatchLyricsFetcher::new().get_song_uri(&SongDescriptor{
            name: "House of fire".to_string(),
            artist: "Dave Rodgers".to_string(),
            uri: None
        });

        assert_eq!(result, Ok("https://www.musixmatch.com/lyrics/Dave-Rodgers/The-House-of-Fire".to_string()));
    }

    #[test]
    #[ignore]
    pub fn integration_can_fetch_music_match_song_with_explicit_uri() {
        let result = MusixMatchLyricsFetcher::new().fetch_lyrics(&SongDescriptor{
            name: String::new(),
            artist: String::new(),
            uri: Some(SongUri::MusixMatchUri("https://www.musixmatch.com/lyrics/Dave-Rodgers/The-House-of-Fire".to_string()))
        });

        assert_eq!(result, Ok("
I'm gonna play the real life
I'm gonna do my best to survive
Really the best
To keep the fire into my heart

'Cause I wanna be free in the sky tonight
I don't wanna hear from you all more lies
I know I will be your star
When we'll reach to the sky
I fly now

Welcome to the house of fire
Let me go - let me go
I just wanna let you go
Take me to the house of fire
Wanna spend all my life time

Welcome to the house of fire
Let me go - let me go
I just wanna let you go
Take me to the house of fire
Wanna spend all my life time

The satellite is on me now
Now, I can see the rays of fire
Stay by my side
Until the end of your silence

'Cause I wanna be free in the sky tonight
I don't wanna hear from you all more lies
I know I will be your star
When we'll reach to the sky
I fly now

Welcome to the house of fire
Let me go - let me go
I just wanna let you go
Take me to the house of fire
Wanna spend all my life time

Welcome to the house of fire
Let me go - let me go
I just wanna let you go
Take me to the house of fire
Wanna spend all my life time

The satellite is on me now
Now, I can see the rays of fire
Stay by my side
Until the end of your silence
        ".to_string().trim().to_string()));
    }
}