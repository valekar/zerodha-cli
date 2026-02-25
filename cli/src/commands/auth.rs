//! Authentication command handlers

use anyhow::{Context, Result};
use zerodha_cli_core::{
    auth::{self, AuthStatus},
    config::Config,
};

use super::AuthCommands;

pub async fn run_auth(
    cmd: AuthCommands,
    config: &mut Config,
    api_client: &zerodha_cli_core::api::KiteConnectClient,
) -> Result<()> {
    match cmd.command {
        super::AuthSubcommands::Login => run_auth_login(config, api_client).await?,
        super::AuthSubcommands::Status => run_auth_status(config)?,
        super::AuthSubcommands::Logout => run_auth_logout(config)?,
        super::AuthSubcommands::Setup {
            api_key,
            api_secret,
        } => run_auth_setup(api_key, api_secret, config)?,
    }
    Ok(())
}

pub async fn run_auth_login(
    config: &mut Config,
    api_client: &zerodha_cli_core::api::KiteConnectClient,
) -> Result<()> {
    println!("Initiating OAuth login flow...");
    let token = auth::login(api_client, config)
        .await
        .context("Failed to complete login")?;
    println!("✓ Logged in successfully!");
    println!("Access token: {}...", &token[..8.min(token.len())]);
    Ok(())
}

pub fn run_auth_status(config: &Config) -> Result<()> {
    let status = auth::status(config);
    match status {
        AuthStatus::NotAuthenticated => {
            println!("Authentication status: Not authenticated");
            println!("Run 'kite auth login' to authenticate.");
        }
        AuthStatus::Authenticated { expiry } => {
            println!("Authentication status: Authenticated");
            if let Some(expiry) = expiry {
                println!("Token expires: {}", expiry);
            } else {
                println!("Token expiry: Unknown");
            }
        }
        AuthStatus::TokenExpired => {
            println!("Authentication status: Token expired");
            println!("Run 'kite auth login' to renew.");
        }
    }
    Ok(())
}

pub fn run_auth_logout(config: &mut Config) -> Result<()> {
    print!("Are you sure you want to logout? [y/N]: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();

    if input == "y" || input == "yes" {
        auth::logout(config).context("Failed to logout")?;
        println!("✓ Logged out successfully!");
    } else {
        println!("Logout cancelled.");
    }
    Ok(())
}

pub fn run_auth_setup(api_key: String, api_secret: String, config: &mut Config) -> Result<()> {
    config.api.api_key = api_key;
    config.api.api_secret = api_secret;
    config.save().context("Failed to save config")?;
    println!("✓ API credentials configured successfully!");
    println!("Config file: {}", Config::config_path()?.display());
    Ok(())
}
