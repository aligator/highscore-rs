// TODO: https://gist.github.com/sean3z/eec7128ec21f23e14c95f66ac998edf3

use diesel::prelude::*;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use crate::config;

pub struct DbPool {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

pub async fn connect_from_config(cfg: config::Config) -> Result<DbPool, r2d2::PoolError> {
    let manager = ConnectionManager::<SqliteConnection>::new(cfg.database_url);
    let pool = Pool::builder().build(manager)?;
    Ok(DbPool { pool })
}
