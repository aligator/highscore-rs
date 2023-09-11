use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, post, Route, State};
use rocket_okapi::{openapi, openapi_get_routes};

use crate::api::dto::highscore::{HighscoreDTO, NewHighscoreDTO};
use crate::api::dto::IdDTO;
use crate::model;
use crate::service::highscore::HighscoreService;

pub fn routes() -> Vec<Route> {
    openapi_get_routes![create_highscore, get_highscores]
}

/// # Create a new highscore
///
/// Returns the id of the newly created highscore.
#[openapi(tag = "Highscore")]
#[post("/highscore", data = "<new_highscore>")]
async fn create_highscore(
    highscore: &State<HighscoreService>,
    new_highscore: Json<NewHighscoreDTO>,
) -> (Status, Json<IdDTO>) {
    let new_id = highscore
        .create_highscore(model::highscore::CreateHighscore {
            name: new_highscore.name.clone(),
            score: new_highscore.score,
        })
        .expect("Failed to create highscore");

    (Status::Created, Json(IdDTO { id: new_id }))
}

/// # Get the highscore
///
/// Returns a list of highscore entries.
#[openapi(tag = "Highscore")]
#[get("/highscore?<page>&<page_size>")]
async fn get_highscores(
    highscore: &State<HighscoreService>,
    page: i64,
    page_size: i64,
) -> (Status, Json<Vec<HighscoreDTO>>) {
    let highscores = highscore
        .get_highscores(page, page_size)
        .expect("Failed to get highscore");

    (
        Status::Ok,
        Json(
            highscores
                .iter()
                .map(|score| HighscoreDTO {
                    id: score.id,
                    name: score.name.clone(),
                    score: score.score,
                    created_at: score
                        .created_at
                        .assume_offset(time::UtcOffset::UTC)
                        .format(&crate::serde::ISO8601_FORMAT)
                        .expect("Failed to format date"),
                })
                .collect::<Vec<HighscoreDTO>>(),
        ),
    )
}
