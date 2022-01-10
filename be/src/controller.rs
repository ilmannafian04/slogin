use actix_web::{web, HttpResponse, Responder};
use log::error;
use r2d2_redis::redis::Commands;

use crate::{
    dto::{Response, ResponseBuilder, RetrieveQuery, StoreBody},
    RedisPool,
};

pub async fn ping() -> impl Responder {
    HttpResponse::Ok().json(Response {
        message: Some("pong"),
        error: None,
    })
}

pub async fn store(pool: web::Data<RedisPool>, body: web::Json<StoreBody>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(err) => {
            error!("{}", err);
            let response: Response<()> = ResponseBuilder::new().error(err.to_string()).build();
            return HttpResponse::InternalServerError().json(response);
        }
    };
    match conn.set::<&str, &str, ()>(&body.key, &body.value) {
        Ok(_) => HttpResponse::Ok().json(ResponseBuilder::new().message(&body.key).build()),
        Err(err) => {
            let response: Response<()> = ResponseBuilder::new().error(err.to_string()).build();
            HttpResponse::InternalServerError().json(response)
        }
    }
}

pub async fn retrieve(
    pool: web::Data<RedisPool>,
    query: web::Query<RetrieveQuery>,
) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(err) => {
            error!("{}", err);
            let response: Response<()> = ResponseBuilder::new().error(err.to_string()).build();
            return HttpResponse::InternalServerError().json(response);
        }
    };
    match conn.get::<&str, String>(&query.key) {
        Ok(value) => HttpResponse::Ok().json(ResponseBuilder::new().message(value).build()),
        Err(err) => {
            let response: Response<()> = ResponseBuilder::new().error(err.to_string()).build();
            HttpResponse::InternalServerError().json(response)
        }
    }
}
