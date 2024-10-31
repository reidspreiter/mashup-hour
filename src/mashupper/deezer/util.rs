use super::models::DeezerResponse;
use crate::{Client, Error, Result};
use log::{debug, error, info};
use rand::Rng;
use tokio::time::{sleep, Duration};

fn get_random_query() -> String {
    let mut rng = rand::thread_rng();

    // generate a lowercase value and capitalize it 50% of the time
    let letter1: u8 = rng.gen_range(97..=122);
    let letter2: u8 = rng.gen_range(97..=122);
    let query = format!("{}{}", letter1 as char, letter2 as char);

    format!(
        "{}\"{}\"",
        match rng.gen_range(0..=2) {
            0 => "artist:",
            1 => "track:",
            2 => "album:",
            _ => "",
        },
        query
    )
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

async fn search_tracks(client: &Client) -> Result<(String, DeezerResponse)> {
    let query = get_random_query();
    debug!("Searching for '{}'", query);

    let url = format!("https://api.deezer.com/search/track?q={}", query);
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let text = response.text().await?;
        let data: DeezerResponse = serde_json::from_str(&text)?;

        info!("Found {} results at '{}'", data.total, url);
        return Ok((url, data));
    }
    Err(Error::ResponseError {
        status_code: response.status().as_u16(),
        message: response.text().await?,
    })
}

pub async fn get_initial_track_search(client: &Client) -> Result<(String, DeezerResponse)> {
    info!("<< Initial Track Search >>");
    let num_retries: u8 = 3;
    let mut attempt = 1;

    loop {
        info!("Attempt {}...", attempt);
        match search_tracks(client).await {
            Ok(search_results) => {
                return Ok(search_results);
            }
            Err(err) => {
                error!("{}", err);
                attempt += 1;
                if attempt > num_retries {
                    return Err(Error::CriticalError(
                        "Failed to complete initial track search".into(),
                    ));
                }
                sleep(Duration::from_secs(10)).await;
                continue;
            }
        };
    }
}
