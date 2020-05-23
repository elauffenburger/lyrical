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

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use clap::{Arg, ArgGroup, App, ArgMatches};

use lyrics::{LyricsFetcher, SongDescriptor};
use word_count::WordCounts;

type WordCountsResult = Result<WordCounts, String>;
type SongWordCountsResult<'a> = (&'a SongDescriptor, WordCountsResult);

fn main() {
    let matches = App::new("Lyrical")
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
        .group(ArgGroup::with_name("json_source")
            .args(&["json_file", "json"])
            .required(true))
        .get_matches();

    let songs = get_songs_to_fetch(&matches).unwrap();

    let mut fetcher = lyrics::make_lyrics_fetcher();
    let word_counts = get_word_counts_for_songs(&mut fetcher, &songs);

    print_word_counts_for_songs(word_counts);
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

/// Prints aggregated word count results in [word_counts] to stdout.
fn print_word_counts_for_songs(word_counts: Vec<SongWordCountsResult>) {
    // Record the total number of songs for later.
    let num_songs = word_counts.len() as f32;

    // Split [word_counts] into successes and failures.
    // [successful_word_counts] contains the actual [WordCounts] and [failures]
    // contains the [Result<_, _>].
    let (successful_word_counts, failures) = word_counts
        .into_iter()
        .fold((vec![], vec![]), |mut acc, result| {
            match result.1 {
                Ok(counts) => {
                    acc.0.push(counts);
                },
                _ => acc.1.push(result)
            };

            acc
        });

    let aggregated_word_counts = aggregate_word_counts(successful_word_counts);

    // Collect some metrics for use in reporting failures.
    let num_failures = failures.len() as f32;
    let num_successes = num_songs - num_failures;
    let success_rate = (num_successes / num_songs) * 100f32;

    println!("Fetched {}/{} songs; success rate: {}%", num_successes, num_songs, success_rate);
    println!();

    println!("Failures:");
    for failure in &failures {
        println!("\t{:?}", &failure.0);
    }

    println!("-------");

    // Transform [aggregated_word_counts] into a [Vec] of (&String, i32) sorted
    // by desc value.
    let sorted_kvps = word_count::sort_word_counts(&aggregated_word_counts, word_count::SortOrder::Descending);

    println!("Most Common Words:");
    for kvp in sorted_kvps {
        println!("{}: {}", kvp.0, kvp.1);
    }

    println!("\n\nDone!");
}

/// Aggregates word counts from [word_counts] into a single [WordCounts] result.
fn aggregate_word_counts(word_counts: Vec<WordCounts>) -> WordCounts {
    word_counts.into_iter()
        .fold(HashMap::new(), |mut acc, result| {
            for (word, result_count) in result {
                let old_acc_count = match acc.get(&word) {
                    Some(count) => *count,
                    None => 0
                };

                acc.insert(word, old_acc_count + result_count);
            }

            acc
        })
}

/// Gets a list of [SongWordCountsResult] for [songs] using [fetcher] to fetch lyrics.
fn get_word_counts_for_songs<'a, 'b>(fetcher: &'a mut dyn LyricsFetcher, songs: &'b Vec<SongDescriptor>) -> Vec<SongWordCountsResult<'b>> {
    songs.iter()
        .map(|song| {
            let count = fetcher.fetch_lyrics(song)
                .map(|lyrics| word_count::count_words(lyrics));

            (song, count)
        })
        .collect()
}
