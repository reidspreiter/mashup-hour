mod error;
mod mashupper;
use mashupper::{manager::make_mashup, util::delete_directory_contents};

pub use self::error::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    delete_directory_contents("output")?;
    make_mashup().await?;
    Ok(())
}
