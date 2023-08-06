extern crate rocket;

use crate::config::Config;
use std::error::Error;

mod api;
mod config;
pub mod db;
pub mod schema;

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg = Config::from_env()?;

    rocket::build()
        .mount("/highscore", api::highscore_routes())
        .manage(db::init_pool(cfg))
        .launch()
        .await?;

    Ok(())
}
