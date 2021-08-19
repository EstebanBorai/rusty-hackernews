use chrono::{DateTime, Utc};
use link_preview::fetch::fetch_partially;
use link_preview::html::remove_html_tags;
use link_preview::LinkPreview;
use sqlx::{query_as, FromRow, PgPool};
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::utils::sha256;

pub struct LinkPreviewService {
    database_pool: Arc<PgPool>,
}

#[derive(Debug, FromRow)]
struct PreviewsRow {
    id: Uuid,
    title: Option<String>,
    description: Option<String>,
    domain: Option<String>,
    url_hash: String,
    image_url: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<PreviewsRow> for LinkPreview {
    fn from(row: PreviewsRow) -> Self {
        let image_url = if row.image_url.is_some() {
            let image_url = row.image_url.unwrap();

            Some(reqwest::Url::from_str(image_url.as_str()).unwrap())
        } else {
            None
        };

        LinkPreview {
            title: row.title,
            description: row.description,
            domain: row.domain,
            image_url,
        }
    }
}

impl LinkPreviewService {
    pub fn new(database_pool: Arc<PgPool>) -> Self {
        LinkPreviewService { database_pool }
    }

    pub async fn preview_from_url(&self, url: &str) -> Option<LinkPreview> {
        let url_hash = sha256::hash(url);

        if let Ok(response) = self.find_preview(&url_hash).await {
            if let Some(link_preview) = response {
                return Some(link_preview);
            }
        }

        if let Ok(html) = fetch_partially(url).await {
            let mut link_preview = LinkPreview::from(&html);

            if let Some(description) = link_preview.description {
                // if a description is available in the `LinkPreview` instance
                // attempt to remove any existent HTML tags and replace the
                // current description value with the sanitized version
                link_preview.description = Some(remove_html_tags(description.as_str()));
            }

            if let Err(err) = self.store_preview(&url_hash, &link_preview).await {
                eprintln!("An error ocurred storing the link preview:\n{:?}", err);
            }

            return Some(link_preview);
        }

        None
    }

    async fn find_preview(&self, url_hash: &str) -> Result<Option<LinkPreview>> {
        let row: PreviewsRow = query_as("SELECT * FROM previews WHERE url_hash = $1")
            .bind(url_hash)
            .fetch_one(&*self.database_pool)
            .await
            .map_err(Error::from)?;

        let link_preview = LinkPreview::from(row);

        Ok(Some(link_preview))
    }

    async fn store_preview(
        &self,
        url_hash: &str,
        link_preview: &LinkPreview,
    ) -> Result<LinkPreview> {
        let image_url = if link_preview.image_url.is_some() {
            let image_url = link_preview.image_url.clone().unwrap();

            Some(image_url.to_string())
        } else {
            None
        };

        let row: PreviewsRow = sqlx::query_as(
            r#"
        INSERT INTO previews (
            title,
            description,
            domain,
            url_hash,
            image_url
        ) VALUES (
            $1,
            $2,
            $3,
            $4,
            $5
        ) RETURNING *
        "#,
        )
        .bind(link_preview.title.clone())
        .bind(link_preview.description.clone())
        .bind(link_preview.domain.clone())
        .bind(url_hash)
        .bind(image_url)
        .fetch_one(&*self.database_pool)
        .await?;

        Ok(LinkPreview::from(row))
    }
}
