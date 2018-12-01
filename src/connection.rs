use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};

pub type ConnectionPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> ConnectionPool {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("Environment variable DATABASE_URL must be set");

    Pool::new(ConnectionManager::new(database_url))
        .expect("Cannot build connection pool")
}
