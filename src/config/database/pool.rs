use super::{DbMigration, DbPool};
use diesel::{
    r2d2::{self},
    PgConnection,
};

pub async fn create_db_pool() -> DbPool {
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(conn_spec);

    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file")
        .run_migrations()
        .await
}

#[cfg(test)]
pub async fn create_test_db_pool() -> DbPool {
    let database_url: &str = "postgres://postgres:postgres@localhost:5432/postgres";
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file")
        .run_migrations()
        .await
}
