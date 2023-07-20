use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::services;

pub async fn get_html(req: HttpRequest) -> impl Responder {
    let path = &req.uri().to_string()[1..];
    let filename = if &path == &"" {
        "index.html".to_owned()
    } else {
        format!("{}.html", &path)
    };
    match services::asset::AssetService::get_text_file(&filename) {
        Ok(asset) => HttpResponse::Ok()
            .content_type(
                mime_guess::from_path(&filename)
                    .first_or_octet_stream()
                    .as_ref(),
            )
            .body(asset),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn get_asset(path: web::Path<String>) -> impl Responder {
    let filename = if &path.to_string() == &"" {
        "index.html"
    } else {
        &path
    };
    if let Ok(asset) = services::asset::AssetService::get_text_file(&filename) {
        return HttpResponse::Ok()
            .content_type(
                mime_guess::from_path(filename)
                    .first_or_octet_stream()
                    .as_ref(),
            )
            .body(asset);
    };

    match services::asset::AssetService::get_raw(&filename) {
        Ok(data) => HttpResponse::Ok()
            .content_type(
                mime_guess::from_path(filename)
                    .first_or_octet_stream()
                    .as_ref(),
            )
            .body(data),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
