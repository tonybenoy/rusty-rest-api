#[macro_use]
extern crate diesel;

use actix_web::{get, web, App, HttpResponse, HttpServer, Result};
use actix_web_validator::Json;
use env_logger;
use serde::{Deserialize, Serialize};
mod schema;
mod user;
mod utils;
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    let pool = utils::establish_connection();
    let server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(test)
            .service(web::resource("/make_score").route(web::post().to(make_score)))
    })
    .bind(format!("127.0.0.1:{}", utils::CONFIG.server_port))?
    .run();
    println!(
        "Server running at http://127.0.0.1:{}/",
        utils::CONFIG.server_port
    );

    server.await
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
async fn make_score(user_info: Json<user::UserInfo>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(AppResponseSuccess {
        result: user_info.name.to_string(),
        message: "It works".to_string(),
    }))
}
