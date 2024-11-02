use super::apis::deezer::{album_image, Track};
use crate::Result;
use rand::Rng;
use std::fs;
use std::path::{Path, PathBuf};

const OUTPUT_DIRECTORY: &str = "output";

pub async fn download_cover(track: &Track) -> Result<PathBuf> {
    let path =
        Path::new(OUTPUT_DIRECTORY).join(format!("{}.jpg", track.title_short.replace(" ", "-")));
    fs::write(
        &path,
        album_image(&track.album.cover_url).await?.response.as_ref(),
    )?;
    Ok(path)
}

pub async fn create_checkered_image(path1: &PathBuf, path2: &PathBuf) -> Result<()> {
    let img1 = image::open(path1)?.into_rgba8();
    let img2 = image::open(path2)?.into_rgba8();
    let (width, height) = img1.dimensions();
    let mut output_image = image::RgbaImage::new(width, height);

    let mut rng = rand::thread_rng();
    let step_height = height / rng.gen_range(2..=20);
    let step_width = width / rng.gen_range(2..=20);

    for y in (0..height).step_by(step_height as usize) {
        for x in (0..width).step_by(step_width as usize) {
            let use_img1 = (x / step_width + y / step_height) % 2 == 0;

            // Determine the size of the block (may be less than normal at the edges)
            // TODO: make full blocks centered, and remaining edges compensated on all sides
            let block_width = (x + step_width).min(width) - x;
            let block_height = (y + step_height).min(height) - y;

            let source_image = if use_img1 { &img1 } else { &img2 };

            for block_y in 0..block_height {
                for block_x in 0..block_width {
                    let pixel = source_image.get_pixel(x + block_x, y + block_y);
                    output_image.put_pixel(x + block_x, y + block_y, *pixel);
                }
            }
        }
    }
    output_image.save(Path::new(OUTPUT_DIRECTORY).join("output.png"))?;
    Ok(())
}
