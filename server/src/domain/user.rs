use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const EMAIL_REGEXP: &str = r#"/^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$/"#;

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub surname: Option<String>,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn is_valid_email(email: &str) -> bool {
        Regex::new(EMAIL_REGEXP).unwrap().is_match(email);
        true
    }
}
