use actix_web::{middleware::Logger, web, App, HttpServer};
use log::info;
use r2d2_redis::RedisConnectionManager;
use route::routes;

mod config;
mod controller;
mod dto;
mod route;

pub type RedisPool = r2d2::Pool<RedisConnectionManager>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    info!("Starting server");

    let app_config = crate::config::Config::new();
    let config_clone = app_config.clone();

    info!("Connecting to redis");
    let redis_conn_manager =
        RedisConnectionManager::new(app_config.redis_url.clone()).expect("Cannot connect to redis");
    let pool = r2d2::Pool::builder()
        .build(redis_conn_manager)
        .expect("Cannot create connection pool");

    info!(
        "Binding server to {}:{}",
        &app_config.host, &app_config.port
    );
    HttpServer::new(move || {
        App::new()
            .configure(routes)
            .wrap(Logger::default())
            .app_data(config_clone.clone())
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(format!("{}:{}", &app_config.host, &app_config.port))?
    .run()
    .await
}
