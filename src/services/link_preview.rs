use link_preview::LinkPreview;
use reqwest::get;
use std::str::FromStr;

pub struct LinkPreviewService {
    #[allow(dead_code)]
    stream_max_iterations: u8,
}

impl LinkPreviewService {
    pub fn new(stream_max_iterations: u8) -> Self {
        LinkPreviewService {
            stream_max_iterations,
        }
    }

    pub async fn preview_from_url(&self, url: &str) -> Option<LinkPreview> {
        if let Some(html) = self.fetch(url).await {
            return LinkPreview::from_str(html.as_str()).ok();
        }

        None
    }

    async fn fetch(&self, url: &str) -> Option<String> {
        if let Ok(response) = get(url).await {
            return response.text().await.ok();
        }

        None
    }
}
