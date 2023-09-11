use rocket::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewHighscoreDTO {
    pub name: String,
    pub score: i64,
}
