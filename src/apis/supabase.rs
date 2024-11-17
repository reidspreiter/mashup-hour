use super::base::{request_builder, APIResult, ContentType, RequestBuilder, RequestMethod};
use super::deezer::Track;
use super::dictionary::Word;
use crate::Result;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::env;

fn supabase_request_builder(method: RequestMethod, url: &str) -> Result<RequestBuilder> {
    let mut builder = request_builder(method, url);
    let key = env::var("SUPABASE_RLS_KEY")?;
    builder = builder
        .header("apikey", &key)
        .bearer(&key)
        .content_type(ContentType::JSON)
        .header("Prefer", "return=representation");
    Ok(builder)
}

fn get_supabase_url() -> Result<String> {
    Ok(env::var("SUPABASE_URL")?)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MashupAssetsInsert {
    pub track1: TrackAsset,
    pub track2: TrackAsset,

    #[serde(rename = "mashedTrack")]
    pub mashed_track: MashedTrackAsset,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MashupAssets {
    pub id: i8,

    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub track1: TrackAsset,
    pub track2: TrackAsset,

    #[serde(rename = "mashedTrack")]
    pub mashed_track: MashedTrackAsset,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TrackAsset {
    pub id: u64,
    pub title: String,

    #[serde(rename = "fullTitle")]
    pub full_title: String,
    pub artist: String,

    #[serde(rename = "previewUrl")]
    pub preview_url: String,

    #[serde(rename = "albumTitle")]
    pub album_title: String,

    #[serde(rename = "coverUrl")]
    pub cover_url: String,
    pub origin: TrackOrigin,
}

impl TrackAsset {
    pub fn from_track(track: Track, origin: TrackOrigin) -> Self {
        Self {
            id: track.id,
            title: track.title,
            full_title: track.full_title,
            artist: track.artist.name,
            preview_url: track.preview_url,
            album_title: track.album.title,
            cover_url: track.album.cover_url,
            origin,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TrackOrigin {
    pub word: Word,

    #[serde(rename = "totalTracks")]
    pub total_tracks: u64,

    #[serde(rename = "trackIndex")]
    pub track_index: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MashedTrackAsset {
    pub title: String,
    pub artist: String,

    #[serde(rename = "albumTitle")]
    pub album_title: String,
}

// TODO: convert supabase calls into a builder pattern and potentially expand to a library replicating the javascript one

pub async fn insert<T>(table_name: &str, row: &T) -> Result<APIResult<Vec<T>>>
where
    T: DeserializeOwned + Serialize,
{
    let url = format!("{}/{}", get_supabase_url()?, table_name);
    supabase_request_builder(RequestMethod::POST, &url)?
        .json(row)
        .request_model::<Vec<T>>()
        .await
}

pub async fn select<T>(
    table_name: &str,
    columns: Option<&str>,
    order_by: Option<&str>,
    limit: Option<&u64>,
) -> Result<APIResult<Vec<T>>>
where
    T: DeserializeOwned,
{
    let _columns = columns.unwrap_or("*");
    let mut url = format!("{}/{}?select={}", get_supabase_url()?, table_name, _columns);
    if let Some(o) = order_by {
        url.push_str(&format!("&order={o}"));
    }
    if let Some(l) = limit {
        url.push_str(&format!("&limit={l}"));
    }
    supabase_request_builder(RequestMethod::GET, &url)?
        .request_model::<Vec<T>>()
        .await
}

pub async fn delete<T>(table_name: &str, criteria: Option<&str>) -> Result<APIResult<Vec<T>>>
where
    T: DeserializeOwned,
{
    let mut url = format!("{}/{}", get_supabase_url()?, table_name);
    if let Some(c) = criteria {
        url.push_str(&format!("?{c}"));
    }
    supabase_request_builder(RequestMethod::DELETE, &url)?
        .request_model::<Vec<T>>()
        .await
}
