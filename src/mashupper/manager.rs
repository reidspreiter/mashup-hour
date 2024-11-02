use super::cover::{create_checkered_image, download_cover};
use super::track::get_track;
use crate::Result;

pub async fn make_mashup() -> Result<()> {
    let track = get_track().await?;
    let path1 = download_cover(&track).await?;
    println!("{:?}", track);

    let track = get_track().await?;
    let path2 = download_cover(&track).await?;
    println!("{:?}", track);

    create_checkered_image(&path1, &path2).await?;

    Ok(())
}
