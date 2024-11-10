use super::base::{request, APIResult, Pagination, RequestMethod};
use crate::Result;
use serde::{de::DeserializeOwned, Deserialize};

#[derive(Debug, Deserialize)]
pub struct DeezerPaginationResponse<T> {
    pub data: T,
    pub next: Option<String>,
    pub total: u64,
}

impl<T> Pagination for DeezerPaginationResponse<T>
where
    T: DeserializeOwned,
{
    fn get_pagination_url(&self, url: &str, page_index: &u64) -> String {
        format!("{url}&index={page_index}")
    }

    fn page_limit(&self) -> u64 {
        25
    }

    fn next(&self) -> &Option<String> {
        &self.next
    }
}

pub type TrackList = Vec<Track>;

#[derive(Debug, Deserialize)]
pub struct Track {
    #[serde(rename = "title_short")]
    pub title: String,

    #[serde(rename = "title")]
    pub full_title: String,

    #[serde(rename = "preview")]
    pub preview_url: String,

    pub artist: Artist,
    pub album: Album,
}

#[derive(Debug, Deserialize)]
pub struct Album {
    pub title: String,
    #[serde(rename = "cover_big")]
    pub cover_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Artist {
    pub name: String,
}

pub async fn search_tracks(query: &str) -> Result<APIResult<DeezerPaginationResponse<TrackList>>> {
    let url = format!("https://api.deezer.com/search/track?q={query}");
    request::<DeezerPaginationResponse<TrackList>>(RequestMethod::GET, &url, None).await
}
