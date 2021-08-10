use config::ConfigError;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use dotenv::dotenv;
use lazy_static::lazy_static;
use serde::Deserialize;
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
