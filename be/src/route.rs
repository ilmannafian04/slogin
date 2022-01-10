use actix_web::web;

use crate::controller as c;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("").route("/ping", web::get().to(c::ping)));
}
