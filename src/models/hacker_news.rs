use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Story {
    id: u64,
    r#type: String,
    by: String,
    descendants: u32,
    kids: Option<Vec<u64>>,
    score: u32,
    time: u64,
    title: String,
    url: Option<String>,
}
