//! Zerodha CLI - Main entry point
//!
//! A terminal-based trading tool for Zerodha's Kite Connect API

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load .env from current directory
    dotenv::from_filename(".env").ok();
    
    // Run CLI
    zerodha_cli::run().await
}
