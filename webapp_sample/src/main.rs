
extern crate dotenv;
extern crate env_logger;
extern crate handlebars;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

extern crate actix;
extern crate actix_web;
extern crate futures;

#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

mod db;
mod models;
mod schema;

use dotenv::dotenv;
use std::env;
use handlebars::{Handlebars, to_json};
use std::sync::Arc;
use serde_json::value::{Map};

use actix::prelude::*;
use actix_web::{server, fs, App, Form, HttpRequest, HttpResponse, FutureResponse, AsyncResponder};
use actix_web::http::{Method, StatusCode};
use actix_web::middleware::Logger;
use actix_web::middleware::session::{RequestSession, SessionStorage, CookieSessionBackend};
use futures::Future;

use diesel::prelude::*;
use r2d2_diesel::ConnectionManager;
use db::{DbExecutor, MessageReadUsers, MessageCreateUser, MessageReadUser, MessageUpdateUser, MessageDeleteUser};

#[derive(Clone)]
struct Context {
    templates: Arc<Handlebars>,
    db:        Addr<DbExecutor>,
}

impl Context {
    fn new(db: Addr<DbExecutor>) -> Self {
        let mut templates = Handlebars::new();
        
        for (name, path) in vec![
            ("layout", "./src/views/layout.hbs"),
            ("index", "./src/views/index.hbs"),
            ("users_index", "./src/views/users_index.hbs"),
            ("users_new", "./src/views/users_new.hbs"),
            ("users_show", "./src/views/users_show.hbs"),
            ("users_edit", "./src/views/users_edit.hbs"),
        ] {
            templates
                .register_template_file(name, path)
                .expect("failed to register template");
        }

        Self {
            templates: Arc::new(templates),
            db: db,
        }
    }
}

fn handle_index(req: HttpRequest<Context>) -> HttpResponse {
    let counter_key = "counter";
    
    let counter =  match req.session().get::<i32>(counter_key) {
        Ok(Some(count)) => {
            if count >= 9 {
                1
            } else {
                count + 1
            }
        },
        _ => 1,
    };

    req.session().set(counter_key, counter);

    let mut data = Map::new();
    data.insert("count".to_string(), to_json(&counter));
   
    match req.state().templates.render("index", &data) {
        Ok(body) => HttpResponse::Ok().body(body),
        Err(_)   => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Deserialize)]
pub struct UsersCreateParam {
    user_name:  String,
    user_email: String,
}

#[derive(Deserialize)]
pub struct UsersEditParam {
    method:     String,
    user_name:  String,
    user_email: String,
}

