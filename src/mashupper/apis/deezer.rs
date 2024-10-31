use super::base::APIResult;
use crate::{Client, Error, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DeezerResponse<T> {
    pub data: T,
    pub next: String,
    pub total: u64,
}

pub type TrackList = Vec<SearchTrack>;

#[derive(Debug, Deserialize)]
pub struct SearchTrack {
    pub id: u64,
    pub title: String,
    pub album: Album,
    pub artist: Artist,
    #[serde(rename = "link")]
    pub track_url: String,
    #[serde(rename = "preview")]
    pub preview_url: String,
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

pub async fn search_tracks(
    client: &Client,
    query: &str,
) -> Result<APIResult<DeezerResponse<TrackList>>> {
    let url = format!("https://api.deezer.com/search/track?q={}", query);
    let res = client.get(&url).send().await?;

    if res.status().is_success() {
        let text = res.text().await?;
        let response: DeezerResponse<TrackList> = serde_json::from_str(&text)?;
        return Ok(APIResult { url, response });
    }
    Err(Error::ResponseError {
        status_code: res.status().as_u16(),
        message: res.text().await?,
    })
}
