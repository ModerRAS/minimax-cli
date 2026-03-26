use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub api_host: String,
    pub db_path: PathBuf,
    pub output_dir: PathBuf,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        // Load .env file if present
        dotenvy::dotenv().ok();

        let api_key =
            env::var("MINIMAX_API_KEY").map_err(|_| ConfigError::Missing("MINIMAX_API_KEY"))?;
        let api_host =
            env::var("MINIMAX_API_HOST").map_err(|_| ConfigError::Missing("MINIMAX_API_HOST"))?;

        let db_path = env::var("MINIMAX_DB_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                dirs::home_dir()
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join(".minimax-cli")
                    .join("tasks.db")
            });

        let output_dir = env::var("MINIMAX_OUTPUT_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("./downloads"));

        Ok(Config {
            api_key,
            api_host,
            db_path,
            output_dir,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Missing required environment variable: {0}")]
    Missing(&'static str),
    #[error("Invalid value for {0}: {1}")]
    Invalid(&'static str, String),
}
