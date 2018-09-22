extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate actix;
extern crate actix_web;

#[macro_use]
extern crate diesel;
extern crate futures;
extern crate r2d2;
extern crate r2d2_diesel;

mod db;
mod models;
mod schema;

use actix::prelude::*;
use actix_web::{
    http, server, App, State, Path, HttpResponse, AsyncResponder, FutureResponse,
};

use diesel::prelude::*;
use r2d2_diesel::ConnectionManager;
use futures::Future;
use db::{CreateUser, DbExecutor};

struct AppState {
    db: Addr<DbExecutor>,
}

fn index((name, state): (Path<String>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(CreateUser {
            name: name.into_inner(),
        })
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

fn main() {
    let sys = actix::System::new("actix-web_diesel_sample");

    // Start 3 db executor actors
    let manager = ConnectionManager::<SqliteConnection>::new("test.db");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let addr = SyncArbiter::start(3, move || DbExecutor(pool.clone()));

    // Start http server
    server::new(move || {
        App::with_state(AppState{db: addr.clone()})
            .resource("/{name}", |r| r.method(http::Method::GET).with(index))
    }).bind("127.0.0.1:8080")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
