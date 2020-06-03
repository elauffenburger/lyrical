extern crate serde;
extern crate tokio;
extern crate warp;

use std::net::SocketAddr;

use serde::{Deserialize, Serialize};
use warp::Filter;

const SERVER_ADDR: &'static str = "127.0.0.1:8081";

#[tokio::main]
async fn main() {
    // GET /
    // Provides app.
    let ui = warp::get()
        .and(warp::fs::dir("www/static"));

    // GET /buildz
    // Displays build info.
    let build_info = warp::path!("buildz")
        .and(warp::get())
        .map(|| warp::http::Response::builder()
            .header("content-type", "application/json; charset=utf-8")
            .body(include_str!("../build_info.json")));

    // Build all routes.
    let routes = ui
        .or(build_info);

    println!("starting lyrical-fe on {}", SERVER_ADDR);

    warp::serve(routes)
        .run(SERVER_ADDR.parse::<SocketAddr>().unwrap())
        .await;
}
