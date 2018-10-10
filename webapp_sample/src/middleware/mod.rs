use actix_web::middleware::{Finished, Middleware, Response, Started};
use actix_web::middleware::session::{RequestSession};
use actix_web::{HttpRequest, HttpResponse, Result};
use futures::Future;

use db::{users_message};
use context::{Context};
use helpers::{sessions_helper};
use controllers;

pub struct Authenticate;

impl Authenticate {
    pub fn new() -> Self {
        Self {}
    }
}

impl Middleware<Context> for Authenticate {
    fn start(&self, req: &HttpRequest<Context>) -> Result<Started> {
        let session = req.session();

        match sessions_helper::user_session(&session) {
            Ok(Some(user_session)) => {
                Ok(Started::Future(Box::new(
                    req
                        .state()
                        .db
                        .send(users_message::ReadUser{id: user_session.user_id})
                        .from_err()
                        .and_then(move |res| {
                            res.and_then(move |user| {
                                sessions_helper::valid_session_id(
                                    &user_session,
                                    &user.session_digest.clone().unwrap(),
                                ).and_then(|_| {
                                    println!("signin");
                                    Ok(None)
                                })
                            })
                        })
                        .or_else(|_| {
                            Ok(Some(controllers::http_status(403)))
                        })
                )))
            },
            _ => {
                println!("not signin");
                if req.path() == "/signin" {
                    Ok(Started::Done)
                } else {
                    Ok(Started::Response(controllers::http_redirect("/signin", 303)))
                }
            },
        }
    }

    fn response(&self, _req: &HttpRequest<Context>, resp: HttpResponse) -> Result<Response> {
        Ok(Response::Done(resp))
    }

    fn finish(&self, _req: &HttpRequest<Context>, _resp: &HttpResponse) -> Finished {
        Finished::Done
    }
}
