//! Configuration management using confy.
//!
//! Config file location:
//! - Linux/macOS: ~/.config/earningsfeed/config.toml
//! - Windows: C:\Users\<user>\AppData\Roaming\earningsfeed\config.toml

use serde::{Deserialize, Serialize};

use crate::error::{CliError, Result};

const APP_NAME: &str = "earningsfeed";

/// Configuration stored in the config file.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    /// API key for EarningsFeed API.
    #[serde(default)]
    pub api_key: Option<String>,
}

impl Config {
    /// Load config from file.
    pub fn load() -> Result<Self> {
        let cfg: Config = confy::load(APP_NAME, None)?;
        Ok(cfg)
    }

    /// Save config to file.
    pub fn save(&self) -> Result<()> {
        confy::store(APP_NAME, None, self)?;
        Ok(())
    }

    /// Get the config file path.
    pub fn path() -> Result<std::path::PathBuf> {
        confy::get_configuration_file_path(APP_NAME, None)
            .map_err(|e| CliError::Config(e.to_string()))
    }

    /// Get the API key, returning an error if not set.
    pub fn require_api_key(&self) -> Result<&str> {
        self.api_key.as_deref().ok_or_else(|| {
            CliError::Authentication(
                "not authenticated. Run 'earningsfeed auth login' first.".to_string(),
            )
        })
    }
}
