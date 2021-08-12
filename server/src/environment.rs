use dotenv::dotenv;
use std::env;

pub struct Environment {
    pub database_url: String,
    pub postgres_user: String,
    pub postgres_password: String,
    pub postgres_db: String,
}

impl Environment {
    pub fn new() -> Self {
        if cfg!(debug_assertions) {
            dotenv().ok().expect("Failed to read .env file");
        }

        Environment {
            database_url: Environment::get("DATABASE_URL"),
            postgres_user: Environment::get("POSTGRES_USER"),
            postgres_password: Environment::get("POSTGRES_PASSWORD"),
            postgres_db: Environment::get("POSTGRES_DB"),
        }
    }

    fn get(key: &str) -> String {
        env::var(key).expect(&format!("Failed to read environment variable: {}", key))
    }
}
