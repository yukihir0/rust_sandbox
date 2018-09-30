use std::sync::Arc;

use handlebars::{Handlebars};
use serde_json::value::{Map, Value};

use actix::prelude::*;
use actix_web::{HttpResponse};
use actix_web::http::{StatusCode};

use db::{DbExecutor}; 

#[derive(Clone)]
pub struct Context {
    templates: Arc<Handlebars>,
    pub db:    Addr<DbExecutor>,
}

impl Context {
    pub fn new(db: Addr<DbExecutor>) -> Self {
        let mut templates = Handlebars::new();
        
        for (name, path) in vec![
            ("layout",          "./src/views/layout.hbs"),
            ("index",           "./src/views/index.hbs"),
            ("users_index",     "./src/views/users_index.hbs"),
            ("users_new",       "./src/views/users_new.hbs"),
            ("users_show",      "./src/views/users_show.hbs"),
            ("users_edit",      "./src/views/users_edit.hbs"),
            ("sessions_new",    "./src/views/sessions_new.hbs"),
            ("sessions_delete", "./src/views/sessions_delete.hbs"),
        ] {
            templates
                .register_template_file(name, path)
                .expect("failed to register template");
        }

        Self {
            templates: Arc::new(templates),
            db:        db,
        }
    }

    pub fn render_template(&self, name: &str, data: Option<Map<String, Value>>) -> HttpResponse {
        let params = match data {
            Some(d) => d,
            None    => Map::new(),
        };

        match self.templates.render(name, &params) {
            Ok(body) => HttpResponse::Ok().body(body),
            Err(_)   => HttpResponse::InternalServerError().finish(),
        }
    }

    pub fn http_redirect(&self, path: &str, code: u16) -> HttpResponse {
        let status = StatusCode::from_u16(code)
            .expect("invalide status given");

        HttpResponse::build(status)
            .header("Location", path)
            .finish()
    }

    pub fn http_status(&self, code: u16) -> HttpResponse {
        let status = StatusCode::from_u16(code)
            .expect("invalide status given");
        
        HttpResponse::build(status)
            .finish()
    }

    pub fn http_internal_server_error(&self) -> HttpResponse {
        self.http_status(500)
    }
}
