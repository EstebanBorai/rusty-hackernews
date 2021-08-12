use dotenv::dotenv;
use std::env;

pub struct Environment {
    pub database_url: String,
}

impl Environment {
    pub fn new() -> Self {
        if cfg!(debug_assertions) {
            dotenv().ok().expect("Failed to read .env file");
        }

        Environment {
            database_url: Environment::get("DATABASE_URL"),
        }
    }

    fn get(key: &str) -> String {
        env::var(key).expect(&format!("Failed to read environment variable: {}", key))
    }
}
