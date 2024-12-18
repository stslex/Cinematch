use diesel::{
    r2d2::{self},
    PgConnection,
};

use super::DbPool;

pub fn create_db_pool() -> DbPool {
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(conn_spec);
    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file")
}

pub fn create_test_db_pool() -> DbPool {
    let url: &str = "postgres://postgres:postgres@localhost:5432/postgres";
    let conn_spec = std::env::var(url).expect("DATABASE_URL should be set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(conn_spec);
    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file")
}
