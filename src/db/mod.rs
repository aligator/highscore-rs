// TODO: https://gist.github.com/sean3z/eec7128ec21f23e14c95f66ac998edf3

use crate::config;
use diesel::prelude::*;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;

pub struct DB {
    pub pool: r2d2::Pool<ConnectionManager<SqliteConnection>>,
}

pub fn init_pool(cfg: config::Config) -> DB {
    let manager = ConnectionManager::<SqliteConnection>::new(cfg.database_url);
    DB {
        pool: r2d2::Pool::new(manager).expect("db pool"),
    }
}
