use actix_web::web::Data;
use actix_web::HttpResponse;
use serde_json::json;

use crate::AppData;

pub async fn index(app_data: Data<AppData>) -> HttpResponse {
    let stories = app_data
        .hacker_news_service
        .find_new_stories()
        .await
        .unwrap();
    let page = app_data
        .handlebars
        .render(
            "index",
            &json!({
              "parent": "layout",
              "stories": stories,
            }),
        )
        .unwrap();

    HttpResponse::Ok().body(page)
}
