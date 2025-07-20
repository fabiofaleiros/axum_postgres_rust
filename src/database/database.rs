use sqlx::{postgres::PgPoolOptions, PgPool};
use crate::config::Config;

/// Database connection management
pub struct Database;

impl Database {
    /// Creates a new database connection pool
    pub async fn connect(config: &Config) -> Result<PgPool, sqlx::Error> {
        PgPoolOptions::new()
            .max_connections(config.max_connections)
            .connect(&config.database_url)
            .await
    }
}