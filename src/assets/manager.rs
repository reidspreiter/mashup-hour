use super::track::{build_track_asset, mash_track_assets};
use crate::{
    apis::supabase::{self as sb, MashupAssets},
    Result,
};
use actix_web::web::Data;
use log::info;
use redis::{aio::MultiplexedConnection, AsyncCommands, Client};
use std::sync::Arc;

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
        Some("created_at.desc"),
        Some(&3),
    )
    .await?;
    info!("Selected: {:?}", assets.response);
    Ok(assets.response)
}

async fn refresh_assets_cache(
    assets: &Vec<sb::MashupAssets>,
    connection: &mut MultiplexedConnection,
) -> Result<()> {
    let json = serde_json::to_string(&assets)?;
    let _: () = connection.set("mashup_assets", &json).await?;
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

pub async fn retrieve_assets(client: &Data<Arc<Client>>) -> Result<Vec<MashupAssets>> {
    let mut conn = client.get_multiplexed_tokio_connection().await?;
    let cached_assets: Option<String> = conn.get("mashup_assets").await?;
    match cached_assets {
        Some(assets) => {
            info!("Retrieved assets from cache");
            Ok(serde_json::from_str::<Vec<MashupAssets>>(&assets)?)
        }
        None => {
            info!("Cache miss");
            let assets = select_assets_from_database().await?;
            refresh_assets_cache(&assets, &mut conn).await?;
            Ok(assets)
        }
    }
}
