use std::env::var;

use log::info;

#[derive(Clone)]
pub struct Config {
    pub secret_key: String,
    pub host: String,
    pub port: u16,
    pub redis_url: String,
    pub database_url: String,
}

impl Config {
    pub fn new() -> Self {
        info!("Reading environment variables");
        Self {
            secret_key: var("SECRET_KEY").expect("SECRET_KEY is required"),
            host: var("HOST").unwrap_or("127.0.0.1".to_owned()),
            port: var("PORT")
                .unwrap_or("8080".to_owned())
                .parse::<u16>()
                .expect("failed to parse PORT"),
            redis_url: var("REDIS_URL").expect("REDIS_URL is required"),
            database_url: var("DATABASE_URL").expect("DATABASE_URL is required"),
        }
    }
}
