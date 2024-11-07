use super::track::get_track;
use crate::Result;

pub async fn retrieve_assets() -> Result<()> {
    let track = get_track().await?;

    println!("{:?}", track);

    let track = get_track().await?;
    println!("{:?}", track);

    Ok(())
}
