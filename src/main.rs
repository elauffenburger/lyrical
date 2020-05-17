extern crate clap;
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate maplit;
extern crate regex;
extern crate reqwest;
extern crate scraper;
extern crate serde;
extern crate tokio;

mod lyrics;
mod utils;
#[cfg(test)]
mod tests;
mod word_count;

use std::fs::File;
use std::io::Read;

use clap::{Arg, App, ArgMatches};

use lyrics::{LyricsFetcher, SongDescriptor};
use word_count::WordCounts;

type WordCountsResult = Result<WordCounts, String>;
type SongWordCountsResult<'a> = (&'a SongDescriptor, WordCountsResult);

fn main() {
    let matches = App::new("Lyrics")
        .version("0.1")
        .author("Eric Lauffenburger <elauffenburger@gmail.com>")
        .arg(Arg::with_name("json_file")
            .short("f")
            .long("json-file")
            .value_name("JSON_FILE")
            .takes_value(true)
            .conflicts_with("json")
            .help("Sets the json file to use as input"))
        .arg(Arg::with_name("json")
            .short("j")
            .long("json")
            .value_name("JSON")
            .conflicts_with("json_file")
            .help("Sets the json to use as input"))
        .get_matches();

    let songs = get_songs_to_fetch(&matches).unwrap();

    let mut fetcher = lyrics::make_lyrics_fetcher();
    let word_counts = get_word_counts_for_songs(&mut fetcher, &songs);

    print_word_counts_for_songs(&word_counts);
}

/// Extracts a list of [SongDescriptor]s from args provided in [matches].
fn get_songs_to_fetch(matches: &ArgMatches) -> Result<Vec<SongDescriptor>, String> {
    let json = matches.value_of("json")
        .map(|json| json.to_string())
        .or_else(|| 
            matches.value_of("json_file")
                .map(|json_file_path|
                    File::open(json_file_path)
                        .map(|mut file| {
                            let mut json = String::new();
                            file.read_to_string(&mut json).unwrap();

                            json
                        })
                        .unwrap()
                )
        )
        .unwrap();

    Ok(serde_json::from_str::<Vec<SongDescriptor>>(&json).unwrap())
}

/// Prints word count results in [word_counts] to stdout.
fn print_word_counts_for_songs(word_counts: &Vec<SongWordCountsResult>) {
    for result in word_counts {
        match &result.1 {
            Ok(count) => println!("song: {:?}\nlyrics: {:?}\n\n", result.0, count),
            Err(err) => println!("error fetching song {:?}: {:?}\n\n", result.0, err)
        }
    }
}

/// Gets a [WordCountsResult] for [song] using [fetcher] to fetch lyrics.
fn get_word_count_for_song<'a, 'b>(fetcher: &'a mut dyn LyricsFetcher, song: &'b SongDescriptor) -> WordCountsResult {
    fetcher.fetch_lyrics(song)
        .map(|lyrics| word_count::count_words(lyrics))
}

/// Gets a list of [SongWordCountsResult] for [songs] using [fetcher] to fetch lyrics.
fn get_word_counts_for_songs<'a, 'b>(fetcher: &'a mut dyn LyricsFetcher, songs: &'b Vec<SongDescriptor>) -> Vec<SongWordCountsResult<'b>> {
    songs.iter()
        .map(|song| {
            let count = get_word_count_for_song(fetcher, song);

            (song, count)
        })
        .collect()
}
