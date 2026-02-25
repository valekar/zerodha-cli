//! Authentication commands

use crate::config::Config;
use crate::api::KiteClient;
use crate::error::CliError;
use std::process::Command;

/// Authentication subcommands
#[derive(Debug, clap::Subcommand)]
pub enum AuthCommand {
    /// Setup API credentials
    Setup {
        /// API key
        #[arg(short, long)]
        api_key: String,
        /// API secret
        #[arg(short, long)]
        api_secret: String,
    },
    /// Login to get access token
    Login,
    /// Logout and clear token
    Logout,
    /// Show authentication status
    Status,
}

/// Execute auth command
pub async fn execute(command: AuthCommand) -> Result<(), CliError> {
    match command {
        AuthCommand::Setup { api_key, api_secret } => {
            setup(api_key, api_secret).await
        }
        AuthCommand::Login => {
            login().await
        }
        AuthCommand::Logout => {
            logout().await
        }
        AuthCommand::Status => {
            status().await
        }
    }
}

async fn setup(api_key: String, api_secret: String) -> Result<(), CliError> {
    let mut config = Config::load()?;
    config.api.api_key = Some(api_key);
    config.api.api_secret = Some(api_secret);
    config.save()?;
    println!("API credentials saved successfully!");
    Ok(())
}

async fn login() -> Result<(), CliError> {
    let config = Config::load()?;
    let api_key = config.api.api_key.ok_or(CliError::InvalidCredentials)?;

    // Construct OAuth URL
    let login_url = format!("https://kite.zerodha.com/connect/login?v=3&api_key={}", api_key);

    // Open browser
    println!("Opening browser for authentication...");
    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("open").arg(&login_url).status();
    }
    #[cfg(target_os = "linux")]
    {
        let _ = Command::new("xdg-open").arg(&login_url).status();
    }
    #[cfg(target_os = "windows")]
    {
        let _ = Command::new("cmd").args(["/C", "start", "", &login_url]).status();
    }

    println!("\nAfter logging in, enter the request_token from the redirect URL:");
    let mut request_token = String::new();
    std::io::stdin().read_line(&mut request_token)?;
    let request_token = request_token.trim();

    print!("Enter TOTP (2FA): ");
    let mut totp = String::new();
    std::io::stdin().read_line(&mut totp)?;
    let totp = totp.trim();

    // Generate checksum
    let _api_secret = config.api.api_secret.ok_or(CliError::InvalidCredentials)?;
    let checksum = format!("{}{}{}", api_key, request_token, totp);

    // Create client and generate session
    let client = KiteClient::new(api_key.clone(), String::new());
    let session = client.generate_session(request_token, &checksum).await?;

    // Save access token
    let mut config = Config::load()?;
    config.api.access_token = Some(session.access_token);
    config.save()?;

    println!("\n✓ Authentication successful!");
    println!("User: {}", session.user_name);
    Ok(())
}

async fn logout() -> Result<(), CliError> {
    let config = Config::load()?;
    let api_key = config.api.api_key.as_ref().ok_or(CliError::InvalidCredentials)?.clone();
    let token = config.get_access_token()?;

    let client = KiteClient::new(api_key, token);
    client.invalidate_token().await?;

    let mut config = Config::load()?;
    config.api.access_token = None;
    config.save()?;

    println!("✓ Logged out successfully.");
    Ok(())
}

async fn status() -> Result<(), CliError> {
    let config = Config::load()?;

    match config.api.access_token {
        Some(_) => {
            println!("✓ Authenticated");
        }
        None => {
            println!("✗ Not authenticated");
        }
    }

    Ok(())
}
