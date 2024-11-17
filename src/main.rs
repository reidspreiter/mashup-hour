mod apis;
mod assets;
mod error;

pub use self::error::{Error, Result};

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{
    get, http::header, post, web, App, HttpResponse, HttpServer, Responder, Result as ActixResult,
};
use log::{error, info};
use redis::Client;
use std::{env, sync::Arc};

use assets::manager;

async fn get_redis_connection() -> Result<Client> {
    let instance = env::var("UPSTASH_INSTANCE")?;
    let key = env::var("UPSTASH_KEY")?;
    let port = env::var("UPSTASH_PORT")?;
    Ok(Client::open(format!(
        "rediss://default:{key}@{instance}:{port}"
    ))?)
}

#[get("/retrieve-assets")]
async fn retrieve_assets(redis_client: web::Data<Arc<Client>>) -> ActixResult<impl Responder> {
    info!("retrieving assets...");
    match manager::retrieve_assets(&redis_client).await {
        Ok(assets) => Ok(HttpResponse::Ok().json(assets)),
        Err(e) => {
            error!("Error retrieving assets: {e}");
            Ok(HttpResponse::InternalServerError().json("Encountered error retrieving assets"))
        }
    }
}

#[post("/refresh-assets")]
async fn refresh_assets(redis_client: web::Data<Arc<Client>>) -> ActixResult<impl Responder> {
    info!("refreshing assets...");
    match manager::refresh_assets(&redis_client).await {
        Ok(_) => Ok(HttpResponse::Ok().json("Assets refreshed successfully")),
        Err(e) => {
            error!("Error refreshing assets: {e}");
            Ok(HttpResponse::InternalServerError().json("Encountered error refreshing assets"))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();
    dotenv::dotenv().ok();
    let redis_client = Arc::new(get_redis_connection().await?);

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173")
                    .allowed_methods(vec!["GET"])
                    .allowed_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION])
                    .max_age(3600),
            )
            .app_data(web::Data::new(redis_client.clone()))
            .service(retrieve_assets)
            .service(refresh_assets)
            .service(Files::new("/", "./mashup-hour-frontend/dist").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
