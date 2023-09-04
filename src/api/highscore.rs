use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{post, routes, Route, State};

use crate::api::dto::highscore::NewHighscoreDTO;
use crate::model;
use crate::service::highscore::HighscoreService;

pub fn routes() -> Vec<Route> {
    routes![create_highscore]
}

#[post("/", data = "<new_highscore>")]
async fn create_highscore(
    highscore: &State<HighscoreService>,
    new_highscore: Json<NewHighscoreDTO>,
) -> Status {
    highscore
        .create_highscore(model::highscore::CreateHighscore {
            name: new_highscore.name.clone(),
            score: new_highscore.score,
        })
        .expect("Failed to create highscore");

    Status::Created
}
