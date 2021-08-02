use actix_web::web::{Data, Path};
use actix_web::HttpResponse;

use crate::AppData;

pub async fn statics(app_data: Data<AppData>, path: Path<String>) -> HttpResponse {
    let path = path.into_inner();

    match path.as_str() {
        "styles.css" => HttpResponse::Ok().body(app_data.static_resources.styles.clone()),
        _ => HttpResponse::NotFound().body(String::from("Not Found")),
    }
}
