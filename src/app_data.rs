use actix_web::web::Data;
use handlebars::Handlebars;
use sass_rs::compile_file;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::services::hacker_news::HackerNewsService;
use crate::services::link_preview::LinkPreviewService;

pub struct StaticResources {
    pub styles: String,
}

pub struct AppData {
    pub hacker_news_service: Arc<Mutex<HackerNewsService>>,
    pub link_preview_service: Arc<Mutex<LinkPreviewService>>,
    pub handlebars: Arc<Handlebars<'static>>,
    pub static_resources: Arc<StaticResources>,
}

impl AppData {
    pub async fn new() -> Data<Self> {
        let link_preview_service = Arc::new(Mutex::new(LinkPreviewService::new(5)));
        let hacker_news_service = Arc::new(Mutex::new(HackerNewsService::new(
            Arc::clone(&link_preview_service),
            10,
        )));
        let mut handlebars = Handlebars::new();
        let static_resources = StaticResources {
            styles: AppData::compile_styles(),
        };

        handlebars
            .register_templates_directory(".hbs", "templates")
            .expect("Failed to register templates directory for Handlebars");

        Data::new(AppData {
            hacker_news_service,
            link_preview_service,
            handlebars: Arc::new(handlebars),
            static_resources: Arc::new(static_resources),
        })
    }

    pub fn compile_styles() -> String {
        compile_file("styles/app.scss", sass_rs::Options::default()).unwrap()
    }
}
