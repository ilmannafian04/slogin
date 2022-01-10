use std::env::var;

use log::info;

#[derive(Clone)]
pub struct Config {
    pub host: String,
    pub port: String,
}

impl Config {
    pub fn new() -> Self {
        info!("Reading environment variables");
        let host = var("HOST").unwrap_or("127.0.0.1".to_owned());
        let port = var("PORT").unwrap_or("8080".to_owned());
        Self { host, port }
    }
}
