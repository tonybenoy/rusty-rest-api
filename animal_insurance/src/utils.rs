use crate::user::UserInfo;
use actix_web_validator::Json;
use config::ConfigError;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use dotenv::dotenv;
use lazy_static::lazy_static;
use serde::Deserialize;
use strum_macros::Display;
#[derive(Deserialize, Debug)]
pub struct Config {
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
    pub db_port: i32,
    pub db_url: String,
    pub server_port: i32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            db_user: "postgres".to_string(),
            db_password: "postgres".to_string(),
            db_name: "insurance".to_string(),
            db_port: 5432,
            db_url: "localhost".to_string(),
            server_port: 8000,
        }
    }
}

impl Config {
    pub fn get_env_info() -> Result<Self, ConfigError> {
        let mut config = config::Config::new();
        config.merge(config::Environment::new())?;
        config.try_into()
    }
}

lazy_static! {
    pub static ref CONFIG: Config = {
        dotenv().ok();
        Config::get_env_info().unwrap()
    };
}

pub type Pool = diesel::r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> Pool {
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        CONFIG.db_user, CONFIG.db_password, CONFIG.db_url, CONFIG.db_port, CONFIG.db_name
    );
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
#[derive(Debug, Display)]
pub enum Plans {
    BachatPlus,
    BimaBachat,
    AadharShila,
    Ineligibile,
}

pub fn get_recommendation(uinfo: Json<UserInfo>) -> Plans {
    let mut total_score: i32 = uinfo.risk_questions.iter().sum();
    if uinfo.age > 60 || uinfo.income <= 0 || uinfo.get_num_if_available() <= 0 {
        return Plans::Ineligibile;
    }
    if uinfo.age < 30 {
        total_score = total_score - 2;
    }
    if uinfo.age >= 30 && uinfo.age < 40 {
        total_score = total_score - 1;
    }
    if uinfo.income >= 100000 {
        total_score = total_score - 1;
    }
    if uinfo.dependents >= 1 {
        total_score = total_score + 1;
    }
    match total_score {
        d if d <= 0 => return Plans::BachatPlus,
        d if d >= 3 => Plans::AadharShila,
        _ => Plans::BimaBachat,
    }
}
