use dotenv::dotenv;
use std::env;

pub struct Environment {
    pub database_url: String,
    pub json_web_token_encoding_key: String,
}

impl Environment {
    pub fn new() -> Self {
        if cfg!(debug_assertions) {
            dotenv().ok().expect("Failed to read .env file");
        }

        Environment {
            database_url: Environment::get("DATABASE_URL"),
            json_web_token_encoding_key: Environment::get("JSON_WEB_TOKEN_ENCODING_KEY"),
        }
    }

    fn get(key: &str) -> String {
        env::var(key).expect(&format!("Failed to read environment variable: {}", key))
    }
}
