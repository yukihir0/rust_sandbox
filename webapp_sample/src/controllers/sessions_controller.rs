use handlebars::{to_json};
use regex::{Regex};
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
        let flash_message = sessions_helper::get_flash_message(req.session());

        let mut data = Map::new();
        data.insert(
            "flash_message".to_string(),
            to_json(
                flash_message,
            )
        );
    
        Box::new(ok(req.state().render_template("sessions_new", Some(data))))
    }
}

pub fn handle_create((req, params): (HttpRequest<Context>, Form<SessionsCreateParam>)) -> FutureResponse<HttpResponse> {
    use futures::future::ok;
   
    let mut error_messages =  Vec::new();

    let email_len = params.user_email.chars().count();
    if email_len <= 0 {
        error_messages.push("メールアドレスを入力してください".to_string());
    }
    
    let password_len = params.user_password.chars().count();
    if password_len <= 0  {
        error_messages.push("パスワードを入力してください".to_string());
    }

    let re_email = Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*$").unwrap();
    if !re_email.is_match(&params.user_email) {
        error_messages.push("メールアドレスはxxx@xxxの形式で入力してください".to_string());

    }
    let re_password = Regex::new(r"^[a-zA-Z\d]{8,30}$").unwrap();
    if !re_password.is_match(&params.user_password) {
        error_messages.push("パスワードは英数字8文字以上、30文字以下を入力してください".to_string());

    }

    if error_messages.len() > 0 {
        let flash_message = sessions_helper::FlashMessage{error_messages: error_messages};
        sessions_helper::set_flash_message(
            req.session(),
            flash_message,
        );
        return Box::new(ok(req.state().http_redirect("/signin", 303)));
    }

    req.state()
        .db
        .send(users_message::ReadUserByEmail{email: params.user_email.clone()})
        .from_err()
        .and_then(move |res| match res {
            Ok(user) => { 
                if sessions_helper::is_valid_password(&params.user_password, &user.password_digest) {
                    sessions_helper::signin(req.session(), user.id);
                } else {
                    let flash_message = sessions_helper::FlashMessage{
                        error_messages: vec!["メールアドレスもしくはパスワードが間違っています。".to_string()]
                    };

                    sessions_helper::set_flash_message(
                        req.session(),
                        flash_message,
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
