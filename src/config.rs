use std::env;
use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Missing required configuration: {0}")]
    Missing(&'static str),
    #[error("Invalid value for {0}: {1}")]
    Invalid(&'static str, String),
    #[error("Failed to load config file: {0}")]
    ConfigFileError(#[from] crate::config_file::ConfigFileError),
    #[error("Failed to access keyring: {0}")]
    KeyringError(#[from] crate::keyring::KeyringError),
}

#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub api_host: String,
    pub db_path: PathBuf,
    pub output_dir: PathBuf,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let config_file = crate::config_file::ConfigFile::load()?;

        let api_key = Self::get_api_key_with_migration()?;

        Ok(Config {
            api_key,
            api_host: config_file.api_host,
            db_path: config_file.db_path,
            output_dir: config_file.output_dir,
        })
    }

    fn get_api_key_with_migration() -> Result<String, ConfigError> {
        match crate::keyring::get_api_key() {
            Ok(key) => Ok(key),
            Err(crate::keyring::KeyringError::NotFound) => Self::migrate_from_env(),
            Err(e) => Err(ConfigError::KeyringError(e)),
        }
    }

    fn migrate_from_env() -> Result<String, ConfigError> {
        dotenvy::dotenv().ok();

        env::var("MINIMAX_API_KEY").map_err(|_| ConfigError::Missing("MINIMAX_API_KEY"))
    }

    pub fn api_key_is_set() -> bool {
        crate::keyring::get_api_key().is_ok()
    }

    pub fn config_file_path() -> PathBuf {
        crate::config_file::ConfigFile::path()
    }
}
