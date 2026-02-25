//! Status command

use crate::config::Config;
use crate::error::CliError;
use crate::output::OutputFormat;

/// Execute status command
pub async fn execute(_output_format: OutputFormat) -> Result<(), CliError> {
    let config = Config::load()?;

    println!("=== Kite CLI Status ===");
    
    // Check authentication
    match config.api.access_token {
        Some(_) => println!("✓ Authenticated"),
        None => println!("✗ Not authenticated"),
    }

    // Check API key
    match config.api.api_key {
        Some(key) => {
            println!("✓ API Key configured: {}...", &key[..8.min(key.len())]);
        }
        None => println!("✗ API Key not configured"),
    }

    Ok(())
}
