mod apis;
mod assets;
mod error;

pub use self::error::{Error, Result};

use actix_files::Files;
use actix_web::{post, web, App, HttpServer, Responder, Result as ActixResult};

#[post("/collect-mashup-assets")]
async fn collect_mashup_assets() -> ActixResult<impl Responder> {
    Ok(web::Json("Collected assets"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .service(collect_mashup_assets)
            .service(Files::new("/", "./mashup-hour-frontend/dist").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
