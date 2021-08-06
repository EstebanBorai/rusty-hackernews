mod app_data;
mod error;
mod routes;
mod services;

use actix_web::{dev::Service, http::HeaderValue, App, HttpServer};
use reqwest::header;

use self::app_data::AppData;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap_or(String::from("3000"));
    let data = AppData::new().await;

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
