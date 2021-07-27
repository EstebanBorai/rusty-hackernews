use reqwest::get;

use crate::error::{Error, Result};
use crate::models::hacker_news::Story;

const BASE_URL: &str = "https://hacker-news.firebaseio.com/v0";

pub struct HackerNewsService;

impl HackerNewsService {
    pub fn new() -> Self {
        HackerNewsService
    }

    pub async fn find_new_stories(&self) -> Result<Vec<Story>> {
        match get(format!("{}/newstories.json", BASE_URL)).await {
            Ok(res) => {
                let stories_ids: Vec<u64> =
                    serde_json::from_str(&res.text().await.unwrap()).unwrap();
                let mut stories: Vec<Story> = Vec::new();

                for story_id in stories_ids.iter().take(25) {
                    let story = self.find_story(*story_id).await?;

                    stories.push(story);
                }

                Ok(stories)
            }
            Err(err) => Err(Error::from(err)),
        }
    }

    pub async fn find_story(&self, id: u64) -> Result<Story> {
        match get(format!("{}/item/{}.json", BASE_URL, id)).await {
            Ok(res) => Ok(serde_json::from_str::<Story>(&res.text().await.unwrap()).unwrap()),
            Err(err) => Err(Error::from(err)),
        }
    }
}
