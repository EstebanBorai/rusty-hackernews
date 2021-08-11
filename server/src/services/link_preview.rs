use link_preview::fetch::fetch_partially;
use link_preview::LinkPreview;

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
        if let Ok(html) = fetch_partially(url).await {
            return Some(LinkPreview::from(&html));
        }

        None
    }
}
