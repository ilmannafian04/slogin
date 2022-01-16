use crate::{
    config::Config,
    dto::{Response, ResponseBuilder, RetrieveQuery, SignInBody, SignUpBody, StoreBody},
    handler,
    middleware::AuthenticationInfo,
    RedisPool,
};
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use log::error;
use r2d2_redis::redis::Commands;
use sea_orm::DatabaseConnection;

pub async fn ping() -> impl Responder {
    HttpResponse::Ok().json(Response {
        message: Some("pong"),
        error: None,
    })
}

pub async fn store(
    pool: web::Data<RedisPool>,
    body: web::Json<StoreBody>,
    req: HttpRequest,
) -> impl Responder {
    let user = match req
        .extensions()
        .get::<AuthenticationInfo>()
        .unwrap()
        .user
        .clone()
    {
        Some(user) => user,
        None => return HttpResponse::Unauthorized().finish(),
    };
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(err) => {
            error!("{}", err);
            let response: Response<()> = ResponseBuilder::new().error(err.to_string()).build();
            return HttpResponse::InternalServerError().json(response);
        }
    };
    match conn.set::<&str, &str, ()>(&format!("{}-{}", user.username, &body.key), &body.value) {
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
    req: HttpRequest,
) -> impl Responder {
    let user = match req
        .extensions()
        .get::<AuthenticationInfo>()
        .unwrap()
        .user
        .clone()
    {
        Some(user) => user,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(err) => {
            error!("{}", err);
            let response: Response<()> = ResponseBuilder::new().error(err.to_string()).build();
            return HttpResponse::InternalServerError().json(response);
        }
    };
    match conn.get::<&str, String>(&format!("{}-{}", user.username, query.key)) {
        Ok(value) => HttpResponse::Ok().json(ResponseBuilder::new().message(value).build()),
        Err(err) => {
            let response: Response<()> = ResponseBuilder::new().error(err.to_string()).build();
            HttpResponse::InternalServerError().json(response)
        }
    }
}

pub async fn sign_up(
    body: web::Json<SignUpBody>,
    db: web::Data<DatabaseConnection>,
) -> impl Responder {
    let id = match handler::register_user(&db, body.into_inner()).await {
        Ok(id) => id,
        Err(err) => {
            error!("{}", err);
            let error: Response<()> = ResponseBuilder::new().error(err.to_string()).build();
            return HttpResponse::BadRequest().json(error);
        }
    };
    HttpResponse::Ok().json(ResponseBuilder::new().message(id).build())
}

pub async fn sign_in(
    body: web::Json<SignInBody>,
    db: web::Data<DatabaseConnection>,
    config: web::Data<Config>,
) -> impl Responder {
    match handler::authenticate_user(&db, body.into_inner(), &config.secret_key).await {
        Some(jwt) => {
            info!("Successfully signed in");
            HttpResponse::Ok().json(ResponseBuilder::new().message(jwt).build())
        },
        None => {
            info!("Failed to sign in");
            let error: Response<()> = ResponseBuilder::new().build();
            HttpResponse::BadRequest().json(error)
        }
    }
}