fn handle_users_index(req: HttpRequest<Context>) -> FutureResponse<HttpResponse> {
    req.state()
        .db
        .send(MessageReadUsers{})
        .from_err()
        .and_then(move |res| match res {
            Ok(users) => {
                let mut data = Map::new();
                data.insert("users".to_string(), to_json(&users));

                match req.state().templates.render("users_index", &data) {
                    Ok(body) => Ok(HttpResponse::Ok().body(body)),
                    Err(_)   => Ok(HttpResponse::InternalServerError().into()),
                }
            },
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

fn handle_users_new(req: HttpRequest<Context>) -> HttpResponse {
    match req.state().templates.render("users_new", &json!({})) {
        Ok(body) => HttpResponse::Ok().body(body),
        Err(_)   => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

fn handle_users_create((req, params): (HttpRequest<Context>, Form<UsersCreateParam>)) -> FutureResponse<HttpResponse> {
    req.state()
        .db
        .send(MessageCreateUser{name: params.user_name.clone(), email: params.user_email.clone()})
        .from_err()
        .and_then(move |res| match res {
            Ok(_user) => {
                let status = StatusCode::from_u16(303).expect("invalide status given");
                Ok(HttpResponse::build(status).header("Location", "/users").finish())
            },
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

fn handle_users_show(req: HttpRequest<Context>) -> FutureResponse<HttpResponse> {
    use futures::future::ok;
    
    let id: i32 = match req.match_info().query("id") {
        Ok(id) => id,
        Err(_) => return Box::new(ok(HttpResponse::InternalServerError().into())),
    };

    req.state()
        .db
        .send(MessageReadUser{id: id})
        .from_err()
        .and_then(move |res| match res {
            Ok(user) => {
                let mut data = Map::new();
                data.insert("user".to_string(), to_json(&user));

                match req.state().templates.render("users_show", &data) {
                    Ok(body) => Ok(HttpResponse::Ok().body(body)),
                    Err(_)   => Ok(HttpResponse::InternalServerError().into()),
                }
            },
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

fn handle_users_edit(req: HttpRequest<Context>) -> FutureResponse<HttpResponse> {
    use futures::future::ok;
    
    let id: i32 = match req.match_info().query("id") {
        Ok(id) => id,
        Err(_) => return Box::new(ok(HttpResponse::InternalServerError().into())),
    };

    req.state()
        .db
        .send(MessageReadUser{id: id})
        .from_err()
        .and_then(move |res| match res {
            Ok(user) => {
                let mut data = Map::new();
                data.insert("user".to_string(), to_json(&user));

                match req.state().templates.render("users_edit", &data) {
                    Ok(body) => Ok(HttpResponse::Ok().body(body)),
                    Err(_)   => Ok(HttpResponse::InternalServerError().into()),
                }
            },
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

fn handle_users_post((req, params): (HttpRequest<Context>, Form<UsersEditParam>)) -> FutureResponse<HttpResponse> {
    use futures::future::ok;
   
     match Method::from_bytes(params.method.as_bytes()) {
         Ok(Method::PATCH)  => handle_users_update((req, params)),
         Ok(Method::DELETE) => handle_users_destroy((req, params)),
         _                  => Box::new(ok(HttpResponse::InternalServerError().into())),
     }
}

fn handle_users_update((req, params): (HttpRequest<Context>, Form<UsersEditParam>)) -> FutureResponse<HttpResponse> {
    use futures::future::ok;
    
    let id: i32 = match req.match_info().query("id") {
        Ok(id) => id,
        Err(_) => return Box::new(ok(HttpResponse::InternalServerError().into())),
    };

    req.state()
        .db
        .send(MessageUpdateUser{id: id, name: params.user_name.clone(), email:
        params.user_email.clone()})
        .from_err()
        .and_then(move |res| match res {
            Ok(_user) => {
                let status = StatusCode::from_u16(303).expect("invalide status given");
                Ok(HttpResponse::build(status).header("Location", "/users").finish())
            },
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

fn handle_users_destroy((req, _params): (HttpRequest<Context>, Form<UsersEditParam>)) -> FutureResponse<HttpResponse> {
    use futures::future::ok;
    
    let id: i32 = match req.match_info().query("id") {
        Ok(id) => id,
        Err(_) => return Box::new(ok(HttpResponse::InternalServerError().into())),
    };

    req.state()
        .db
        .send(MessageDeleteUser{id: id})
        .from_err()
        .and_then(move |res| match res {
            Ok(_user) => {
                let status = StatusCode::from_u16(303).expect("invalide status given");
                Ok(HttpResponse::build(status).header("Location", "/users").finish())
            },
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

fn app(context: Context) -> App<Context> {
    let mut app = App::with_state(context);
     
    app = app.middleware(
        Logger::default()
    );
    
    app = app.middleware(
        SessionStorage::new(CookieSessionBackend::signed(&[0; 32]).secure(false))
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
        handle_index,
    );
    
    app = app.route(
        "/users",
        Method::GET,
        handle_users_index,
    );

    app = app.route(
        "/users/new",
        Method::GET,
        handle_users_new,
    );

    app = app.route(
        "/users",
        Method::POST,
        handle_users_create,
    );

    app = app.route(
        "/users/{id}",
        Method::GET,
        handle_users_show,
    );

    app = app.route(
        "/users/{id}/edit",
        Method::GET,
        handle_users_edit,
    );

    app = app.route(
        "/users/{id}",
        Method::POST,
        handle_users_post,
    );

    app = app.route(
        "/users/{id}",
        Method::PATCH,
        handle_users_update,
    );

    app = app.route(
        "/users/{id}",
        Method::DELETE,
        handle_users_destroy,
    );

    app
}

fn main() {
    let sys = actix::System::new("webapp_sample");

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let addr = SyncArbiter::start(3, move || DbExecutor(pool.clone()));

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let context = Context::new(addr);
 
    server::new(move || app(context.clone()))
        .bind("127.0.0.1:8088")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
