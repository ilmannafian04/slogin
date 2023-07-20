use actix_web::{middleware::Logger, web};

use crate::controller as c;
use crate::handlers as h;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/ping", web::get().to(c::ping))
            .route("/store", web::post().to(c::store))
            .route("/retrieve", web::get().to(c::retrieve))
            .route("/signup", web::post().to(c::sign_up))
            .route("/signin", web::post().to(c::sign_in))
            .wrap(Logger::default()),
    )
    .service(
        web::scope("health")
            .route("/live", web::get().to(c::ping))
            .route("/ready", web::get().to(c::ping)),
    )
    .service(
        web::scope("")
            .route("/signup", web::get().to(h::asset::get_html))
            .route("/signin", web::get().to(h::asset::get_html))
            .route("/{_:.*}", web::get().to(h::asset::get_asset))
            .wrap(Logger::default()),
    );
}
