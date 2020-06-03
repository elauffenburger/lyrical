extern crate liblyrical;
extern crate serde;
extern crate tokio;
extern crate warp;

use std::collections::HashMap;
use std::net::SocketAddr;

use liblyrical::lyrics::{LyricsFetcher, SongDescriptor};
use liblyrical::word_count;
use serde::{Deserialize, Serialize};
use warp::Filter;

const SERVER_ADDR: &'static str = "127.0.0.1:8080";

#[derive(Serialize, Deserialize)]
struct GetLyricalFrequencyRequest {
    pub songs: Vec<SongDescriptor>,
}

#[derive(Serialize, Deserialize)]
struct GetLyricalFrequencyResponse {
    pub results: Vec<GetLyricalFrequencyResponseResult>,
}

#[derive(Serialize, Deserialize)]
struct GetLyricalFrequencyResponseResult {
    pub song: SongDescriptor,
    pub frequencies: Option<HashMap<String, i32>>
}

#[tokio::main]
async fn main() {
    // POST /lyrical-frequency
    // Synchronously fetches word frequency for a song.
    let word_frequency_sync = warp::path!("word-frequency-sync")
        .and(warp::post())
        .and(warp::body::json())
        .map(|req: GetLyricalFrequencyRequest| {
            let mut fetcher = liblyrical::lyrics::make_lyrics_fetcher();

            let response = GetLyricalFrequencyResponse {
                results: req.songs.into_iter()
                    .map(|song| {
                        let frequencies = fetcher
                            .fetch_lyrics(&song)
                            .map(|lyrics| Some(word_count::count_words(lyrics)))
                            .unwrap_or(None);

                        GetLyricalFrequencyResponseResult { song, frequencies }
                    })
                    .collect()
            };

            warp::reply::json(&response)
        });

    // GET /buildz
    // Displays build info.
    let build_info = warp::path!("buildz")
        .and(warp::get())
        .map(|| warp::http::Response::builder()
            .header("content-type", "application/json; charset=utf-8")
            .body(include_str!("../build_info.json")));

    // Build all routes.
    let routes = word_frequency_sync
        .or(build_info);

    println!("starting lyrical-api on {}", SERVER_ADDR);

    warp::serve(routes)
        .run(SERVER_ADDR.parse::<SocketAddr>().unwrap())
        .await;
}
