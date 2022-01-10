use actix_web::{HttpResponse, Responder};

use crate::dto::Response;

pub async fn ping() -> impl Responder {
    HttpResponse::Ok().json(Response {
        message: "pong",
        error: None,
    })
}
