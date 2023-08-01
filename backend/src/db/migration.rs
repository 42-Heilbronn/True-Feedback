use diesel::{Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn run_migration(database_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = PgConnection::establish(database_url)?;
    conn.run_pending_migrations(MIGRATIONS).expect("failed to run migration");

    Ok(())
}
