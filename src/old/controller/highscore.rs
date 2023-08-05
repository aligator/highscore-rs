
#[derive(Serialize, Deserialize)]
pub struct NewHighscore {
    pub name: String,
    pub score: f32,
}

#[post("/highscore", data = "<highscore>")]
pub fn create_highscore(highscore: Json<NewHighscore>) {
    service::create_highscore(model::CreateHighscore {
        name: highscore.name.clone(),
        score: highscore.score,
    });
}