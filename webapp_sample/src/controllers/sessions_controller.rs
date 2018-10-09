use handlebars::{to_json};
use regex::{Regex};
use serde_json::value::{Map};

use actix_web::{State, Form, HttpResponse, FutureResponse, AsyncResponder};
use actix_web::http::{Method};
use actix_web::middleware::session::{Session};
use futures::Future;

use db::{users_message};
use context::{Context};
use controllers;
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

pub fn handle_new((state, session): (State<Context>, Session)) -> FutureResponse<HttpResponse> {
    use futures::future::ok;
    
    let templates = state.templates.clone();
   
    // TODO Refarctor Me
    if sessions_helper::is_signined(&session) {
        match sessions_helper::current_user_id(&session) {
            Some(user_id) => {
                state
                    .db
                    .send(users_message::ReadUser{id: user_id})
                    .from_err()
                    .and_then(move |res| match res {
                        Ok(user) => {
                            if sessions_helper::is_valid_session_id(&session, &user.session_digest.clone().unwrap()) {
                                let mut data = Map::new();
                                data.insert("user".to_string(), to_json(&user));

                                Ok(controllers::render(templates, "sessions_delete", Some(data)))
                            } else {
                                Ok(controllers::http_internal_server_error())
                            }
                        },
                        Err(_) => {
                            Ok(controllers::http_internal_server_error())
                        },
                    })
                    .responder()
            },
            None => {
                Box::new(ok(controllers::http_internal_server_error()))
            },
        }
    } else {
        let flash_message = sessions_helper::get_flash_message(&session);

        let mut data = Map::new();
        data.insert(
            "flash_message".to_string(),
            to_json(
                flash_message,
            )
        );
    
        Box::new(ok(controllers::render(templates, "sessions_new", Some(data))))
    }
}

pub fn handle_create((state, session, params): (State<Context>, Session, Form<SessionsCreateParam>)) -> FutureResponse<HttpResponse> {
    use futures::future::{ok, Either};
   
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
            &session,
            flash_message,
        );
        return Box::new(ok(controllers::http_redirect("/signin", 303)));
    }

    state
        .db
        .send(users_message::ReadUserByEmail{email: params.user_email.clone()})
        .from_err()
        .and_then(move |res| {
            // TODO Process missing user error
            res.map(move |user| user)
        })
        .and_then(move |user| {
            match sessions_helper::signin(&user, &params.user_password, &session) {
                Ok(user_session) => {
                    Either::A(
                        state
                            .db
                            .send(users_message::UpdateUserSession{
                                id: user_session.user_id,
                                session_id: user_session.session_id,
                            })
                            .from_err()
                            .and_then(move |res| {
                                res.map(move |_user| ())
                            })
                        )
                },
                Err(_e) => {
                    Either::B({
                        let flash_message = sessions_helper::FlashMessage{
                            error_messages: vec!["メールアドレスもしくはパスワードが間違っています。".to_string()]
                        };

                        sessions_helper::set_flash_message(&session, flash_message);
                        ok(())
                    })
                }
            }
        })
        .and_then(move |_| {
            Ok(controllers::http_redirect("/signin", 303))
        })
        .responder()
}

pub fn handle_post((state, session, params): (State<Context>, Session, Form<SessionsDeleteParam>)) -> FutureResponse<HttpResponse> {
    use futures::future::ok;
   
     match Method::from_bytes(params.method.as_bytes()) {
         Ok(Method::DELETE) => handle_destroy((state, session, params)),
         _                  => Box::new(ok(controllers::http_internal_server_error())),
     }
}

pub fn handle_destroy((_state, session, _params): (State<Context>, Session, Form<SessionsDeleteParam>)) -> FutureResponse<HttpResponse> {
    use futures::future::ok;
   
    sessions_helper::signout(&session);
    Box::new(ok(controllers::http_redirect("/signin", 303)))
}
