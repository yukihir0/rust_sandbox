use handlebars::{to_json};
use serde_json::value::{Map};

use actix_web::{Form, HttpRequest, HttpResponse, FutureResponse, AsyncResponder};
use actix_web::http::{Method};
use actix_web::middleware::session::{RequestSession};
use futures::Future;

use db::{users_message};

use context::{Context};
use helpers::{sessions_helper};

#[derive(Deserialize)]
pub struct SessionsCreateParam {
    user_email:    String,
    user_password: String,
}

#[derive(Deserialize)]
pub struct SessionsDeleteParam {
    method: String,
}

pub fn handle_new(req: HttpRequest<Context>) -> FutureResponse<HttpResponse> {
    use futures::future::ok;
    
    if sessions_helper::is_signined(req.session()) {
        match sessions_helper::current_user_id(req.session()) {
            Some(user_id) => {
                req.state()
                    .db
                    .send(users_message::ReadUser{id: user_id})
                    .from_err()
                    .and_then(move |res| match res {
                        Ok(user) => {
                            let mut data = Map::new();
                            data.insert("user".to_string(), to_json(&user));

                            Ok(req.state().render_template("sessions_delete", Some(data)))
                        },
                        Err(_) => {
                            Ok(req.state().http_internal_server_error())
                        },
                    })
                    .responder()
            },
            None => {
                Box::new(ok(req.state().http_internal_server_error()))
            },
        }
    } else {
        let mut data = Map::new();
        data.insert(
            "error_message".to_string(),
            to_json(
                sessions_helper::get_flash_message(req.session())
            )
        );
    
        Box::new(ok(req.state().render_template("sessions_new", Some(data))))
    }
}

pub fn handle_create((req, params): (HttpRequest<Context>, Form<SessionsCreateParam>)) -> FutureResponse<HttpResponse> {
    req.state()
        .db
        .send(users_message::ReadUserByEmail{email: params.user_email.clone()})
        .from_err()
        .and_then(move |res| match res {
            Ok(user) => { 
                if sessions_helper::is_valid_password(&params.user_password, &user.password_digest) {
                    sessions_helper::signin(req.session(), user.id);
                } else {
                    sessions_helper::set_flash_message(
                        req.session(),
                        "メールアドレスもしくはパスワードが間違っています。".to_string(),
                    );
                }

                Ok(req.state().http_redirect("/signin", 303))
            },
            Err(_) => {
                Ok(req.state().http_internal_server_error())
            },
        })
        .responder()
}

pub fn handle_post((req, params): (HttpRequest<Context>, Form<SessionsDeleteParam>)) -> FutureResponse<HttpResponse> {
    use futures::future::ok;
   
     match Method::from_bytes(params.method.as_bytes()) {
         Ok(Method::DELETE) => handle_destroy(req),
         _                  => Box::new(ok(req.state().http_internal_server_error())),
     }
}

pub fn handle_destroy(req: HttpRequest<Context>) -> FutureResponse<HttpResponse> {
    use futures::future::ok;
   
    sessions_helper::signout(req.session());
    Box::new(ok(req.state().http_redirect("/signin", 303)))
}
