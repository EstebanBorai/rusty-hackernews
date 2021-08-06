use actix_files::Files;
use actix_web::web::{get, scope, ServiceConfig};

mod api;

pub fn bind_routes(app: &mut ServiceConfig) {
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

    // File Serving
    if cfg!(debug_assertions) {
        app.service(Files::new("/", "./dist").index_file("index.html"));
    } else {
        app.service(Files::new("/", "./static").index_file("index.html"));
    }
}
