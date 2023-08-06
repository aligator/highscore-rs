use crate::api::model;
use crate::db::DB;
use crate::schema;
use diesel::RunQueryDsl;
use rocket::{post, routes, Route, State};

pub fn routes() -> Vec<Route> {
    routes![create_highscore]
}

#[post("/")]
async fn create_highscore(db: &State<DB>) -> &'static str {
    let highscore = model::CreateHighscore {
        name: "test".to_string(),
        score: 1.0,
    };

    let mut conn = db.pool.get().unwrap();

    diesel::insert_into(schema::highscore::dsl::highscore)
        .values(&highscore)
        .execute(&mut conn)
        .expect("Error saving new highscore");
    "Hello, world!"
}
