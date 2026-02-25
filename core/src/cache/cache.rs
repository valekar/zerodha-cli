//! Instrument cache

use crate::models::Instrument;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use std::fs;
use std::path::PathBuf;

/// Instrument cache manager
pub struct InstrumentCache;

impl InstrumentCache {
    /// Get cache directory
    pub fn cache_dir() -> Result<PathBuf> {
        let cache_dir =
            dirs::cache_dir().ok_or_else(|| anyhow::anyhow!("Failed to get cache directory"))?;
        let dir = cache_dir.join("zerodha-cli").join("instruments");

        // Create directory if it doesn't exist
        fs::create_dir_all(&dir).context("Failed to create cache directory")?;

        Ok(dir)
    }

    /// Get cache file path for exchange
    pub fn cache_file(exchange: &str) -> Result<PathBuf> {
        let cache_dir = Self::cache_dir()?;
        Ok(cache_dir.join(format!("{}.csv", exchange.to_lowercase())))
    }

    /// Get cache file path with date
    pub fn cache_file_with_date(exchange: &str, date: DateTime<Utc>) -> Result<PathBuf> {
        let cache_dir = Self::cache_dir()?;
        let date_str = date.format("%Y-%m-%d");
        Ok(cache_dir.join(format!("{}_{}.csv", exchange.to_lowercase(), date_str)))
    }

    /// Check if cache is valid (not expired)
    pub fn is_valid(exchange: &str) -> Result<bool> {
        let cache_file = Self::cache_file(exchange)?;

        if !cache_file.exists() {
            return Ok(false);
        }

        // Check modification time (max 24h)
        let metadata = fs::metadata(&cache_file).context("Failed to read cache metadata")?;
        let modified = metadata
            .modified()
            .context("Failed to get modification time")?;

        let modified_time: DateTime<Utc> = modified.into();
        let now = Utc::now();
        let age = now - modified_time;

        // Cache is valid if less than 24 hours old
        Ok(age.num_hours() < 24)
    }

    /// Load instruments from cache
    pub fn load(exchange: &str) -> Result<Vec<Instrument>> {
        let cache_file = Self::cache_file(exchange)?;

        if !cache_file.exists() {
            anyhow::bail!("Cache file not found for exchange: {}", exchange);
        }

        let mut rdr = csv::Reader::from_path(&cache_file).context("Failed to open cache file")?;
        let mut instruments = Vec::new();

        for result in rdr.deserialize() {
            let instrument: Instrument = result.context("Failed to parse instrument from cache")?;
            instruments.push(instrument);
        }

        Ok(instruments)
    }

    /// Save instruments to cache
    pub fn save(exchange: &str, instruments: &[Instrument]) -> Result<()> {
        let cache_file = Self::cache_file(exchange)?;

        // Create parent directory if needed
        if let Some(parent) = cache_file.parent() {
            fs::create_dir_all(parent).context("Failed to create cache parent directory")?;
        }

        let mut wtr = csv::Writer::from_path(&cache_file).context("Failed to create cache file")?;

        for instrument in instruments {
            wtr.serialize(instrument)
                .context("Failed to serialize instrument to cache")?;
        }

        wtr.flush().context("Failed to write cache file")?;

        Ok(())
    }

    /// Refresh cache by fetching from API and saving
    pub async fn refresh(
        exchange: &str,
        api_client: &crate::api::KiteConnectClient,
    ) -> Result<Vec<Instrument>> {
        println!("Fetching instruments for {}...", exchange);

        let instruments = api_client
            .list_instruments(Some(exchange))
            .await
            .context("Failed to fetch instruments from API")?;

        println!("Found {} instruments", instruments.len());

        // Save to cache
        Self::save(exchange, &instruments).context("Failed to save instruments to cache")?;

        println!("Cache updated: {} instruments saved", instruments.len());

        Ok(instruments)
    }

    /// Load from cache, or refresh if invalid
    pub async fn load_or_refresh(
        exchange: &str,
        api_client: &crate::api::KiteConnectClient,
        force_refresh: bool,
    ) -> Result<Vec<Instrument>> {
        if force_refresh || !Self::is_valid(exchange)? {
            println!("Cache for {} is expired or refresh requested", exchange);
            Self::refresh(exchange, api_client).await
        } else {
            println!("Loading {} instruments from cache...", exchange);
            Self::load(exchange)
        }
    }

    /// Clear all cached instrument files
    pub fn clear_all() -> Result<()> {
        let cache_dir = Self::cache_dir()?;

        if !cache_dir.exists() {
            println!("Cache directory does not exist");
            return Ok(());
        }

        let mut cleared_count = 0;

        for entry in fs::read_dir(&cache_dir).context("Failed to read cache directory")? {
            let entry = entry.context("Failed to read directory entry")?;
            let path = entry.path();

            if path.extension().is_some_and(|ext| ext == "csv") {
                fs::remove_file(&path)
                    .context(format!("Failed to remove cache file: {:?}", path))?;
                cleared_count += 1;
                println!("Removed: {:?}", path.file_name().unwrap_or_default());
            }
        }

        println!("Cleared {} cache file(s)", cleared_count);

        Ok(())
    }

    /// Get cache info (files and sizes)
    pub fn info() -> Result<CacheInfo> {
        let cache_dir = Self::cache_dir()?;

        if !cache_dir.exists() {
            return Ok(CacheInfo {
                cache_dir: cache_dir.clone(),
                files: Vec::new(),
                total_size: 0,
            });
        }

        let mut files = Vec::new();
        let mut total_size = 0u64;

        for entry in fs::read_dir(&cache_dir).context("Failed to read cache directory")? {
            let entry = entry.context("Failed to read directory entry")?;
            let path = entry.path();

            if path.extension().is_some_and(|ext| ext == "csv") {
                let metadata = fs::metadata(&path).context("Failed to read file metadata")?;
                let size = metadata.len();
                let modified: DateTime<Utc> = metadata.modified()?.into();

                files.push(CacheFile {
                    exchange: path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("unknown")
                        .to_string(),
                    size,
                    modified,
                });

                total_size += size;
            }
        }

        Ok(CacheInfo {
            cache_dir,
            files,
            total_size,
        })
    }
}

/// Cache file information
#[derive(Debug, Clone)]
pub struct CacheFile {
    pub exchange: String,
    pub size: u64,
    pub modified: DateTime<Utc>,
}

/// Cache information summary
#[derive(Debug, Clone)]
pub struct CacheInfo {
    pub cache_dir: PathBuf,
    pub files: Vec<CacheFile>,
    pub total_size: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_file_path() {
        let path = InstrumentCache::cache_file("NSE").unwrap();
        assert!(path.to_str().unwrap().contains("instruments"));
        assert!(path.to_str().unwrap().to_lowercase().contains("nse"));
    }

    #[test]
    fn test_is_valid_no_file() {
        // Use a non-existent exchange
        let result = InstrumentCache::is_valid("NONEXISTENT");
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }
}
