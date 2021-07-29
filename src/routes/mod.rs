use actix_web::web::{get, scope, ServiceConfig};

mod api;
mod index;
mod statics;

pub fn bind_routes(app: &mut ServiceConfig) {
    // File Serving
    app.route("/static/{path}", get().to(statics::statics));

    // Views
    app.route("/", get().to(index::index));

    // API
    app.service(
        scope("/api").service(
            scope("/v1").service(
                scope("/stories")
                    .route("", get().to(api::v1::stories::list_new_stories))
                    .route("/{id}", get().to(api::v1::stories::find_one)),
            ),
        ),
    );
}
