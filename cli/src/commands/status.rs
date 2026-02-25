//! Status command handlers

use anyhow::Result;
use zerodha_cli_core::{api::KiteConnectClient, auth::AuthStatus, config::Config};

pub async fn run_status(config: &Config, api_client: &KiteConnectClient) -> Result<()> {
    println!("Zerodha CLI Status");
    println!("==================");
    println!();

    // Version
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!();

    // Config
    println!("Configuration:");
    if let Ok(config_path) = Config::config_path() {
        println!("  Config: {}", config_path.display());
        if config_path.exists() {
            println!("  Config Status: ✓ Loaded");
            println!("  API Key: {}", mask_key(&config.api.api_key));
        } else {
            println!("  Config Status: Not found (run 'kite auth setup')");
        }
    }
    println!();

    // Auth
    println!("Authentication:");
    let auth_status = zerodha_cli_core::auth::status(config);
    match auth_status {
        AuthStatus::NotAuthenticated => {
            println!("  Status: ✗ Not authenticated (run 'kite auth login')");
        }
        AuthStatus::Authenticated { expiry: None } => {
            println!("  Status: ✓ Authenticated");
            println!("  Token Expires: Unknown");
        }
        AuthStatus::Authenticated {
            expiry: Some(expiry),
        } => {
            println!("  Status: ✓ Authenticated");
            println!("  Token Expires: {}", expiry);
        }
        AuthStatus::TokenExpired => {
            println!("  Status: ✗ Token expired (run 'kite auth login')");
        }
    }
    println!();

    // Cache
    println!("Cache:");
    let exchanges = ["NSE", "BSE", "NFO", "BFO", "MCX", "CDS"];
    for exchange in exchanges {
        if let Ok(valid) = zerodha_cli_core::cache::InstrumentCache::is_valid(exchange) {
            let status = if valid {
                "✓ Cached"
            } else {
                "○ Not cached"
            };
            println!("  {}: {}", exchange, status);
        }
    }
    println!();

    // API Connection
    println!("API Connection:");
    println!("  Endpoint: https://api.kite.trade");
    println!("  Status: Checking...");

    // Try a simple API call to check connectivity
    match api_client.get_margins().await {
        Ok(_) => println!("  Status: ✓ Connected"),
        Err(e) => {
            if e.to_string().contains("401") || e.to_string().contains("Authentication") {
                println!("  Status: ⚠ Connected but not authenticated");
            } else {
                println!("  Status: ✗ Connection failed");
                println!("  Error: {}", e);
            }
        }
    }

    Ok(())
}

fn mask_key(key: &str) -> String {
    if key.len() <= 8 {
        format!("{}****", &key[..2])
    } else {
        format!("{}...{}", &key[..4], &key[key.len() - 4..])
    }
}
