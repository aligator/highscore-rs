use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::highscore)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CreateHighscore {
    pub name: String,
    pub score: i32,
}
