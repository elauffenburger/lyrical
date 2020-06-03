use reqwest;
use scraper::{Html, Selector};

use super::*;

const MUSIX_MATCH_URI: &'static str = "https://www.musixmatch.com";
const MUSIX_MATCH_SEARCH_URI: &'static str = "https://www.musixmatch.com/search";
const MUSIX_MATCH_SEARCH_TRACK_URI_SELECTOR: &'static str = "#search-all-results > .main-panel > .box > .box-content .track-card > meta[itemprop=\"url\"]";
const MUSIX_MATCH_LYRICS_SEGMENT_SELECTOR: &'static str = ".mxm-lyrics__content > span";

const USER_AGENT: &'static str = "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:15.0) Gecko/20100101 Firefox/15.0.1";

#[derive(Builder, Clone, Debug)]
pub struct MusixMatchLyricsFetcherOptions {
    proxy: Option<String>
}

#[derive(Clone, Debug)]
pub struct MusixMatchLyricsFetcher {
    options: MusixMatchLyricsFetcherOptions,
}

impl MusixMatchLyricsFetcher {
    pub fn new(options: MusixMatchLyricsFetcherOptions) -> Self {
        MusixMatchLyricsFetcher{ options }
    }

    fn get_song_uri(&self, song: &SongDescriptor) -> Result<String, String> {
        match &song.uri {
            // If we have the uri available, just use it directly.
            Some(SongUri::MusixMatchUri(uri)) => Ok(uri.clone()),

            // Otherwise, we need to derive it from a search.
            _ => {
                let search_uri = format!("{}/{} {}", MUSIX_MATCH_SEARCH_URI, song.name, song.artist);

                let client = self.make_client();
                let request = client
                    .get(&search_uri)
                    .build()
                    .map_err(|err| format!("Failed to build request: {}", err))
                    .unwrap();

                let search_result_html = client
                    .execute(request)
                    .map_err(|err| format!("Failed to retrieve search content for song \"{:?}\": {}", song, err))
                    .and_then(|resp| {
                        resp.text()
                            .map_err(|err| format!("Failed to extract response body when searching for song \"{:?}\": {}", song, err))
                    })?;
                
                let search_result = Html::parse_document(&search_result_html);

                let result_url_meta_selector = Selector::parse(MUSIX_MATCH_SEARCH_TRACK_URI_SELECTOR)
                    .map_err(|err| format!("Failed to parse selector for musix match search track uri selector for song \"{:?}\": {:?}", song, err))?;

                let uri = search_result
                    .select(&result_url_meta_selector)
                    .next()
                    .and_then(|element_ref| element_ref.value().attr("content"));
                
                match uri {
                    Some(uri) => Ok(format!("{}{}", MUSIX_MATCH_URI, uri)),
                    None => Err(format!("Failed to find a search result for song \"{:?}\" in response: {}", song, search_result_html))
                }
            }
        }
    }

    fn make_client(&self) -> reqwest::blocking::Client {
        let builder = reqwest::blocking::Client::builder()
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(reqwest::header::USER_AGENT, reqwest::header::HeaderValue::from_static(USER_AGENT));

                headers
            });

        let builder = match &self.options.proxy {
            Some(proxy) => builder.proxy(reqwest::Proxy::http(proxy).unwrap()),
            None => builder
        };

        builder
            .build()
            .map_err(|err| format!("Failed to build Client: {}", err))
            .unwrap()
    }
}

impl LyricsFetcher for MusixMatchLyricsFetcher {
    fn fetch_lyrics(&mut self, song: &SongDescriptor) -> Result<String, String> {
        let uri = self.get_song_uri(song)?;

        let client = self.make_client();
        let request = client
            .get(&uri)
            .build()
            .map_err(|err| format!("Failed to build request: {}", err))
            .unwrap();

        let content_html = client.execute(request)
            .and_then(|resp| resp.text())
            .map_err(|err| format!("Failed to extract html from response: {}", err))?;
        
        let content = Html::parse_document(&content_html);

        let segments_selector = Selector::parse(MUSIX_MATCH_LYRICS_SEGMENT_SELECTOR)
            .map_err(|err| format!("failed to parse MusixMatch lyrics segment selector: {:?}", err))?;

        let lyrics = content.select(&segments_selector)
            .map(|element_ref| element_ref.inner_html())
            .fold(None, |acc, lyrics_segment| match acc {
                None => Some(lyrics_segment),
                Some(acc) => Some(format!("{}\n{}", acc, lyrics_segment)) 
            })
            .map(|lyrics| lyrics.to_string());

        match lyrics {
            Some(lyrics) => Ok(lyrics),
            None => Err(format!("Something went unexpectedly wrong while fetching lyrics for song \"{:?}\" with html: {}", song, &content_html))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    pub fn integration_can_fetch_music_match_song_uri_without_explicit_uri() {
        let options = MusixMatchLyricsFetcherOptionsBuilder::default()
            .proxy(None)
            .build()
            .unwrap();

        let result = MusixMatchLyricsFetcher::new(options)
            .get_song_uri(&SongDescriptor{
                name: "House of fire".to_string(),
                artist: "Dave Rodgers".to_string(),
                uri: None
            });

        assert_eq!(result, Ok("https://www.musixmatch.com/lyrics/Dave-Rodgers/The-House-of-Fire".to_string()));
    }

    #[test]
    #[ignore]
    pub fn integration_can_fetch_music_match_song_with_explicit_uri() {
        let options = MusixMatchLyricsFetcherOptionsBuilder::default()
            .proxy(None)
            .build()
            .unwrap();

        let result = MusixMatchLyricsFetcher::new(options)
            .fetch_lyrics(&SongDescriptor{
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