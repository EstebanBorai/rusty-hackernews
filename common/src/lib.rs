use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Story {
    pub id: u64,
    pub r#type: String,
    pub by: String,
    pub descendants: u32,
    pub kids: Option<Vec<u64>>,
    pub score: u32,
    pub time: u64,
    pub title: String,
    pub url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LinkPreview {
    pub title: Option<String>,
    pub image_url: Option<String>,
    pub description: Option<String>,
}
