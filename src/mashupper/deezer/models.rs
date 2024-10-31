use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DeezerResponse {
    pub data: Vec<Track>,
    pub next: String,
    pub total: u64,
}

#[derive(Debug, Deserialize)]
struct Track {
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
struct Album {
    pub title: String,
    #[serde(rename = "cover_big")]
    pub cover_url: String,
}

#[derive(Debug, Deserialize)]
struct Artist {
    pub name: String,
}
