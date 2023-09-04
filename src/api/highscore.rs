use rocket::{post, routes, Route, State};

use crate::api::model;
use crate::service::highscore::HighscoreService;

pub fn routes() -> Vec<Route> {
    routes![create_highscore]
}

#[post("/")]
async fn create_highscore(highscore: &State<HighscoreService>) -> &'static str {
    highscore
        .create_highscore(model::CreateHighscore {
            name: "test".to_string(),
            score: 1.0,
        })
        .expect("Error saving new highscore");

    // TODO: return id
    "OK"
}
