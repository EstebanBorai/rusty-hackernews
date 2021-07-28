use futures::future::join_all;
use reqwest::get;

use crate::error::{Error, Result};
use crate::models::hacker_news::Story;

const BASE_URL: &str = "https://hacker-news.firebaseio.com/v0";

/// Stories endpoints will gather around 500 posts which are too many per page
/// to reduce the amount of requests the endpoint will paginante for this size
/// of posts
const PAGE_SIZE: usize = 30;

pub struct HackerNewsService {
    total_newstories: usize,
}

impl HackerNewsService {
    pub fn new() -> Self {
        HackerNewsService {
            total_newstories: 0,
        }
    }

    pub async fn find_new_stories(&mut self) -> Result<Vec<Story>> {
        let newstories_ids = self.find_newstories_ids().await?;
        let page = HackerNewsService::calc_page(self.total_newstories, PAGE_SIZE);
        let stories = join_all(
            newstories_ids
                .into_iter()
                .skip(50)
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

    fn calc_page(total_items: usize, limit: usize) -> u32 {
        f32::ceil(total_items as f32 / limit as f32) as u32
    }
}
