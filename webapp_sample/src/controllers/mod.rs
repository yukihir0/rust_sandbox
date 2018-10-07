pub mod root_controller;
pub mod users_controller;
pub mod sessions_controller;

use std::sync::Arc;

use handlebars::{Handlebars};
use serde_json::value::{Map, Value};

use actix_web::{HttpResponse};
use actix_web::http::{StatusCode};

pub fn render(templates: Arc<Handlebars>, name: &str, data: Option<Map<String, Value>>) -> HttpResponse {
    let params = match data {
        Some(d) => d,
        None    => Map::new(),
    };

    match templates.render(name, &params) {
        Ok(body) => HttpResponse::Ok().body(body),
        Err(_)   => HttpResponse::InternalServerError().finish(),
    }
}

pub fn http_redirect(path: &str, code: u16) -> HttpResponse {
    let status = StatusCode::from_u16(code)
        .expect("invalide status given");

    HttpResponse::build(status)
        .header("Location", path)
        .finish()
}

pub fn http_status(code: u16) -> HttpResponse {
    let status = StatusCode::from_u16(code)
        .expect("invalide status given");
    
    HttpResponse::build(status)
        .finish()
}

pub fn http_internal_server_error() -> HttpResponse {
    http_status(500)
}
