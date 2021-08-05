use actix_web::web::{Data, HttpRequest, Path, Query};
use actix_web::HttpResponse;
use serde::Deserialize;

use crate::AppData;

#[derive(Debug, Deserialize)]
pub struct ListStoriesParams {
    page: Option<usize>,
}

pub async fn list_new_stories(app_data: Data<AppData>, req: HttpRequest) -> HttpResponse {
    let params = Query::<ListStoriesParams>::from_query(req.query_string()).unwrap();

    match app_data
        .hacker_news_service
        .lock()
        .await
        .find_new_stories(params.page)
        .await
    {
        Ok(story) => HttpResponse::Ok().json(story),
        Err(err) => err.as_http_response(),
    }
}

pub async fn find_one(app_data: Data<AppData>, id: Path<u64>) -> HttpResponse {
    match app_data
        .hacker_news_service
        .lock()
        .await
        .find_story(id.into_inner())
        .await
    {
        Ok(story) => HttpResponse::Ok().json(story),
        Err(err) => err.as_http_response(),
    }
}
