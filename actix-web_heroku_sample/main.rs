extern crate actix_web;

use actix_web::{server, App, HttpRequest, Responder};
use std::env;

fn index(_req: &HttpRequest) -> impl Responder {
    "Hello Actix-web!!"
}

fn get_server_port() -> u16 {
    env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8000)
}

fn main() {
    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind(format!("0.0.0.0:{}", get_server_port()))
        .expect("Can not bind to port 8000")
        .run();
}
