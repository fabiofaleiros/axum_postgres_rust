use serde::Deserialize;

/// Application configuration
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server_address: String,
    pub database_url: String,
    pub max_connections: u32,
}

impl Config {
    /// Loads configuration from environment variables
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenvy::dotenv().ok();
        
        Ok(Self {
            server_address: std::env::var("SERVER_ADDRESS")
                .unwrap_or_else(|_| "127.0.0.1:7878".to_string()),
            database_url: std::env::var("DATABASE_URL")
                .map_err(|_| "DATABASE_URL environment variable is required")?,
            max_connections: std::env::var("MAX_DB_CONNECTIONS")
                .unwrap_or_else(|_| "16".to_string())
                .parse()
                .unwrap_or(16),
        })
    }
}