use handlebars::{to_json};
use serde_json::value::{Map};

use actix_web::{State, Path, Form, HttpResponse, FutureResponse, AsyncResponder};
use actix_web::http::{Method};
use futures::Future;

use db::{users_message};
use context::{Context};
use controllers;

#[derive(Deserialize)]
pub struct UsersReadPath{
    pub id: i32,
}

#[derive(Deserialize)]
pub struct UsersCreateParam {
    user_name:     String,
    user_email:    String,
    user_password: String,
}

#[derive(Deserialize)]
pub struct UsersPostParam {
    method:        String,
    user_name:     Option<String>,
    user_email:    Option<String>,
    user_password: Option<String>,
}

pub fn handle_index(state: State<Context>) -> FutureResponse<HttpResponse> {
    let templates = state.templates.clone();
    
    state
        .db
        .send(users_message::ReadUsers{})
        .from_err()
        .and_then(move |res| {
            res.map(move |users| {
                let mut data = Map::new();
                data.insert("users".to_string(), to_json(&users));
                data
            })
        })
        .and_then(move |data| {
            Ok(controllers::render(templates, "users_index", Some(data)))
        })
        .responder()
}

pub fn handle_new(state: State<Context>) -> FutureResponse<HttpResponse> {
    use futures::future::ok;

    Box::new(ok(controllers::render(state.templates.clone(), "users_new", None)))
}

pub fn handle_create((state, params): (State<Context>, Form<UsersCreateParam>)) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(users_message::CreateUser{
            name: params.user_name.clone(),
            email: params.user_email.clone(),
            password: params.user_password.clone()
        })
        .from_err()
        .and_then(move |res| {
            res.map(move |user| user)
        })
        .and_then(move |_| {
            Ok(controllers::http_redirect("/users", 303))
        })
        .responder()
}

pub fn handle_show((state, path): (State<Context>, Path<UsersReadPath>)) -> FutureResponse<HttpResponse> {
    let templates = state.templates.clone();
    
    state
        .db
        .send(users_message::ReadUser{id: path.id})
        .from_err()
        .and_then(move |res| {
            res.map(move |user| {
                let mut data = Map::new();
                data.insert("user".to_string(), to_json(&user));
                data
            })
        })
        .and_then(move |data| {
            Ok(controllers::render(templates, "users_show", Some(data)))
        })
        .responder()
}

pub fn handle_show_chain((state, path): (State<Context>, Path<UsersReadPath>)) -> FutureResponse<HttpResponse> {
    let templates = state.templates.clone();

    state
        .db
        .send(users_message::ReadUser{id: path.id})
        .from_err()
        .and_then(move |res| {
            res.map(move |user| user)
        })
        .and_then(move |user| {
            state
                .db
                .send(users_message::ReadUser{id: user.id})
                .from_err()
                .and_then(move |res| {
                    res.map(move |user| user)
                })
                .and_then(move |user| {
                    state
                        .db
                        .send(users_message::ReadUser{id: user.id})
                        .from_err()
                        .and_then(move |res| {
                            res.map(move |user| {
                                let mut data = Map::new();
                                data.insert("user".to_string(), to_json(&user));
                                data
                            })
                        })
                })
        }) 
        .and_then(move |data| {
            Ok(controllers::render(templates, "users_show", Some(data)))
        })
        .responder()
}

pub fn handle_edit((state, path): (State<Context>, Path<UsersReadPath>)) -> FutureResponse<HttpResponse> {
    let templates = state.templates.clone();
    
    state
        .db
        .send(users_message::ReadUser{id: path.id})
        .from_err()
        .and_then(move |res| {
            res.map(move |user| {
                let mut data = Map::new();
                data.insert("user".to_string(), to_json(&user));
                data
            })
        })
        .and_then(move |data| {
            Ok(controllers::render(templates, "users_edit", Some(data)))
        })
        .responder()
}

pub fn handle_post((state, path, params): (State<Context>, Path<UsersReadPath>, Form<UsersPostParam>)) -> FutureResponse<HttpResponse> {
    use futures::future::ok;
   
     match Method::from_bytes(params.method.as_bytes()) {
         Ok(Method::PATCH)  => handle_update((state, path, params)),
         Ok(Method::DELETE) => handle_destroy((state, path, params)),
         _                  => Box::new(ok(controllers::http_internal_server_error())),
     }
}

pub fn handle_update((state, path, params): (State<Context>, Path<UsersReadPath>, Form<UsersPostParam>)) -> FutureResponse<HttpResponse> {
    let UsersPostParam{
        method:_,
        user_name,
        user_email,
        user_password
    } = params.into_inner();
   
    let name = user_name.unwrap_or("".to_string());
    let email = user_email.unwrap_or("".to_string());
    let password = user_password.unwrap_or("".to_string());
    
    state
        .db
        .send(users_message::UpdateUser{
            id: path.id,
            name: name.clone(),
            email: email.clone(),
            password: password.clone()
        })
        .from_err()
        .and_then(move |res| {
            res.map(move |user| user)
        })
        .and_then(move |_| {
            Ok(controllers::http_redirect("/users", 303))
        })
        .responder()
}

pub fn handle_destroy((state, path, _params): (State<Context>, Path<UsersReadPath>, Form<UsersPostParam>)) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(users_message::DeleteUser{id: path.id})
        .from_err()
        .and_then(move |res| {
            res.map(move |user| user)
        })
        .and_then(move |_| {
            Ok(controllers::http_redirect("/users", 303))
        })
        .responder()
}
