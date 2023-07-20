use std::process::exit;

use actix_web::{web, App, HttpServer};
use log::{error, info};
use migration::{Migrator, MigratorTrait};
use r2d2_redis::RedisConnectionManager;
use route::routes;
use sea_orm::Database;

use crate::middleware::AuthMiddlewareFactory;

mod config;
mod controller;
mod dto;
mod handler;
mod handlers;
mod middleware;
mod route;
mod services;
mod storages;

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

    info!("Connecting to database");
    let db = Database::connect(&app_config.database_url)
        .await
        .expect("Cannot connect to database");

    info!("Starting database migration");
    if let Err(err) = Migrator::up(&db, None).await {
        error!("Failed to run db migration: {}", err);
        exit(1)
    }

    info!(
        "Binding server to {}:{}",
        &app_config.host, &app_config.port
    );
    HttpServer::new(move || {
        App::new()
            .configure(routes)
            .wrap(AuthMiddlewareFactory {})
            .app_data(web::Data::new(config_clone.clone()))
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(db.clone()))
    })
    .bind((app_config.host, app_config.port))?
    .run()
    .await
}
