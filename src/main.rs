extern crate reqwest;
extern crate tokio;
extern crate scraper;

#[macro_use]
extern crate maplit;

mod lyrics;
mod word_count;

use std::collections::HashMap;

fn main() {
    // TODO: make this an input.
    let songs = vec![];

    print_word_counts_for_songs(songs);
}

fn print_word_counts_for_songs(songs: Vec<lyrics::SongDescriptor>) {
    let fetcher = lyrics::make_lyrics_fetcher();

    let results = songs.into_iter()
        .map(|song| {
            let count = get_word_count_for_song(&fetcher, &song);

            (song, count)
        });

    for result in results {
        match &result.1 {
            Ok(count) => println!("song: {:?}\nlyrics: {:?}\n\n", result.0, count),
            Err(err) => println!("error fetching song {:?}: {:?}\n\n", result.0, err)
        }
    }
}

fn get_word_count_for_song(fetcher: &dyn lyrics::LyricsFetcher, song: &lyrics::SongDescriptor) -> Result<HashMap<String, i32>, String> {
    fetcher.fetch_lyrics(song)
        .map(|lyrics| word_count::count_words(lyrics))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_get_word_count_for_song() {
        let songs = vec![
            lyrics::SongDescriptor {
                name: "House of Fire".to_string(),
                artist: "Dave Rodgers".to_string(),
                uri: None
            }
        ];

        print_word_counts_for_songs(songs);

        assert!(false);
    }
}