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

    let app_config = web::Data::new(crate::config::Config::new());

    info!("Connecting to redis");
    let redis_conn_manager =
        RedisConnectionManager::new(app_config.redis_url.clone()).expect("Cannot connect to redis");
    let redis_pool = web::Data::new(
        r2d2::Pool::builder()
            .build(redis_conn_manager)
            .expect("Cannot create connection pool"),
    );

    info!("Connecting to database");
    let db = web::Data::new(
        Database::connect(&app_config.database_url)
            .await
            .expect("Cannot connect to database"),
    );

    info!("Starting database migration");
    if let Err(err) = Migrator::up(db.as_ref(), None).await {
        error!("Failed to run db migration: {}", err);
        exit(1)
    }

    let bind_address = (app_config.host.clone(), app_config.port);
    info!("Binding server to {}:{}", bind_address.0, bind_address.1);
    HttpServer::new(move || {
        App::new()
            .configure(routes)
            .wrap(AuthMiddlewareFactory {})
            .app_data(app_config.clone())
            .app_data(redis_pool.clone())
            .app_data(db.clone())
    })
    .bind(bind_address)?
    .run()
    .await
}
