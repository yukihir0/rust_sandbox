use std::sync::Arc;

use handlebars::{Handlebars};

use actix::prelude::*;

use db::{DbExecutor}; 

#[derive(Clone)]
pub struct Context {
    pub templates: Arc<Handlebars>,
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
}
