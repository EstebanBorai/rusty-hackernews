use common::Story;
use futures::future::join_all;
use reqwest::get;

use crate::error::{Error, Result};

const BASE_URL: &str = "https://hacker-news.firebaseio.com/v0";
const PAGE_SIZE: usize = 20;

pub struct HackerNewsService {
    total_newstories: usize,
}

impl HackerNewsService {
    pub fn new() -> Self {
        HackerNewsService {
            total_newstories: 0,
        }
    }

    pub async fn find_new_stories(&mut self, page: Option<usize>) -> Result<Vec<Story>> {
        let skip_pages = page.unwrap_or(0);
        let newstories_ids = self.find_newstories_ids().await?;
        let stories = join_all(
            newstories_ids
                .into_iter()
                .skip(skip_pages * PAGE_SIZE)
                .take(PAGE_SIZE)
                .map(|id| self.find_story(id)),
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
