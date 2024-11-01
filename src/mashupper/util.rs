use super::apis::base::{APIResult, Pagination};
use super::apis::deezer::{find_track, search_tracks, DeezerPaginationResponse, Track, TrackList};
use crate::{Error, Result};
use log::{debug, error, info, warn};
use rand::Rng;
use random_word::{gen_starts_with, Lang};
use tokio::time::{sleep, Duration};

fn get_random_query() -> String {
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        let letter = rng.gen_range(97..=122) as u8 as char;
        debug!("Letter: {}", letter);
        if let Some(query) = gen_starts_with(letter, Lang::En) {
            return query.to_owned();
        }
    }
    debug!("No word found");
    let letter1 = rng.gen_range(97..=122) as u8 as char;
    let letter2 = rng.gen_range(97..=122) as u8 as char;
    format!("{}{}", letter1, letter2)
}

async fn initial_track_search() -> Result<APIResult<DeezerPaginationResponse<TrackList>>> {
    info!("<< Initial Track Search >>");
    let num_retries: u8 = 3;
    let mut attempt = 1;

    loop {
        info!("Attempt {}...", attempt);
        let query = get_random_query();
        debug!("Querying '{}'", query);
        match search_tracks(&query).await {
            Ok(result) => {
                info!(
                    "Found {} results for query '{}'",
                    result.response.total, query
                );
                return Ok(result);
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

fn get_random_song_index(total_songs: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..total_songs)
}

fn get_track_id_with_preview(tracks: &TrackList, start_index: usize) -> Result<Option<u64>> {
    let start_index = start_index as usize;
    if start_index >= tracks.len() {
        return Err(Error::IndexError {
            index: start_index,
            length: tracks.len(),
        });
    }
    for (i, track) in tracks[start_index..].iter().enumerate() {
        if !track.preview_url.is_empty() {
            debug!(
                "Found track '{}' positions from start index '{}'",
                i, start_index
            );
            return Ok(Some(track.id));
        }
    }
    Ok(None)
}

pub async fn get_track() -> Result<Track> {
    info!("<< Get Track >>");
    let track_search = initial_track_search().await?;
    let index = get_random_song_index(track_search.response.total);
    debug!("Searching index '{}'", index);

    let page = track_search.get_page_from_index(&index).await?;
    let start_index = index % track_search.response.page_limit();
    let track_id = match get_track_id_with_preview(&page.response.data, start_index as usize)? {
        Some(id) => id,
        None => {
            warn!("Did not find track with preview url on first page");
            let next_page = page.next_page().await?;
            match get_track_id_with_preview(&next_page.response.data, 0)? {
                Some(id) => id,
                None => {
                    return Err(Error::CriticalError(
                        "Could not locate track with preview".into(),
                    ))
                }
            }
        }
    };
    info!("Fetching track with id '{}'", track_id);
    Ok(find_track(&track_id).await?.response)
}
