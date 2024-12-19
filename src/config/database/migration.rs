use super::{DbMigration, DbPool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

impl DbMigration for DbPool {
    async fn run_migrations(&self) -> Self {
        match self.get().unwrap().run_pending_migrations(MIGRATIONS) {
            Ok(_) => self.clone(),
            Err(e) => panic!("Failed to run migrations: {}", e),
        };
        self.clone()
    }
}

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");