extern crate rocket;

use std::error::Error;

use log::info;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Orbit, Rocket};
use rocket_okapi::rapidoc::{make_rapidoc, GeneralConfig, HideShowConfig, RapiDocConfig};
use rocket_okapi::settings::UrlObject;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

use crate::config::Config;

mod api;
mod config;
pub mod db;
pub mod format;
pub mod model;
pub mod schema;
pub mod service;

struct StartupFairing;

#[rocket::async_trait]
impl Fairing for StartupFairing {
    fn info(&self) -> Info {
        Info {
            name: "Startup Fairing",
            kind: Kind::Liftoff,
        }
    }

    async fn on_liftoff(&self, rocket: &Rocket<Orbit>) {
        info!(
            "📝 API docu: http://{}:{}/swagger-ui/",
            rocket.config().address,
            rocket.config().port
        );
    }
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg = Config::from_env()?;

    let db = db::init_pool(&cfg);

    // Setup the services.
    let highscore_service = service::highscore::HighscoreService::new(db);

    let cors = rocket_cors::CorsOptions::default().to_cors()?;

    rocket::build()
        .mount("/", api::highscore_routes())
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
        .manage(highscore_service)
        .attach(cors)
        .attach(StartupFairing {})
        .launch()
        .await?;

    Ok(())
}
