use actix_web::web::Data;
use handlebars::Handlebars;
use sass_rs::compile_file;
use std::sync::{Arc, Mutex};

use crate::services::hacker_news::HackerNewsService;

pub struct StaticResources {
    pub styles: String,
}

pub struct AppData {
    pub hacker_news_service: Arc<Mutex<HackerNewsService>>,
    pub handlebars: Arc<Handlebars<'static>>,
    pub static_resources: Arc<StaticResources>,
}

impl AppData {
    pub async fn new() -> Data<Self> {
        let hacker_news_service = Arc::new(Mutex::new(HackerNewsService::new()));
        let mut handlebars = Handlebars::new();
        let static_resources = StaticResources {
            styles: AppData::compile_styles(),
        };

        handlebars
            .register_templates_directory(".hbs", "templates")
            .expect("Failed to register templates directory for Handlebars");

        Data::new(AppData {
            hacker_news_service,
            handlebars: Arc::new(handlebars),
            static_resources: Arc::new(static_resources),
        })
    }

    pub fn compile_styles() -> String {
        let main_scss_file_path = "src/styles/app.scss";

        compile_file(main_scss_file_path, sass_rs::Options::default())
            .expect("Failed to compile SASS")
    }
}
