use actix_web::web;

use crate::controller as c;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/store", web::post().to(c::store))
            .route("/retrieve", web::get().to(c::retrieve)),
    )
    .service(web::scope("").route("/ping", web::get().to(c::ping)));
}
