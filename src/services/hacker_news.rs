use futures::future::join_all;
use link_preview::LinkPreview;
use reqwest::get;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::error::{Error, Result};
use crate::models::hacker_news::Story;

use super::link_preview::LinkPreviewService;

const BASE_URL: &str = "https://hacker-news.firebaseio.com/v0";

pub struct HackerNewsService {
    total_newstories: usize,
    page_size: usize,
    link_preview_service: Arc<Mutex<LinkPreviewService>>,
}

impl HackerNewsService {
    pub fn new(link_preview_service: Arc<Mutex<LinkPreviewService>>, page_size: usize) -> Self {
        HackerNewsService {
            total_newstories: 0,
            page_size,
            link_preview_service,
        }
    }

    pub async fn find_new_stories(&mut self) -> Result<Vec<(Story, Option<LinkPreview>)>> {
        let newstories_ids = self.find_newstories_ids().await?;
        let stories = join_all(
            newstories_ids
                .into_iter()
                .take(self.page_size)
                .map(|id| self.find_story_with_preview(id)),
        )
        .await
        .into_iter()
        .map(|story_result| story_result.unwrap())
        .collect();

        Ok(stories)
    }

    pub async fn find_story(&self, id: u64) -> Result<Story> {
        match get(format!("{}/item/{}.json", BASE_URL, id)).await {
            Ok(res) => Ok(serde_json::from_str::<Story>(&res.text().await.unwrap()).unwrap()),
            Err(err) => Err(Error::from(err)),
        }
    }

    pub async fn find_story_with_preview(&self, id: u64) -> Result<(Story, Option<LinkPreview>)> {
        let story = self.find_story(id).await?;

        if let Some(url) = story.clone().url {
            let preview = self
                .link_preview_service
                .lock()
                .await
                .preview_from_url(url.as_str())
                .await;

            return Ok((story, preview));
        }

        Ok((story, None))
    }

    async fn find_newstories_ids(&mut self) -> Result<Vec<u64>> {
        match get(format!("{}/newstories.json", BASE_URL)).await {
            Ok(res) => {
                let stories_ids: Vec<u64> =
                    serde_json::from_str(&res.text().await.unwrap()).unwrap();
                self.total_newstories = stories_ids.len();

                Ok(stories_ids)
            }
            Err(err) => Err(Error::from(err)),
        }
    }
}
