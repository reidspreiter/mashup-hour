use crate::apis::{deezer::Track, dictionary::Word};
use serde::{Deserialize, Serialize};

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
    pub preview: String,

    #[serde(rename = "albumTitle")]
    pub album_title: String,

    #[serde(rename = "coverUrl")]
    pub cover_url: String,
    pub origin: TrackOrigin,
}

impl TrackAsset {
    pub fn from_track(track: Track, preview: String, origin: TrackOrigin) -> Self {
        Self {
            id: track.id,
            title: track.title,
            full_title: track.full_title,
            artist: track.artist.name,
            preview,
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
