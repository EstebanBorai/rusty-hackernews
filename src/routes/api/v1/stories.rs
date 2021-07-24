use actix_web::web::{Data, Path};
use actix_web::HttpResponse;

use crate::AppData;

pub async fn list_new_stories(app_data: Data<AppData>) -> HttpResponse {
    match app_data.hacker_news_service.find_new_stories().await {
        Ok(story) => HttpResponse::Ok().json(story),
        Err(err) => err.as_http_response(),
    }
}

pub async fn find_one(app_data: Data<AppData>, id: Path<u64>) -> HttpResponse {
    match app_data
        .hacker_news_service
        .find_story(id.into_inner())
        .await
    {
        Ok(story) => HttpResponse::Ok().json(story),
        Err(err) => err.as_http_response(),
    }
}
