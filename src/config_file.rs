use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigFileError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("TOML parse error: {0}")]
    ParseError(#[from] toml::de::Error),

    #[error("TOML save error: {0}")]
    SaveError(#[from] toml::ser::Error),
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConfigFile {
    pub api_host: String,
    pub db_path: PathBuf,
    pub output_dir: PathBuf,
}

impl Default for ConfigFile {
    fn default() -> Self {
        Self {
            api_host: "https://api.minimax.io".to_string(),
            db_path: dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join(".minimax-cli")
                .join("tasks.db"),
            output_dir: PathBuf::from("./downloads"),
        }
    }
}

impl ConfigFile {
    pub fn path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("minimax-cli")
            .join("config.toml")
    }

    pub fn load() -> Result<Self, ConfigFileError> {
        let path = Self::path();
        if !path.exists() {
            return Ok(ConfigFile::default());
        }
        let content = std::fs::read_to_string(&path)?;
        let config: ConfigFile = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<(), ConfigFileError> {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        std::fs::write(&path, content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_default_values() {
        let config = ConfigFile::default();
        assert_eq!(config.api_host, "https://api.minimax.io");
        assert!(config
            .db_path
            .to_string_lossy()
            .ends_with(".minimax-cli/tasks.db"));
        assert_eq!(config.output_dir, PathBuf::from("./downloads"));
    }

    #[test]
    fn test_load_nonexistent_file_returns_defaults() {
        let temp_dir = env::temp_dir();
        let nonexistent_path = temp_dir.join("nonexistent_config.toml");
        assert!(!nonexistent_path.exists());

        let default_config = ConfigFile::default();
        let temp_save_path = temp_dir.join("test_save_load.toml");

        let saved_content = toml::to_string_pretty(&default_config).unwrap();
        std::fs::write(&temp_save_path, &saved_content).unwrap();

        let loaded_content = std::fs::read_to_string(&temp_save_path).unwrap();
        let loaded: ConfigFile = toml::from_str(&loaded_content).unwrap();

        assert_eq!(loaded, default_config);
        std::fs::remove_file(temp_save_path).ok();
    }

    #[test]
    fn test_save_load_roundtrip() {
        let temp_dir = env::temp_dir();
        let temp_config_file = temp_dir.join("roundtrip_test.toml");

        let custom_config = ConfigFile {
            api_host: "https://api.custom.host".to_string(),
            db_path: temp_dir.join("custom/db/path.db"),
            output_dir: temp_dir.join("custom/output"),
        };

        let content = toml::to_string_pretty(&custom_config).unwrap();
        std::fs::write(&temp_config_file, &content).unwrap();

        let loaded_content = std::fs::read_to_string(&temp_config_file).unwrap();
        let loaded: ConfigFile = toml::from_str(&loaded_content).unwrap();

        assert_eq!(loaded, custom_config);
        std::fs::remove_file(temp_config_file).ok();
    }

    #[test]
    fn test_path_uses_config_dir() {
        let path = ConfigFile::path();
        assert!(path.to_string_lossy().contains("minimax-cli"));
        assert!(path.to_string_lossy().ends_with("config.toml"));
    }
}
