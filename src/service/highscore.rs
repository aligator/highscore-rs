use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl};

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

        let inserted = diesel::insert_into(schema::highscore::dsl::highscore)
            .values(&highscore)
            .returning(schema::highscore::dsl::id)
            .get_result(&mut conn)?;

        Ok(inserted)
    }

    pub fn get_highscores(
        &self,
        game: String,
        page: i64,
        page_size: i64,
    ) -> Result<(Vec<model::highscore::Highscore>, i64), diesel::result::Error> {
        let mut conn = self.db.pool.get().unwrap();

        conn.transaction(|connection| {
            let highscores = schema::highscore::dsl::highscore
                .select((
                    schema::highscore::dsl::id,
                    schema::highscore::dsl::name,
                    schema::highscore::dsl::score,
                    schema::highscore::dsl::created_at,
                ))
                .filter(schema::highscore::dsl::game.eq(game.clone()))
                .order(schema::highscore::dsl::score.desc())
                .limit(page_size)
                .offset(page * page_size)
                .load::<model::highscore::Highscore>(connection)?;

            let total = schema::highscore::dsl::highscore
                .filter(schema::highscore::dsl::game.eq(game))
                .count()
                .get_result(connection)?;

            Ok((highscores, total))
        })
    }
}
