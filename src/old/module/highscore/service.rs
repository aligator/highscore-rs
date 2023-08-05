extern crate diesel;
extern crate rocket;

use std::env;
use diesel::{Connection, RunQueryDsl, SqliteConnection};
use dotenvy::dotenv;
use crate::{model, schema};

struct Service {
    db: r2d2::Pool<ConnectionManager<SqliteConnection>>,
}

impl Service {
    fn new(db: r2d2::Pool<ConnectionManager<SqliteConnection>>) -> Self {
        Self { db }
    }

    pub fn create_highscore(highscore: model::CreateHighscore) {
        diesel::insert_into(schema::highscore::dsl::highscore)
            .values(&highscore)
            .execute(self::db.get().unwrap())
            .expect("Error saving new highscore");
    }
}

/*
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}*/
