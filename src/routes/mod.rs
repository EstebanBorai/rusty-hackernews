use actix_web::web::{get, ServiceConfig};

mod index;
mod statics;

pub fn bind_routes(app: &mut ServiceConfig) {
    // File Serving
    app.route("/static/{path}", get().to(statics::statics));

    // Views
    app.route("/", get().to(index::index));
}
