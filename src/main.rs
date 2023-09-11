extern crate rocket;

use crate::config::Config;
use std::error::Error;

mod api;
mod config;
pub mod db;
pub mod model;
pub mod schema;
pub mod serde;
pub mod service;

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg = Config::from_env()?;
    let db = db::init_pool(cfg);

    // Setup the services.
    let highscore_service = service::highscore::HighscoreService::new(db);

    rocket::build()
        .mount("/highscore", api::highscore_routes())
        .manage(highscore_service)
        .launch()
        .await?;

    Ok(())
}
