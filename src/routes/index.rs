use actix_web::web::Data;
use actix_web::HttpResponse;
use futures::future::join_all;
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
        .map(|story| async {
            if story.url.is_some() {
                let url = story.clone().url.unwrap();
                if let Ok(response) = get(url).await {
                    let html = response.text().await.unwrap();
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
            }

            Post {
                story,
                link_title: None,
                link_description: None,
                link_domain: None,
                link_image_url: None,
            }
        });

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
