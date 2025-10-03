use crate::iprintln;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

const CONFIG_FILE: &str = "astrbot.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub token: String,
    pub server_url: String,
    pub username: String,
}

pub struct ConfigManager;

impl ConfigManager {
    /// Get the config directory path
    fn get_config_dir() -> anyhow::Result<PathBuf> {
        #[cfg(target_os = "windows")]
        let home_dir =
            env::var("USERPROFILE").context("USERPROFILE environment variable not set")?;

        #[cfg(not(target_os = "windows"))]
        let home_dir = env::var("HOME").context("HOME environment variable not set")?;

        Ok(PathBuf::from(home_dir))
    }

    /// Get the config file path
    fn get_config_file_path() -> anyhow::Result<PathBuf> {
        Ok(Self::get_config_dir()?.join(CONFIG_FILE))
    }

    /// Save credentials to config file
    pub fn save_credentials(credentials: &Config) -> anyhow::Result<()> {
        let config_dir = Self::get_config_dir()?;

        // Create config directory if it doesn't exist
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
        }

        let config_file = Self::get_config_file_path()?;
        let json_content = serde_json::to_string_pretty(credentials)?;

        fs::write(&config_file, json_content)?;

        iprintln!("Credentials saved to: {}", config_file.display());
        Ok(())
    }

    /// Load credentials from config file
    pub fn load_credentials() -> anyhow::Result<Config> {
        let config_file = Self::get_config_file_path()?;

        if !config_file.exists() {
            anyhow::bail!("Config file does not exist");
        }

        let content = fs::read_to_string(&config_file)
            .with_context(|| format!("Failed to read config file: {}", config_file.display()))?;

        let credentials: Config = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", config_file.display()))?;
        Ok(credentials)
    }
}
