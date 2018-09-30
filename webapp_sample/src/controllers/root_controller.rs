use actix_web::{HttpRequest, HttpResponse};

use context::{Context};

pub fn handle_index(req: HttpRequest<Context>) -> HttpResponse {
    req.state().render_template("index", None)
}
