use actix_web::http::StatusCode;
use common::hacker_news::{Comment, Item, Story};
use futures::future::join_all;
use reqwest::get;

use crate::error::{Error, Result};

const BASE_URL: &str = "https://hacker-news.firebaseio.com/v0";
const PAGE_SIZE: usize = 20;

pub struct HackerNewsService;

impl HackerNewsService {
    pub fn new() -> Self {
        HackerNewsService
    }

    pub async fn find_new_stories(&mut self, page: Option<usize>) -> Result<Vec<Story>> {
        let newstories_ids = self.find_newstories_ids(page).await?;
        let find_story_futures = newstories_ids.iter().map(|id| self.find_story(id));
        let stories = join_all(find_story_futures)
            .await
            .into_iter()
            .collect::<Result<Vec<Story>>>()?;

        Ok(stories)
    }

    pub async fn find_story(&self, id: &u64) -> Result<Story> {
        let item = self.find_item(id).await?;

        match item {
            Item::Story(story) => Ok(story),
            _ => Err(Error::new(
                StatusCode::BAD_REQUEST,
                &format!("The provided ID doesn't belong to a story item"),
                None,
            )),
        }
    }

    pub async fn find_story_comments(&self, id: &u64) -> Result<Vec<Comment>> {
        let item = self.find_item(id).await?;

        match item {
            Item::Story(story) => {
                if let Some(kids) = story.kids {
                    if !kids.is_empty() {
                        let comments_futures = kids.iter().map(|id| self.find_item(id));

                        return match join_all(comments_futures)
                            .await
                            .into_iter()
                            .map(|story| story.map_err(Error::from))
                            .collect::<Result<Vec<Item>>>()
                        {
                            Ok(items) => {
                                let comments = items
                                    .into_iter()
                                    .filter_map(|item| match item {
                                        Item::Comment(comment) => Some(comment),
                                        _ => None,
                                    })
                                    .collect::<Vec<Comment>>();

                                Ok(comments)
                            }
                            Err(err) => Err(err),
                        };
                    }
                }

                return Ok(Vec::new());
            }
            _ => Err(Error::new(
                StatusCode::BAD_REQUEST,
                &format!("The provided ID doesn't belong to a story item"),
                None,
            )),
        }
    }

    #[allow(dead_code)]
    async fn find_max_item_id(&self) -> Result<u64> {
        match get(HackerNewsService::uri("/maxitem.json")).await {
            Ok(res) => {
                let text = &res.text().await.map_err(Error::from)?;
                let maxitem_id = serde_json::from_str::<u64>(&text).map_err(Error::from)?;

                Ok(maxitem_id)
            }
            Err(err) => Err(Error::from(err)),
        }
    }

    async fn find_newstories_ids(&self, page: Option<usize>) -> Result<Vec<u64>> {
        let offset = page.unwrap_or(1) * PAGE_SIZE;

        match get(HackerNewsService::uri("/newstories.json")).await {
            Ok(res) => {
                let text = &res.text().await.map_err(Error::from)?;
                let ids = serde_json::from_str::<Vec<u64>>(&text).map_err(Error::from)?;
                let ids = ids.into_iter().skip(offset).take(PAGE_SIZE).collect();

                Ok(ids)
            }
            Err(err) => Err(Error::from(err)),
        }
    }

    async fn find_item(&self, id: &u64) -> Result<Item> {
        match get(HackerNewsService::uri(&format!("/item/{}.json", id))).await {
            Ok(res) => {
                let text = &res.text().await.map_err(Error::from)?;
                let story = serde_json::from_str::<Item>(&text).map_err(Error::from)?;

                Ok(story)
            }
            Err(err) => Err(Error::from(err)),
        }
    }

    fn uri(path: &str) -> String {
        let mut url = BASE_URL.to_string();

        url.push_str(path);
        url
    }
}
