//! Configuration module

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration file structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api: ApiConfig,
    #[serde(default)]
    pub defaults: DefaultsConfig,
    #[serde(default)]
    pub output: OutputConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub api_key: String,
    pub api_secret: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_expiry: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DefaultsConfig {
    #[serde(default = "default_exchange")]
    pub exchange: String,
    #[serde(default = "default_product")]
    pub product: String,
    #[serde(default = "default_order_type")]
    pub order_type: String,
    #[serde(default = "default_validity")]
    pub validity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OutputConfig {
    #[serde(default = "default_format")]
    pub format: String,
}

fn default_exchange() -> String { "NSE".to_string() }
fn default_product() -> String { "CNC".to_string() }
fn default_order_type() -> String { "LIMIT".to_string() }
fn default_validity() -> String { "DAY".to_string() }
fn default_format() -> String { "table".to_string() }

impl Default for Config {
    fn default() -> Self {
        Self {
            api: ApiConfig {
                api_key: String::new(),
                api_secret: String::new(),
                access_token: None,
                token_expiry: None,
            },
            defaults: DefaultsConfig::default(),
            output: OutputConfig::default(),
        }
    }
}

impl Config {
    /// Load config from file
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path().context("Failed to get config path")?;
        
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path).context("Failed to read config")?;
            let config: Self = toml::from_str(&content).context("Failed to parse config")?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }

    /// Save config to file
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path().context("Failed to get config path")?;
        let config_dir = config_path.parent()
            .ok_or_else(|| anyhow::anyhow!("Invalid config path"))?;

        std::fs::create_dir_all(config_dir).context("Failed to create config directory")?;
        let content = toml::to_string_pretty(self).context("Failed to serialize config")?;
        std::fs::write(&config_path, content).context("Failed to write config")?;

        Ok(())
    }

    /// Get config file path
    pub fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Failed to get config directory"))?;
        Ok(config_dir.join("zerodha-cli").join("config.toml"))
    }

    /// Check if token is valid
    pub fn is_token_valid(&self) -> bool {
        if let Some(expiry_str) = &self.api.token_expiry {
            if let Ok(expiry) = chrono::DateTime::parse_from_rfc3339(expiry_str) {
                return expiry.with_timezone(&chrono::Utc) > chrono::Utc::now();
            }
        }
        false
    }
}
