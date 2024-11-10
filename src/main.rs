mod apis;
mod assets;
mod error;

pub use self::error::{Error, Result};

use actix_files::Files;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result as ActixResult};
use log::error;

use assets::manager::refresh_assets;

#[get("/collect-mashup-assets")]
async fn collect_mashup_assets() -> ActixResult<impl Responder> {
    Ok(web::Json("Collected assets"))
}

#[post("/trigger_refresh")]
async fn trigger_refresh() -> ActixResult<impl Responder> {
    match refresh_assets().await {
        Ok(_) => Ok(HttpResponse::Ok().json("Assets refreshed successfully")),
        Err(e) => {
            error!("Error refreshing assets: {e:?}");
            Ok(HttpResponse::InternalServerError().json("Encountered error refreshing assets"))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv::dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .service(collect_mashup_assets)
            .service(trigger_refresh)
            .service(Files::new("/", "./mashup-hour-frontend/dist").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
