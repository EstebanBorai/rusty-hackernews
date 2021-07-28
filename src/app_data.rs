use actix_web::web::Data;
use handlebars::Handlebars;
use std::sync::{Arc, Mutex};

use crate::services::hacker_news::HackerNewsService;

pub struct AppData {
    pub hacker_news_service: Arc<Mutex<HackerNewsService>>,
    pub handlebars: Arc<Handlebars<'static>>,
}

impl AppData {
    pub async fn new() -> Data<Self> {
        let hacker_news_service = Arc::new(Mutex::new(HackerNewsService::new()));
        let mut handlebars = Handlebars::new();

        handlebars
            .register_templates_directory(".hbs", "templates")
            .expect("Failed to register templates directory for Handlebars");

        Data::new(AppData {
            hacker_news_service,
            handlebars: Arc::new(handlebars),
        })
    }
}
