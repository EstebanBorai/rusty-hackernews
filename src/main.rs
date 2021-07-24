mod app_data;
mod error;
mod models;
mod routes;
mod services;

use actix_web::{App, HttpServer};

use self::app_data::AppData;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = AppData::new().await;

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .configure(routes::bind_routes)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
