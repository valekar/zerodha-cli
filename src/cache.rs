//! Instrument cache management

use std::fs;
use std::path::Path;
use std::time::Duration;

/// Instrument cache manager
pub struct InstrumentCache {
    cache_path: std::path::PathBuf,
    ttl: Duration,
}

impl InstrumentCache {
    /// Create a new cache manager
    pub fn new(cache_path: std::path::PathBuf, ttl: Duration) -> Self {
        InstrumentCache { cache_path, ttl }
    }

    /// Check if cache is valid (not expired)
    pub fn is_valid(&self) -> bool {
        if !self.cache_path.exists() {
            return false;
        }

        match fs::metadata(&self.cache_path) {
            Ok(metadata) => {
                match metadata.modified() {
                    Ok(modified) => {
                        match modified.elapsed() {
                            Ok(age) => age < self.ttl,
                            Err(_) => false,
                        }
                    }
                    Err(_) => false,
                }
            }
            Err(_) => false,
        }
    }

    /// Get cache file path
    pub fn path(&self) -> &Path {
        &self.cache_path
    }

    /// Ensure cache directory exists
    pub fn ensure_dir(&self) -> Result<(), std::io::Error> {
        if let Some(parent) = self.cache_path.parent() {
            fs::create_dir_all(parent)?;
        }
        Ok(())
    }
}
