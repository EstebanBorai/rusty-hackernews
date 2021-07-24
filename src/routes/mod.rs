use actix_web::web::{get, scope, ServiceConfig};

mod api;

pub fn bind_routes(app: &mut ServiceConfig) {
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
