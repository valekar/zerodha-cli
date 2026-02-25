//! Zerodha CLI - Main entry point
//!
//! A terminal-based trading tool for Zerodha's Kite Connect API

use anyhow::Result;
use zerodha_cli_core::cli;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Run CLI
    cli::run().await
}
