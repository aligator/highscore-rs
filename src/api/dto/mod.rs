use rocket::serde::Serialize;
use rocket_okapi::JsonSchema;
use serde::Deserialize;

pub mod highscore;

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde", deny_unknown_fields)]
pub struct IdDTO {
    pub id: i32,
}
