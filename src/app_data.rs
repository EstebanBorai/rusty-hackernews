use actix_web::web::Data;
use std::sync::Arc;

use crate::services::hacker_news::HackerNewsService;

pub struct AppData {
    pub hacker_news_service: Arc<HackerNewsService>,
}

impl AppData {
    pub async fn new() -> Data<Self> {
        let hacker_news_service = Arc::new(HackerNewsService::new());

        Data::new(AppData {
            hacker_news_service,
        })
    }
}
