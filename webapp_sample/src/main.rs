extern crate bcrypt;
extern crate chrono;
extern crate dotenv;
extern crate env_logger;
extern crate failure;
extern crate handlebars;
extern crate log;
extern crate rand;
extern crate regex;
extern crate uuid;

extern crate futures;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate actix;
extern crate actix_web;

#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

mod db;
mod models;
mod schema;
mod context;
mod controllers;
mod helpers;

use std::env;
use std::io::Write;

use chrono::Local;
use dotenv::dotenv;
use log::LevelFilter;

use actix::prelude::*;
use actix_web::{server, fs, App};
use actix_web::http::{Method};
use actix_web::middleware::{Logger};
use actix_web::middleware::session::{SessionStorage, CookieSessionBackend};

use diesel::prelude::*;
use r2d2_diesel::ConnectionManager;

use db::{DbExecutor};
use context::{Context};

fn app(context: Context) -> App<Context> {
    let mut app = App::with_state(context);
   
    app = app.middleware(
        Logger::default()
    );
    
    app = app.middleware(
        SessionStorage::new(
            CookieSessionBackend::signed(&[0; 32])
                .secure(false)
        )
    );

    app = app.handler(
        "/public/css",
        fs::StaticFiles::new("./src/public/css")
        .unwrap()
        .show_files_listing()
    );

    app = app.handler(
        "/public/js",
        fs::StaticFiles::new("./src/public/js")
        .unwrap()
        .show_files_listing()
    );

    app = app.route(
        "/",
        Method::GET,
        controllers::root_controller::handle_index,
    );
    
    app = app.route(
        "/users",
        Method::GET,
        controllers::users_controller::handle_index,
    );

    app = app.route(
        "/users/new",
        Method::GET,
        controllers::users_controller::handle_new,
    );

    app = app.route(
        "/users",
        Method::POST,
        controllers::users_controller::handle_create,
    );

    app = app.route(
        "/users/{id}",
        Method::GET,
        controllers::users_controller::handle_show,
    );

    app = app.route(
        "/users/{id}/edit",
        Method::GET,
        controllers::users_controller::handle_edit,
    );

    app = app.route(
        "/users/{id}",
        Method::POST,
        controllers::users_controller::handle_post,
    );

    app = app.route(
        "/users/{id}",
        Method::PATCH,
        controllers::users_controller::handle_update,
    );

    app = app.route(
        "/users/{id}",
        Method::DELETE,
        controllers::users_controller::handle_destroy,
    );

    app = app.route(
        "/signin",
        Method::GET,
        controllers::sessions_controller::handle_new,
    );

    app = app.route(
        "/signin",
        Method::POST,
        controllers::sessions_controller::handle_create,
    );

    app = app.route(
        "/signout",
        Method::POST,
        controllers::sessions_controller::handle_post,
    );

    app = app.route(
        "/signout",
        Method::DELETE,
        controllers::sessions_controller::handle_destroy,
    );

    app
}

fn main() {
    let sys = actix::System::new("webapp_sample");

    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let addr = SyncArbiter::start(3, move || DbExecutor(pool.clone()));

    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S %z"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    let context = Context::new(addr);
 
    server::new(move || app(context.clone()))
        .bind("127.0.0.1:8088")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
