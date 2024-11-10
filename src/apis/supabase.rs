use super::base::{request, request_with_body, APIResult, RequestMethod};
use super::deezer::Track;
use super::dictionary::Word;
use crate::Result;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::env;

struct SupabaseUtil {
    url: String,
    headers: Vec<(String, String)>,
}

impl SupabaseUtil {
    fn get() -> Result<Self> {
        let url = env::var("SUPABASE_URL")?;
        let key = env::var("SUPABASE_RLS_KEY")?;
        let bearer = format!("Bearer {}", key);
        Ok(Self {
            url,
            headers: vec![
                (String::from("apikey"), key),
                (String::from("Authorization"), bearer),
                (
                    String::from("Content-Type"),
                    String::from("application/json"),
                ),
                (
                    String::from("Prefer"),
                    String::from("return=representation"),
                ),
            ],
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MashupAssetsInsert {
    pub track1: TrackAsset,
    pub track2: TrackAsset,
    pub mashed_track: MashedTrackAsset,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MashupAssets {
    pub id: i8,
    pub created_at: String,
    pub track1: TrackAsset,
    pub track2: TrackAsset,
    pub mashed_track: MashedTrackAsset,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TrackAsset {
    pub title: String,
    pub full_title: String,
    pub artist: String,
    pub preview_url: String,
    pub album_title: String,
    pub cover_url: String,
    pub origin: TrackOrigin,
}

impl TrackAsset {
    pub fn from_track(track: Track, origin: TrackOrigin) -> Self {
        Self {
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
    pub total_tracks: u64,
    pub track_index: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MashedTrackAsset {
    pub title: String,
    pub artist: String,
    pub album_title: String,
}

// TODO: convert supabase calls into a builder pattern and potentially expand to a library replicating the javascript one

pub async fn insert<T>(table_name: &str, row: &T) -> Result<APIResult<Vec<T>>>
where
    T: DeserializeOwned + Serialize,
{
    let util = SupabaseUtil::get()?;
    let url = format!("{}/{}", util.url, table_name);
    request_with_body::<Vec<T>, T>(RequestMethod::POST, &url, Some(&util.headers), row).await
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
    let util = SupabaseUtil::get()?;
    let _columns = columns.unwrap_or("*");
    let mut url = format!("{}/{}?select={}", util.url, table_name, _columns);
    if let Some(o) = order_by {
        url.push_str(&format!("&order={o}"));
    }
    if let Some(l) = limit {
        url.push_str(&format!("&limit={l}"));
    }
    request::<Vec<T>>(RequestMethod::GET, &url, Some(&util.headers)).await
}

pub async fn delete<T>(table_name: &str, criteria: Option<&str>) -> Result<APIResult<Vec<T>>>
where
    T: DeserializeOwned,
{
    let util = SupabaseUtil::get()?;
    let mut url = format!("{}/{}", util.url, table_name);
    if let Some(c) = criteria {
        url.push_str(&format!("?{c}"));
    }
    request::<Vec<T>>(RequestMethod::DELETE, &url, Some(&util.headers)).await
}
