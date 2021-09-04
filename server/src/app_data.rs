use actix_web::web::Data;
use sqlx::postgres::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::environment::Environment;
use crate::services::hacker_news::HackerNewsService;
use crate::services::link_preview::LinkPreviewService;
use crate::services::user::UserService;

pub struct AppData {
    pub hacker_news_service: Arc<Mutex<HackerNewsService>>,
    pub link_preview_service: Arc<Mutex<LinkPreviewService>>,
    pub users_service: Arc<Mutex<UserService>>,
    pub database_pool: Arc<PgPool>,
    #[allow(dead_code)]
    environment: Arc<Environment>,
}

impl AppData {
    pub async fn new() -> Data<Self> {
        let environment = Arc::new(Environment::new());
        let database_pool = Arc::new(AppData::make_db_pool(Arc::clone(&environment)).await);
        let link_preview_service = Arc::new(Mutex::new(LinkPreviewService::new(Arc::clone(
            &database_pool,
        ))));
        let hacker_news_service = Arc::new(Mutex::new(HackerNewsService::new()));
        let users_service = Arc::new(Mutex::new(UserService::new(
            Arc::clone(&database_pool),
            Arc::clone(&environment),
        )));

        Data::new(AppData {
            hacker_news_service,
            link_preview_service,
            users_service,
            database_pool,
            environment,
        })
    }

    async fn make_db_pool(environment: Arc<Environment>) -> PgPool {
        PgPool::connect(environment.database_url.as_str())
            .await
            .expect("Failed to create database connection pool instance")
    }
}
