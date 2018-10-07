use actix_web::{State, HttpResponse, FutureResponse};

use controllers;
use context::{Context};

pub fn handle_index(state: State<Context>) -> FutureResponse<HttpResponse> {
    use futures::future::ok;

    Box::new(ok(controllers::render(state.templates.clone(), "index", None)))
}
