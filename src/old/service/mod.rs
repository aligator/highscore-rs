extern crate diesel;
extern crate rocket;

use std::env;
use diesel::{Connection, RunQueryDsl, SqliteConnection};
use dotenvy::dotenv;
use crate::{model, schema};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_highscore(highscore: model::CreateHighscore) {
    let connection = &mut establish_connection();

    diesel::insert_into(schema::highscore::dsl::highscore)
        .values(&highscore)
        .execute(connection)
        .expect("Error saving new highscore");
}
