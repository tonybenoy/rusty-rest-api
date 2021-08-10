#[macro_use]
extern crate diesel;
use crate::diesel::ExpressionMethods;
use actix_web::{get, web, App, HttpResponse, HttpServer, Result};
use actix_web_validator::Json;
use diesel::insert_into;
use diesel::{QueryDsl, RunQueryDsl};
use env_logger;
use serde::{Deserialize, Serialize};
mod schema;
mod user;
mod utils;
use schema::user_profile::dsl::*;
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

async fn make_score(
    db: web::Data<utils::Pool>,
    user_info: Json<user::UserInfo>,
) -> Result<HttpResponse> {
    let conn = db.get().unwrap();
    let user_exists = user_profile
        .filter(Aadhar.eq(user_info.Aadhar.to_string()))
        .select(id)
        .first::<i32>(&conn);
    if !(user_exists.is_err()) {
        return Ok(HttpResponse::Ok().json(AppResponseSuccess {
            result: "error".to_string(),
            message: "User already exists".to_string(),
        }));
    }
    let newuser = user::NewUser {
        Aadhar: user_info.Aadhar.to_string(),
        name: user_info.name.to_string(),
        Breed: user_info.cattle.Breed.to_string(),
        Number: user_info.cattle.Number,
        age: user_info.age,
        dependents: user_info.dependents,
        income: user_info.income,
        risk_questions: user_info.risk_questions.clone(),
    };
    let res = insert_into(user_profile).values(&newuser).execute(&conn);
    if res.is_err() {
        return Ok(HttpResponse::Ok().json(AppResponseSuccess {
            result: "error".to_string(),
            message: "Request failed due to internal error".to_string(),
        }));
    }
    Ok(HttpResponse::Ok().json(AppResponseSuccess {
        result: user_info.name.to_string(),
        message: "It works".to_string(),
    }))
}
