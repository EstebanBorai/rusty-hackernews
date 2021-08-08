use actix_web::http::StatusCode;
use actix_web::web::{Data, HttpRequest, Path, Query};
use actix_web::HttpResponse;
use common::LinkPreview;
use serde::Deserialize;

use crate::error::Error;
use crate::AppData;

#[derive(Debug, Deserialize)]
pub struct FetchPreviewParams {
    url: Option<String>,
}

pub async fn fetch_preview(app_data: Data<AppData>, req: HttpRequest) -> HttpResponse {
    let params = Query::<FetchPreviewParams>::from_query(req.query_string()).unwrap();

    if let Some(url) = params.url.clone() {
        if let Some(preview) = app_data
            .link_preview_service
            .lock()
            .await
            .preview_from_url(url.as_str())
            .await
        {
            let image_url = preview.image_url.and_then(|url| Some(url.to_string()));

            return HttpResponse::Ok().json(LinkPreview {
                title: preview.title,
                description: preview.description,
                image_url,
            });
        }

        return HttpResponse::NoContent().finish();
    }

    Error::new(
        StatusCode::BAD_REQUEST,
        "The URL query param is required",
        None,
    )
    .as_http_response()
}
