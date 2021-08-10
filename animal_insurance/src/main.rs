#[macro_use]
extern crate diesel;

use actix_web::{get, post, App, HttpResponse, HttpServer, Result};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
mod schema;
mod user;
mod utils;
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let pool = utils::establish_connection();
    HttpServer::new(move || App::new().data(pool.clone()).service(test))
        .bind(format!("127.0.0.1:{}", utils::CONFIG.server_port))?
        .run()
        .await
}

#[derive(Debug, Serialize, Deserialize)]
struct AppResponseSuccess {
    result: String,
    message: String,
}

#[get("/test")]
async fn test() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(AppResponseSuccess {
        result: "success".to_string(),
        message: "It works".to_string(),
    }))
}

#[post("/make_score")]
async fn make_score() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(AppResponseSuccess {
        result: "success".to_string(),
        message: "It works".to_string(),
    }))
}
