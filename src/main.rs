extern crate rocket;

use crate::config::Config;
use std::error::Error;

mod config;
mod db;

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg = Config::from_env()?;
    let pool = db::connect_from_config(cfg).await?;

    rocket::build().launch().await?;

    Ok(())
}
