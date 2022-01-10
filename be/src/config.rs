use std::env::var;

use log::info;

#[derive(Clone)]
pub struct Config {
    pub host: String,
    pub port: String,
    pub redis_url: String,
}

impl Config {
    pub fn new() -> Self {
        info!("Reading environment variables");
        Self {
            host: var("HOST").unwrap_or("127.0.0.1".to_owned()),
            port: var("PORT").unwrap_or("8080".to_owned()),
            redis_url: var("REDIS_URL").expect("REDIS_URL is required"),
        }
    }
}
