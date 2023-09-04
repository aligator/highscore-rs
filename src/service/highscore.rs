use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::db::DB;
use crate::model;
use crate::schema;

pub struct HighscoreService {
    db: DB,
}

impl HighscoreService {
    pub fn new(db: DB) -> Self {
        Self { db }
    }

    pub fn create_highscore(
        &self,
        highscore: model::highscore::CreateHighscore,
    ) -> Result<i32, diesel::result::Error> {
        let mut conn = self.db.pool.get().unwrap();

        let inserted: i32 = diesel::insert_into(schema::highscore::dsl::highscore)
            .values(&highscore)
            .returning(schema::highscore::dsl::id)
            .get_result(&mut conn)?;

        Ok(inserted)
    }

    pub fn get_highscores(
        &self,
        page: i64,
        page_size: i64,
    ) -> Result<Vec<model::highscore::Highscore>, diesel::result::Error> {
        let mut conn = self.db.pool.get().unwrap();

        let highscores = schema::highscore::dsl::highscore
            .order(schema::highscore::dsl::score.desc())
            .limit(page_size)
            .offset(page * page_size)
            .load::<model::highscore::Highscore>(&mut conn)?;

        Ok(highscores)
    }
}
