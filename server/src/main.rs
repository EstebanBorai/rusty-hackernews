mod app_data;
mod domain;
mod environment;
mod error;
mod routes;
mod services;
mod utils;

use actix_web::{dev::Service, http::HeaderValue, App, HttpServer};
use reqwest::header;

use self::app_data::AppData;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = AppData::new().await;
    let port = std::env::var("PORT").unwrap_or(String::from("3000"));

    if cfg!(not(debug_assertions)) {
        let _guard = sentry::init((
            "https://a5eec1eb178d4b368e4dfad2c4b3c044@o446883.ingest.sentry.io/5934543",
            sentry::ClientOptions {
                release: sentry::release_name!(),
                ..Default::default()
            },
        ));
    }

    std::env::set_var("RUST_BACKTRACE", "1");

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .configure(routes::bind_routes)
            .wrap_fn(|req, srv| {
                let fut = srv.call(req);
                async {
                    let mut res = fut.await?;
                    let headers = res.headers_mut();

                    headers.insert(
                        header::ACCESS_CONTROL_ALLOW_ORIGIN,
                        HeaderValue::from_static("*"),
                    );

                    headers.insert(
                        header::ACCESS_CONTROL_ALLOW_METHODS,
                        HeaderValue::from_static("GET,OPTIONS"),
                    );

                    headers.insert(
                        header::ACCESS_CONTROL_ALLOW_HEADERS,
                        HeaderValue::from_static("*"),
                    );
                    Ok(res)
                }
            })
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
