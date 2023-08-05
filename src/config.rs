use std::env;
use dotenvy::dotenv;
use anyhow::Result;

pub struct Config {
    pub database_url: String,
}

fn var(key: &str) -> Result<String> {
    env::var(key).map_err(|err| anyhow::anyhow!("env '{}' could not be read: {}", key, err))
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv().ok();
        let database_url = var("HIGHSCORE_DATABASE_URL")?;
        Ok(Config { database_url })
    }
}