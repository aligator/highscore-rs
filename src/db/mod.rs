// TODO: https://gist.github.com/sean3z/eec7128ec21f23e14c95f66ac998edf3

use crate::config;
use diesel::prelude::*;
use diesel::r2d2;
use diesel::r2d2::{ConnectionManager, ManageConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::error::Error;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub struct DB {
    pub pool: r2d2::Pool<ConnectionManager<SqliteConnection>>,
}

fn run_migrations(database_url: String) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let mut conn = manager.connect().expect("db connection");

    conn.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}

pub fn init_pool(cfg: &config::Config) -> DB {
    run_migrations(cfg.database_url.clone()).expect("db migrations");

    let manager = ConnectionManager::<SqliteConnection>::new(&cfg.database_url);
    let db = DB {
        pool: r2d2::Pool::new(manager).expect("db pool"),
    };

    db
}
