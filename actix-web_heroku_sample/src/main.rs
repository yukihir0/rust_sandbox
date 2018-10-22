extern crate actix_web;

use actix_web::{server, App, HttpRequest, Responder};

fn index(_req: &HttpRequest) -> impl Responder {
    "Hello Actix-web!!"
}

fn main() {
    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind("127.0.0.1:8000")
        .expect("Can not bind to port 8000")
        .run();
}
