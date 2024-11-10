mod apis;
mod assets;
mod error;

pub use self::error::{Error, Result};

use actix_files::Files;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result as ActixResult};
use log::error;
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
    env_logger::init();
    dotenv::dotenv().ok();
    let redis_client = Arc::new(get_redis_connection().await?);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(redis_client.clone()))
            .service(retrieve_assets)
            .service(refresh_assets)
            .service(Files::new("/", "./mashup-hour-frontend/dist").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
