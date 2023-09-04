use rocket::serde::Serialize;
use serde::Deserialize;

pub mod highscore;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct IdDTO {
    pub id: i32,
}
