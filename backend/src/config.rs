use dotenvy::dotenv;
use std::env;

pub struct Config {

    pub database_url: String,
    pub ml_service_url: String,
}

impl Config {

    pub fn load() -> Self {

        dotenv().ok();

        let database_url =
            env::var("DATABASE_URL").unwrap_or("postgres://localhost".to_string());

        let ml_service_url =
            env::var("ML_SERVICE_URL").unwrap_or("http://localhost:8000".to_string());

        Self {
            database_url,
            ml_service_url,
        }
    }
}
