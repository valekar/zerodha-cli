//! Interactive shell module

use anyhow::Result;
use std::path::PathBuf;

#[allow(clippy::module_inception)]
pub mod shell;
pub use shell::run;

/// Get shell history file path
pub fn shell_history_path() -> Result<PathBuf> {
    let data_dir =
        dirs::data_local_dir().ok_or_else(|| anyhow::anyhow!("Failed to get data directory"))?;
    Ok(data_dir.join("zerodha-cli").join("history"))
}
