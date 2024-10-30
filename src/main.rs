use log::{debug, error, info, warn};
use rand::Rng;
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use std::fmt;
use tokio::time::{sleep, Duration};

#[derive(Debug)]
enum SongRequestError {
    CriticalError(String),
}

impl fmt::Display for SongRequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SongRequestError::CriticalError(msg) => write!(f, "Critical Error: {}", msg),
        }
    }
}

impl std::error::Error for SongRequestError {}

#[derive(Debug, Deserialize)]
struct DeezerResponse {
    data: Vec<Track>,
    next: String,
    total: u64,
}

#[derive(Debug, Deserialize)]
struct Track {
    id: u64,
    title: String,
    album: Album,
    artist: Artist,
    #[serde(rename = "link")]
    track_url: String,
    #[serde(rename = "preview")]
    preview_url: String,
}

#[derive(Debug, Deserialize)]
struct Album {
    title: String,
    #[serde(rename = "cover_big")]
    cover_url: String,
}

#[derive(Debug, Deserialize)]
struct Artist {
    name: String,
}

fn get_random_letter() -> char {
    let mut rng = rand::thread_rng();

    // generate a lowercase value and capitalize it 50% of the time
    let mut ascii_value: u8 = rng.gen_range(97..=122);
    if rng.gen_bool(1.0 / 2.0) {
        ascii_value -= 32;
    }

    ascii_value as char
}

fn get_random_song_index(total_songs: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..total_songs)
}

fn get_pagination_url(url: &str, desired_index: &u64) -> String {
    let pagination_limit = 25;
    let pagination_index = desired_index / pagination_limit;
    format!("{url}&index={pagination_index}")
}

async fn get_initial_track_search(client: &Client) -> Result<(String, DeezerResponse), String> {
    info!("<< Initial Track Search >>");
    let num_retries: u8 = 3;

    for i in 0..num_retries {
        info!("Attempt {}...", i + 1);

        let query = get_random_letter();
        debug!("Searching for '{}'", query);

        let url = format!("https://api.deezer.com/search/track?q={}", query);
        let response = match client.get(&url).send().await {
            Ok(res) => res,
            Err(err) => {
                error!("{}", err);
                continue;
            }
        };

        if response.status().is_success() {
            let text = match response.text().await {
                Ok(text) => text,
                Err(err) => {
                    error!("{}", err);
                    continue;
                }
            };

            let data: DeezerResponse = match serde_json::from_str(&text) {
                Ok(data) => data,
                Err(err) => {
                    error!("{}", err);
                    continue; // Skip to the next iteration
                }
            };

            info!("Found {} results at '{}'", data.total, url);
            return Ok((url, data));
        } else {
            let err = response
                .text()
                .await
                .unwrap_or("Failed to read response error message".to_string());
            error!("{}", err);
        }
        sleep(Duration::from_secs(10)).await;
    }
    Err("Failed to get initial search results".to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let client = Client::new();
    let (url, data) = get_initial_track_search(&client).await.unwrap();

    println!("{}", url);
    Ok(())
}
