mod error;
mod mashupper;

pub use self::error::{Error, Result};
pub use reqwest::Client;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let client = Client::new();
    let (url, data) = mashupper::get_initial_track_search(&client).await?;

    println!("{}", url);
    Ok(())
}
