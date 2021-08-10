#[macro_use]
extern crate diesel;
use crate::diesel::ExpressionMethods;
use actix_web::{get, web, App, HttpResponse, HttpServer, Result};
use actix_web_validator::Json;
use diesel::insert_into;
use diesel::{QueryDsl, RunQueryDsl};
use env_logger;
use serde::{Deserialize, Serialize};
use std::string::ToString;
mod models;
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
            .service(get_recommendation)
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
struct AppResponse {
    result: String,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AppResponseId {
    result: String,
    message: String,
    id: i32,
}

#[get("/test")]
async fn test() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(AppResponse {
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
        return Ok(HttpResponse::Ok().json(AppResponseId {
            result: "error".to_string(),
            message: "User already exists".to_string(),
            id: user_exists.unwrap(),
        }));
    }
    if user_info.risk_questions.len() != 3 {
        return Ok(HttpResponse::Ok().json(AppResponse {
            result: "error".to_string(),
            message: "Incorrect number of risk questions".to_string(),
        }));
    }
    let mut bool_vec = vec![];
    for item in &user_info.risk_questions {
        match item {
            1 => bool_vec.push(true),
            0 => bool_vec.push(false),
            _ => {
                return Ok(HttpResponse::Ok().json(AppResponse {
                    result: "error".to_string(),
                    message: "Invalid value in risk questions".to_string(),
                }))
            }
        }
    }
    let newuser = models::NewUser {
        Aadhar: user_info.Aadhar.to_string(),
        name: user_info.name.to_string(),
        Breed: user_info.get_breed_if_available(),
        Number: user_info.get_num_if_available(),
        age: user_info.age,
        dependents: user_info.dependents,
        income: user_info.income,
        risk_questions: bool_vec,
        recommendation: utils::get_recommendation(user_info).to_string(),
    };
    let res: std::result::Result<Vec<i32>, diesel::result::Error> = insert_into(user_profile)
        .values(&newuser)
        .returning(id)
        .get_results(&conn);
    if res.is_err() {
        return Ok(HttpResponse::Ok().json(AppResponse {
            result: "error".to_string(),
            message: "Request failed due to internal error".to_string(),
        }));
    }
    Ok(HttpResponse::Ok().json(AppResponseId {
        result: "success".to_string(),
        message: "Data saved and recommendation is generated".to_string(),
        id: res.unwrap()[0],
    }))
}

#[derive(Debug, Serialize, Deserialize)]
struct AppResponseRecommendation {
    result: String,
    message: String,
    recommendation: String,
    cibil: i32,
}

#[get("/get_recommendation/{user_id}")]
async fn get_recommendation(
    db: web::Data<utils::Pool>,
    web::Path(user_id): web::Path<i32>,
) -> Result<HttpResponse> {
    let conn = db.get().unwrap();
    let user = user_profile.find(user_id).get_result::<models::User>(&conn);
    if user.is_err() {
        return Ok(HttpResponse::Ok().json(AppResponse {
            result: "error".to_string(),
            message: "User not found".to_string(),
        }));
    }
    let user = user.unwrap();
    Ok(HttpResponse::Ok().json(AppResponseRecommendation {
        result: "success".to_string(),
        message: "Score found".to_string(),
        recommendation: user
            .recommendation
            .clone()
            .unwrap_or("No recommendation available".to_string()),
        cibil: user.get_cibil_score(),
    }))
}
