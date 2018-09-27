use handlebars::{to_json};
use serde_json::value::{Map};

use actix_web::{HttpRequest, HttpResponse};
use actix_web::http::{StatusCode};
use actix_web::middleware::session::{RequestSession};

use context::Context;

pub fn handle_index(req: HttpRequest<Context>) -> HttpResponse {
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
