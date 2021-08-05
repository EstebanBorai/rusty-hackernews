use actix_web::web::Data;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::services::hacker_news::HackerNewsService;
use crate::services::link_preview::LinkPreviewService;

pub struct AppData {
    pub hacker_news_service: Arc<Mutex<HackerNewsService>>,
    pub link_preview_service: Arc<Mutex<LinkPreviewService>>,
}

impl AppData {
    pub async fn new() -> Data<Self> {
        let link_preview_service = Arc::new(Mutex::new(LinkPreviewService::new(5)));
        let hacker_news_service = Arc::new(Mutex::new(HackerNewsService::new()));

        Data::new(AppData {
            hacker_news_service,
            link_preview_service,
        })
    }
}
