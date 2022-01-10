use actix_web::{middleware::Logger, App, HttpServer};
use log::info;
use route::routes;

mod config;
mod controller;
mod dto;
mod route;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    info!("Starting server");

    let app_config = crate::config::Config::new();
    let config_clone = app_config.clone();

    info!(
        "Binding server to {}:{}",
        &app_config.host, &app_config.port
    );
    HttpServer::new(move || {
        App::new()
            .configure(routes)
            .wrap(Logger::default())
            .app_data(config_clone.clone())
    })
    .bind(format!("{}:{}", &app_config.host, &app_config.port))?
    .run()
    .await
}
