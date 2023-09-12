use std::env;

use anyhow::Result;
use dotenvy::dotenv;

pub struct Config {
    pub database_url: String,
    pub json_logger: bool,
}

fn var(key: &str) -> Result<String> {
    env::var(key).map_err(|err| anyhow::anyhow!("env '{}' could not be read: {}", key, err))
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv().ok();
        let database_url = var("DATABASE_URL")?;
        let json_logger = var("JSON_LOGGER").unwrap_or("0".to_string()) == "1";
        Ok(Config {
            database_url,
            json_logger,
        })
    }
}
