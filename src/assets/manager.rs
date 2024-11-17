use super::track::{build_track_asset, mash_track_assets};
use crate::{
    apis::supabase::{self as sb, MashupAssets},
    Error, Result,
};
use actix_web::web::Data;
use log::{info, warn};
use redis::{aio::MultiplexedConnection, AsyncCommands, Client, FromRedisValue};
use std::{cmp, sync::Arc};

const TRACK_LIMIT: u8 = 3;

async fn insert_new_asset_row() -> Result<()> {
    let track1 = build_track_asset().await?;
    let track2 = build_track_asset().await?;
    let mashed_track = mash_track_assets(&track1, &track2);
    let inserted = sb::insert(
        "mashup_assets",
        &sb::MashupAssetsInsert {
            track1,
            track2,
            mashed_track,
        },
    )
    .await?;
    info!("Inserted: {:?}", inserted.response);
    Ok(())
}

fn get_delete_criteria(assets: &Vec<sb::MashupAssets>) -> String {
    let mut criteria = String::from("id=not.in.(");

    for (i, mashup_assets) in assets.iter().enumerate() {
        if i > 0 {
            criteria.push(',');
        }
        criteria.push_str(&mashup_assets.id.to_string());
    }
    criteria.push(')');
    criteria
}

async fn select_assets_from_database() -> Result<Vec<sb::MashupAssets>> {
    let assets = sb::select::<sb::MashupAssets>(
        "mashup_assets",
        Some("*"),
        Some("createdAt.desc"),
        Some(&(TRACK_LIMIT as u64)),
    )
    .await?;
    info!("Selected: {:?}", assets.response);
    Ok(assets.response)
}

fn chunk_string(s: &str, chunk_size: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut start = 0;

    while start < s.len() {
        let end = cmp::min(start + chunk_size, s.len());
        let chunk = &s[start..end];
        chunks.push(chunk.to_string());
        start = end;
    }
    chunks
}

async fn refresh_assets_cache(
    assets: &Vec<sb::MashupAssets>,
    conn: &mut MultiplexedConnection,
) -> Result<()> {
    let chunk_size: usize = 786_423;
    let expiration = 14_000;
    let json = serde_json::to_string(&assets)?;
    let chunks = chunk_string(&json, chunk_size);
    let _: () = conn
        .set_ex("total_chunks", chunks.len() as u8, expiration)
        .await?;

    for (i, chunk) in chunks.iter().enumerate() {
        let key = format!("chunk{}", i + 1);
        let _: () = conn.set_ex(key, chunk, expiration).await?;
    }

    info!("Refreshed assets in cache");
    Ok(())
}

pub async fn refresh_assets(client: &Data<Arc<Client>>) -> Result<()> {
    insert_new_asset_row().await?;
    let assets = select_assets_from_database().await?;

    let mut conn = client.get_multiplexed_tokio_connection().await?;
    refresh_assets_cache(&assets, &mut conn).await?;

    let delete_criteria = get_delete_criteria(&assets);
    let deleted = sb::delete::<sb::MashupAssets>("mashup_assets", Some(&delete_criteria)).await?;
    info!("Deleted: {:?}", deleted.response);
    Ok(())
}

async fn get_cached_value<T>(conn: &mut MultiplexedConnection, key: &str) -> Result<T>
where
    T: FromRedisValue,
{
    match conn.get::<_, Option<T>>(key).await? {
        Some(val) => Ok(val),
        None => {
            warn!("Cache miss for key: {}", key);
            let db_assets = select_assets_from_database().await?;
            refresh_assets_cache(&db_assets, conn).await?;

            // The cache was refreshed, so this should result in a value
            if let Some(val) = conn.get::<_, Option<T>>(&key).await? {
                Ok(val)
            } else {
                Err(Error::CriticalError(
                    "Unable to retrieve assets from cache".into(),
                ))
            }
        }
    }
}

pub async fn retrieve_assets(client: &Data<Arc<Client>>) -> Result<Vec<MashupAssets>> {
    let mut conn = client.get_multiplexed_tokio_connection().await?;
    let total_chunks = get_cached_value::<u8>(&mut conn, "total_chunks").await?;

    let mut chunks: Vec<String> = Vec::new();

    for i in 1..=total_chunks {
        let key = format!("chunk{i}");
        let chunk = get_cached_value::<String>(&mut conn, &key).await?;
        chunks.push(chunk);
    }

    let combined_chunks = chunks.join("");
    let assets: Vec<MashupAssets> = serde_json::from_str(&combined_chunks)?;
    info!("Retrieved assets from cache");
    Ok(assets)
}
