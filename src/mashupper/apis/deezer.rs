use super::base::{request, APIResult, Pagination};
use crate::Result;
use serde::{de::DeserializeOwned, Deserialize};

#[derive(Debug, Deserialize)]
pub struct DeezerPaginationResponse<T> {
    pub data: T,
    pub next: String,
    pub total: u64,
}

impl<T> Pagination for DeezerPaginationResponse<T>
where
    T: DeserializeOwned,
{
    fn get_pagination_url(&self, url: &str, page_index: &u64) -> String {
        format!("{}&index={}", url, page_index)
    }

    fn page_limit(&self) -> u64 {
        25
    }

    fn next(&self) -> &String {
        &self.next
    }
}

pub type TrackList = Vec<SearchTrack>;

#[derive(Debug, Deserialize)]
pub struct SearchTrack {
    pub id: u64,
    #[serde(rename = "preview")]
    pub preview_url: String,
    // More fields exist in the response json
}

#[derive(Debug, Deserialize)]
pub struct Album {
    pub title: String,
    #[serde(rename = "cover_big")]
    pub cover_url: String,
    // More fields exist in the response json
}

#[derive(Debug, Deserialize)]
pub struct Artist {
    pub name: String,
    // More fields exist in the response json
}

#[derive(Debug, Deserialize)]
pub struct Track {
    pub id: u64,
    pub title: String,
    pub title_short: String,
    pub bpm: f32,
    pub gain: f32,
    #[serde(rename = "preview")]
    pub preview_url: String,
    pub artist: Artist,
    pub album: Album,
    // More fields exist in the response json
}

pub async fn search_tracks(query: &str) -> Result<APIResult<DeezerPaginationResponse<TrackList>>> {
    let url = format!("https://api.deezer.com/search/track?q={}", query);
    request::<DeezerPaginationResponse<TrackList>>(&url).await
}

pub async fn find_track(track_id: &u64) -> Result<APIResult<Track>> {
    let url = format!("https://api.deezer.com/track/{}", track_id);
    request::<Track>(&url).await
}
