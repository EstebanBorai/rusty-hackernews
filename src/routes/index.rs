use actix_web::web::Data;
use actix_web::HttpResponse;
use futures::future::join_all;
use futures::StreamExt;
use link_preview::LinkPreview;
use reqwest::get;
use serde::Serialize;
use serde_json::json;
use std::str::FromStr;

use crate::models::hacker_news::Story;
use crate::AppData;

#[derive(Serialize)]
pub struct Post {
    story: Story,
    link_title: Option<String>,
    link_description: Option<String>,
    link_domain: Option<String>,
    link_image_url: Option<String>,
}

pub async fn index(app_data: Data<AppData>) -> HttpResponse {
    let stories = app_data
        .hacker_news_service
        .lock()
        .unwrap()
        .find_new_stories()
        .await
        .unwrap()
        .into_iter()
        .map(into_post_schema);

    let stories = join_all(stories).await;

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

async fn into_post_schema(story: Story) -> Post {
    if story.url.is_some() {
        let url = story.clone().url.unwrap();
        let html = fetch(url.as_str()).await;

        if let Some(LinkPreview {
            title,
            description,
            domain,
            image_url,
        }) = LinkPreview::from_str(html.as_str()).ok()
        {
            return Post {
                story,
                link_title: title,
                link_description: description,
                link_domain: domain.and_then(|domain| Some(domain.to_string())),
                link_image_url: image_url.and_then(|url| Some(url.to_string())),
            };
        }
    }

    Post {
        story,
        link_title: None,
        link_description: None,
        link_domain: None,
        link_image_url: None,
    }
}

async fn fetch(url: &str) -> String {
    let limit = 15;
    let mut current = 0;
    let mut html: Vec<String> = Vec::new();
    let mut reading = false;
    let mut finish = false;

    while let Some(value) = get(url).await.unwrap().bytes_stream().next().await {
        current += 1;

        if current >= limit {
            return html.join(" ").to_string();
        }

        if finish {
            return html.join(" ").to_string();
        }

        let bytes = value.unwrap();
        let text = bytes.to_vec();
        let utf8 = String::from_utf8(text).unwrap();

        if reading {
            html.push(utf8);
            continue;
        }

        if utf8.to_lowercase().contains("<!doctype html>") {
            reading = true;
            html.push(utf8);

            continue;
        }

        if utf8.to_lowercase().contains("</head>") {
            html.push(utf8);
            finish = true;
            continue;
        }
    }

    String::default()
}
