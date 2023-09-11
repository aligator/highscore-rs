use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::highscore)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CreateHighscore {
    pub name: String,
    pub game: String,
    pub score: i64,
}

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::highscore)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Highscore {
    pub id: i32,
    pub name: String,
    pub score: i64,
    pub created_at: PrimitiveDateTime,
}
