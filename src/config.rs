//! Configuration management

use dirs::{cache_dir, config_dir};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const CONFIG_FILE: &str = "config.toml";
const INSTRUMENT_CACHE: &str = "instruments.csv";
const CACHE_DIR: &str = "zerodha-cli";

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub api: ApiConfig,
    #[serde(default)]
    pub cache: CacheConfig,
    #[serde(default)]
    pub output: OutputConfig,
    #[serde(default)]
    pub shell: ShellConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            api: ApiConfig::default(),
            cache: CacheConfig::default(),
            output: OutputConfig::default(),
            shell: ShellConfig::default(),
        }
    }
}

/// API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub access_token: Option<String>,
}

impl Default for ApiConfig {
    fn default() -> Self {
        ApiConfig {
            api_key: None,
            api_secret: None,
            access_token: None,
        }
    }
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    #[serde(default = "default_instrument_ttl")]
    pub instrument_ttl_hours: u64,
}

fn default_instrument_ttl() -> u64 {
    24
}

impl Default for CacheConfig {
    fn default() -> Self {
        CacheConfig {
            instrument_ttl_hours: default_instrument_ttl(),
        }
    }
}

/// Output configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    #[serde(default = "default_format")]
    pub default_format: String,
    #[serde(default = "default_color")]
    pub color: bool,
}

fn default_format() -> String {
    "table".to_string()
}

fn default_color() -> bool {
    true
}

impl Default for OutputConfig {
    fn default() -> Self {
        OutputConfig {
            default_format: default_format(),
            color: default_color(),
        }
    }
}

/// Shell configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellConfig {
    #[serde(default = "default_history_size")]
    pub history_size: usize,
}

fn default_history_size() -> usize {
    1000
}

impl Default for ShellConfig {
    fn default() -> Self {
        ShellConfig {
            history_size: default_history_size(),
        }
    }
}

impl Config {
    /// Load configuration from file
    pub fn load() -> Result<Self, String> {
        let config_path = Self::config_path()?;

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)
                .map_err(|e| e.to_string())?;
            let mut config: Config = toml::from_str(&content)
                .map_err(|e| e.to_string())?;
            config.apply_env_overrides();
            Ok(config)
        } else {
            let mut config = Config::default();
            config.apply_env_overrides();
            Ok(config)
        }
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<(), String> {
        let config_path = Self::config_path()?;

        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| e.to_string())?;
        }

        let content = toml::to_string_pretty(self)
            .map_err(|e| e.to_string())?;
        std::fs::write(&config_path, content)
            .map_err(|e| e.to_string())?;

        // Set file permissions to 600
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&config_path)
                .map_err(|e| e.to_string())?
                .permissions();
            perms.set_mode(0o600);
            std::fs::set_permissions(&config_path, perms)
                .map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    /// Get config file path
    pub fn config_path() -> Result<PathBuf, String> {
        let config_dir = config_dir()
            .ok_or("Config directory not found")?;
        Ok(config_dir.join("zerodha-cli").join(CONFIG_FILE))
    }

    /// Get cache directory
    pub fn cache_dir() -> Result<PathBuf, String> {
        let cache_dir = cache_dir()
            .ok_or("Cache directory not found")?;
        Ok(cache_dir.join(CACHE_DIR))
    }

    /// Get instrument cache file path
    pub fn instrument_cache_path() -> Result<PathBuf, String> {
        Ok(Self::cache_dir()?.join(INSTRUMENT_CACHE))
    }

    /// Apply environment variable overrides
    pub fn apply_env_overrides(&mut self) {
        if let Ok(key) = std::env::var("ZERODHA_API_KEY") {
            self.api.api_key = Some(key);
        }
        if let Ok(secret) = std::env::var("ZERODHA_API_SECRET") {
            self.api.api_secret = Some(secret);
        }
        if let Ok(token) = std::env::var("ZERODHA_ACCESS_TOKEN") {
            self.api.access_token = Some(token);
        }
    }

    /// Get access token
    pub fn get_access_token(&self) -> Result<String, String> {
        self.api
            .access_token
            .as_ref()
            .ok_or_else(|| "Missing access token. Run 'kite auth login'".to_string())
            .map(|s| s.clone())
    }
}
