use super::track::{build_track_asset, mash_track_assets};
use crate::{
    apis::supabase::{self as sb, MashupAssets},
    Result,
};
use log::info;

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

fn get_delete_criteria(assets: &Vec<MashupAssets>) -> String {
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

pub async fn refresh_assets() -> Result<()> {
    insert_new_asset_row().await?;
    let assets = sb::select::<sb::MashupAssets>(
        "mashup_assets",
        Some("*"),
        Some("created_at.desc"),
        Some(&3),
    )
    .await?;
    info!("Selected: {:?}", assets.response);

    let delete_criteria = get_delete_criteria(&assets.response);
    let deleted = sb::delete::<sb::MashupAssets>("mashup_assets", Some(&delete_criteria)).await?;
    info!("Deleted: {:?}", deleted.response);
    Ok(())
}
