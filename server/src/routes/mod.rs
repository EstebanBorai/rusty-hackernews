use actix_files::{Files, NamedFile};
use actix_web::web::{get, scope, ServiceConfig};

mod api;

#[cfg(debug_assertions)]
const STATIC_SERVE_FROM: &str = "./dist";

#[cfg(not(debug_assertions))]
const STATIC_SERVE_FROM: &str = "./static";

pub fn bind_routes(app: &mut ServiceConfig) {
    // API
    app.service(
        scope("/api").service(
            scope("/v1")
                .service(
                    scope("/stories")
                        .route("", get().to(api::v1::stories::list_new_stories))
                        .route("/{id}", get().to(api::v1::stories::find_one)),
                )
                .service(scope("/previews").route("", get().to(api::v1::previews::fetch_preview))),
        ),
    );

    // File Serving
    // In order to handle client-side routing accordingly the `index.html` file
    // is always served using the `default_handler`.
    //
    // This affects other static files from being served (images, videos,
    // scripts).
    app.service(
        Files::new("/", STATIC_SERVE_FROM)
            .default_handler(NamedFile::open(format!("{}/index.html", STATIC_SERVE_FROM)).unwrap())
            .index_file("index.html"),
    );
}
