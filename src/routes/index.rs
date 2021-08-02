use actix_web::web::Data;
use actix_web::HttpResponse;
use link_preview::LinkPreview;
use serde::Serialize;
use serde_json::json;

use crate::models::hacker_news::Story;
use crate::AppData;

#[derive(Serialize)]
pub struct Post {
    id: u64,
    r#type: String,
    by: String,
    score: u32,
    descendants: u32,
    time: u64,
    title: String,
    url: Option<String>,
    link_description: Option<String>,
    link_domain: Option<String>,
    link_image_url: Option<String>,
}

pub async fn index(app_data: Data<AppData>) -> HttpResponse {
    if let Ok(stories) = app_data
        .hacker_news_service
        .lock()
        .await
        .find_new_stories()
        .await
    {
        let stories: Vec<Post> = stories.into_iter().map(into_post_schema).collect();
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

        return HttpResponse::Ok().body(page);
    }

    HttpResponse::Ok().body("An error ocurred")
}

fn into_post_schema(data: (Story, Option<LinkPreview>)) -> Post {
    let (story, link_preview) = data;

    if link_preview.is_some() {
        let link_preview = link_preview.unwrap();

        return Post {
            id: story.id,
            r#type: story.r#type,
            by: story.by,
            score: story.score,
            time: story.time,
            title: story.title,
            descendants: story.descendants,
            url: Some(story.url.unwrap().to_string()),
            link_description: link_preview.description,
            link_domain: link_preview
                .domain
                .and_then(|domain| Some(domain.to_string())),
            link_image_url: link_preview.image_url.and_then(|url| Some(url.to_string())),
        };
    }

    Post {
        id: story.id,
        r#type: story.r#type,
        by: story.by,
        score: story.score,
        time: story.time,
        title: story.title,
        descendants: story.descendants,
        url: None,
        link_description: None,
        link_domain: None,
        link_image_url: None,
    }
}
