use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewHighscoreDTO {
    pub name: String,
    pub score: i64,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct HighscoreDTO {
    pub id: i32,
    pub name: String,
    pub score: i64,
    #[serde(with = "crate::serde::iso8601")]
    pub created_at: time::OffsetDateTime,
}
