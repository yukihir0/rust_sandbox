use handlebars::{to_json};
use serde_json::value::{Map};

use actix_web::{Form, HttpRequest, HttpResponse, FutureResponse, AsyncResponder};
use actix_web::http::{Method, StatusCode};
use futures::Future;

use db::{MessageReadUsers, MessageCreateUser, MessageReadUser, MessageUpdateUser, MessageDeleteUser};

use context::Context;

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

pub fn handle_index(req: HttpRequest<Context>) -> FutureResponse<HttpResponse> {
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

pub fn handle_new(req: HttpRequest<Context>) -> HttpResponse {
    match req.state().templates.render("users_new", &json!({})) {
        Ok(body) => HttpResponse::Ok().body(body),
        Err(_)   => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub fn handle_create((req, params): (HttpRequest<Context>, Form<UsersCreateParam>)) -> FutureResponse<HttpResponse> {
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

pub fn handle_show(req: HttpRequest<Context>) -> FutureResponse<HttpResponse> {
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

pub fn handle_edit(req: HttpRequest<Context>) -> FutureResponse<HttpResponse> {
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

pub fn handle_post((req, params): (HttpRequest<Context>, Form<UsersEditParam>)) -> FutureResponse<HttpResponse> {
    use futures::future::ok;
   
     match Method::from_bytes(params.method.as_bytes()) {
         Ok(Method::PATCH)  => handle_update((req, params)),
         Ok(Method::DELETE) => handle_destroy((req, params)),
         _                  => Box::new(ok(HttpResponse::InternalServerError().into())),
     }
}

pub fn handle_update((req, params): (HttpRequest<Context>, Form<UsersEditParam>)) -> FutureResponse<HttpResponse> {
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

pub fn handle_destroy((req, _params): (HttpRequest<Context>, Form<UsersEditParam>)) -> FutureResponse<HttpResponse> {
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
