use super::models::{MashedTrackAsset, TrackAsset, TrackOrigin};
use crate::apis::{
    base::{APIResult, Pagination},
    deezer as d, dictionary as dict,
};
use crate::{Error, Result};
use log::{debug, error, info};
use rand::{random, Rng};
use random_word::{gen_starts_with, Lang};
use tokio::time::{sleep, Duration};

fn random_word() -> String {
    let mut rng = rand::thread_rng();
    loop {
        let letter = rng.gen_range(97..=122) as u8 as char;
        if let Some(query) = gen_starts_with(letter, Lang::En) {
            return query.to_owned();
        }
    }
}

struct TrackSearch {
    word: String,
    result: APIResult<d::DeezerPaginationResponse<d::TrackList>>,
}

async fn random_track_search() -> Result<TrackSearch> {
    info!("Conducting random track search...");
    let mut attempt = 1;
    let num_retries = 10;

    loop {
        debug!("Attempt {}...", attempt);
        let word = random_word();
        debug!("Querying '{}'", word);
        match d::search_tracks(&word).await {
            Ok(result) => {
                if result.response.total > 0 {
                    info!(
                        "Found {} results for query '{}'",
                        result.response.total, word
                    );
                    return Ok(TrackSearch { word, result });
                }
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

fn find_index_with_preview(tracks: &d::TrackList, start_index: &usize) -> Result<usize> {
    if *start_index >= tracks.len() {
        return Err(Error::IndexError {
            index: start_index.to_owned(),
            length: tracks.len(),
        });
    }
    for (i, track) in tracks[*start_index..].iter().enumerate() {
        if !track.preview_url.is_empty() {
            debug!(
                "Found track '{}' positions from start index '{}'",
                i, start_index
            );
            return Ok(i);
        }
    }
    Err(Error::CriticalError("No track with preview".into()))
}

struct RandomTrack {
    index: u64,
    track: d::Track,
}

async fn pick_random_track(track_search: &TrackSearch) -> Result<RandomTrack> {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..track_search.result.response.total);
    let mut page = track_search.result.get_page_from_index(&index).await?;
    let start_index_of_page = index % track_search.result.response.page_limit();
    let index_on_page =
        find_index_with_preview(&page.response.data, &(start_index_of_page as usize))?;

    if index_on_page >= page.response.data.len() {
        return Err(Error::IndexError {
            index: index_on_page,
            length: page.response.data.len(),
        });
    }

    let track = page.response.data.swap_remove(index_on_page);
    let true_index = index + index_on_page as u64 - start_index_of_page;

    Ok(RandomTrack {
        index: true_index,
        track,
    })
}

async fn lookup_dictionary_entry(word: &str) -> dict::Word {
    if let Ok(mut res) = dict::search_dictionary(word).await {
        if !res.response.is_empty() {
            return res.response.swap_remove(0);
        }
    }
    dict::Word::unknown(word.to_string())
}

pub async fn build_track_asset() -> Result<TrackAsset> {
    let track_search = random_track_search().await?;
    let total_tracks = track_search.result.response.total;
    let word = lookup_dictionary_entry(&track_search.word).await;
    let random_track = pick_random_track(&track_search).await?;
    let preview = d::encoded_preview(&random_track.track.preview_url).await?;

    Ok(TrackAsset::from_track(
        random_track.track,
        preview,
        TrackOrigin {
            word,
            total_tracks,
            track_index: random_track.index,
        },
    ))
}

fn combine_alternating_words(string1: &str, string2: &str) -> String {
    let (words1, words2): (Vec<&str>, Vec<&str>);

    if random::<bool>() {
        words1 = string1.split_whitespace().collect();
        words2 = string2.split_whitespace().collect();
    } else {
        words1 = string2.split_whitespace().collect();
        words2 = string1.split_whitespace().collect();
    }

    let length_limit: u8 = 30;
    let mut result_length: u8 = 0;

    let mut result = Vec::new();
    let len = words1.len().max(words2.len());

    for i in 0..len {
        if i < words1.len() {
            let word = words1[i];
            result.push(word);
            result_length += word.len() as u8;
            if result_length > length_limit {
                break;
            }
        }
        if i < words2.len() {
            let word = words2[i];
            result.push(word);
            result_length += word.len() as u8;
            if result_length > length_limit {
                break;
            }
        }
    }
    result.join(" ")
}

pub fn mash_track_assets(track1: &TrackAsset, track2: &TrackAsset) -> MashedTrackAsset {
    let title = combine_alternating_words(&track1.title, &track2.title);
    let artist = combine_alternating_words(&track1.artist, &track2.artist);
    let album_title = combine_alternating_words(&track1.album_title, &track2.album_title);
    MashedTrackAsset {
        title,
        artist,
        album_title,
    }
}
