use chrono::{DateTime, Utc};
use link_preview::fetch::fetch_partially;
use link_preview::html::remove_html_tags;
use link_preview::LinkPreview;
use sqlx::{query_as, FromRow, PgPool};
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

use crate::error::{Error, Result};

pub struct LinkPreviewService {
    database_pool: Arc<PgPool>,
}

#[derive(FromRow)]
struct PreviewsRow {
    id: Uuid,
    title: Option<String>,
    description: Option<String>,
    domain: Option<String>,
    url: String,
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
            // TODO: Implement support for domain
            domain: None,
            image_url,
        }
    }
}

impl LinkPreviewService {
    pub fn new(database_pool: Arc<PgPool>) -> Self {
        LinkPreviewService { database_pool }
    }

    pub async fn preview_from_url(&self, url: &str) -> Option<LinkPreview> {
        if let Ok(response) = self.find_preview(url).await {
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

            if let Err(err) = self.store_preview(url, &link_preview).await {
                eprintln!("An error ocurred storing the link preview:\n{:?}", err);
            }

            return Some(link_preview);
        }

        None
    }

    async fn find_preview(&self, url: &str) -> Result<Option<LinkPreview>> {
        let row: PreviewsRow = query_as("SELECT * FROM previews WHERE url = $1")
            .bind(url)
            .fetch_one(&*self.database_pool)
            .await
            .map_err(Error::from)?;

        let link_preview = LinkPreview::from(row);

        Ok(Some(link_preview))
    }

    async fn store_preview(&self, url: &str, link_preview: &LinkPreview) -> Result<LinkPreview> {
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
            url,
            image_url
        ) VALUES (
            $1,
            $2,
            $3,
            $4
        ) RETURNING *
        "#,
        )
        .bind(link_preview.title.clone())
        .bind(link_preview.description.clone())
        .bind(url)
        .bind(image_url)
        .fetch_one(&*self.database_pool)
        .await?;

        Ok(LinkPreview::from(row))
    }
}
