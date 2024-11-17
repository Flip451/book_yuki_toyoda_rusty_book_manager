pub mod model;

use shared::config::DatabaseConfig;
use sqlx::{postgres::PgConnectOptions, PgPool};

fn make_pg_connect_options(config: &DatabaseConfig) -> PgConnectOptions {
    PgConnectOptions::new()
        .host(&config.host)
        .port(config.port)
        .username(&config.username)
        .password(&config.password)
        .database(&config.database)
}

#[derive(Clone)]
pub struct ConnectionPool(PgPool);

impl ConnectionPool {
    pub fn inner_ref(&self) -> &PgPool {
        &self.0
    }
}

pub fn connect_database_with(config: &DatabaseConfig) -> ConnectionPool {
    let options = make_pg_connect_options(config);
    let pool = PgPool::connect_lazy_with(options);
    ConnectionPool(pool)
}
