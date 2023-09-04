use crate::db::DB;
use crate::model;
use crate::schema;
use diesel::RunQueryDsl;

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
    ) -> Result<(), diesel::result::Error> {
        let mut conn = self.db.pool.get().unwrap();

        diesel::insert_into(schema::highscore::dsl::highscore)
            .values(&highscore)
            .execute(&mut conn)?;

        // TODO: get and return the id
        Ok(())
    }
}
