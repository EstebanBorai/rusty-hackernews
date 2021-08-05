mod app_data;
mod error;
mod middleware;
mod routes;
mod services;

use actix_web::{App, HttpServer};

use self::app_data::AppData;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap_or(String::from("3000"));
    let data = AppData::new().await;

    HttpServer::new(move || {
        let cors = middleware::cors::make_cors_middleware();

        App::new()
            .wrap(cors)
            .app_data(data.clone())
            .configure(routes::bind_routes)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
