use handlebars::{to_json};
use serde_json::value::{Map};

use actix_web::{Form, HttpRequest, HttpResponse, FutureResponse, AsyncResponder};
use actix_web::http::{Method};
use futures::Future;

use db::{users_message};

use context::{Context};

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

pub fn handle_index(req: HttpRequest<Context>) -> FutureResponse<HttpResponse> {
    req.state()
        .db
        .send(users_message::ReadUsers{})
        .from_err()
        .and_then(move |res| match res {
            Ok(users) => {
                let mut data = Map::new();
                data.insert("users".to_string(), to_json(&users));

                Ok(req.state().render_template("users_index", Some(data)))
            },
            Err(_) => {
                Ok(req.state().http_internal_server_error())
            },
        })
        .responder()
}

pub fn handle_new(req: HttpRequest<Context>) -> HttpResponse {
    req.state().render_template("users_new", None)
}

pub fn handle_create((req, params): (HttpRequest<Context>, Form<UsersCreateParam>)) -> FutureResponse<HttpResponse> {
    req.state()
        .db
        .send(users_message::CreateUser{
            name: params.user_name.clone(),
            email: params.user_email.clone(),
            password: params.user_password.clone()
        })
        .from_err()
        .and_then(move |res| match res {
            Ok(_user) => {
                Ok(req.state().http_redirect("/users", 303))
            },
            Err(_) => {
                Ok(req.state().http_internal_server_error())
            },
        })
        .responder()
}

pub fn handle_show(req: HttpRequest<Context>) -> FutureResponse<HttpResponse> {
    use futures::future::ok;
    
    let id: i32 = match req.match_info().query("id") {
        Ok(id) => id,
        Err(_) => return Box::new(ok(req.state().http_internal_server_error())),
    };

    req.state()
        .db
        .send(users_message::ReadUser{id: id})
        .from_err()
        .and_then(move |res| match res {
            Ok(user) => {
                let mut data = Map::new();
                data.insert("user".to_string(), to_json(&user));

                Ok(req.state().render_template("users_show", Some(data)))
            },
            Err(_) => {
                Ok(req.state().http_internal_server_error())
            },
        })
        .responder()
}

pub fn handle_edit(req: HttpRequest<Context>) -> FutureResponse<HttpResponse> {
    use futures::future::ok;
    
    let id: i32 = match req.match_info().query("id") {
        Ok(id) => id,
        Err(_) => return Box::new(ok(req.state().http_internal_server_error())),
    };

    req.state()
        .db
        .send(users_message::ReadUser{id: id})
        .from_err()
        .and_then(move |res| match res {
            Ok(user) => {
                let mut data = Map::new();
                data.insert("user".to_string(), to_json(&user));

                Ok(req.state().render_template("users_edit", Some(data)))
            },
            Err(_) => {
                Ok(req.state().http_internal_server_error())
            },
        })
        .responder()
}

pub fn handle_post((req, params): (HttpRequest<Context>, Form<UsersPostParam>)) -> FutureResponse<HttpResponse> {
    use futures::future::ok;
   
     match Method::from_bytes(params.method.as_bytes()) {
         Ok(Method::PATCH)  => handle_update((req, params)),
         Ok(Method::DELETE) => handle_destroy((req, params)),
         _                  => Box::new(ok(req.state().http_internal_server_error())),
     }
}

pub fn handle_update((req, params): (HttpRequest<Context>, Form<UsersPostParam>)) -> FutureResponse<HttpResponse> {
    use futures::future::ok;
    
    let id: i32 = match req.match_info().query("id") {
        Ok(id) => id,
        Err(_) => return Box::new(ok(req.state().http_internal_server_error())),
    };

    let UsersPostParam{
        method:_,
        user_name,
        user_email,
        user_password
    } = params.into_inner();
   
    let name = user_name.unwrap_or("".to_string());
    let email = user_email.unwrap_or("".to_string());
    let password = user_password.unwrap_or("".to_string());
    
    req.state()
        .db
        .send(users_message::UpdateUser{
            id: id,
            name: name.clone(),
            email: email.clone(),
            password: password.clone()
        })
        .from_err()
        .and_then(move |res| match res {
            Ok(_user) => {
                Ok(req.state().http_redirect("/users", 303))
            },
            Err(_) => {
                Ok(req.state().http_internal_server_error())
            },
        })
        .responder()
}

pub fn handle_destroy((req, _params): (HttpRequest<Context>, Form<UsersPostParam>)) -> FutureResponse<HttpResponse> {
    use futures::future::ok;
    
    let id: i32 = match req.match_info().query("id") {
        Ok(id) => id,
        Err(_) => return Box::new(ok(req.state().http_internal_server_error())),
    };

    req.state()
        .db
        .send(users_message::DeleteUser{id: id})
        .from_err()
        .and_then(move |res| match res {
            Ok(_user) => {
                Ok(req.state().http_redirect("/users", 303))
            },
            Err(_) => {
                Ok(req.state().http_internal_server_error())
            },
        })
        .responder()
}
