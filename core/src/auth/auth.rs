//! Authentication logic

use crate::api::KiteConnectClient;
use crate::config::Config;
use anyhow::{Context, Result};

/// Authentication status
#[derive(Debug, Clone)]
pub enum AuthStatus {
    NotAuthenticated,
    Authenticated { expiry: Option<String> },
    TokenExpired,
}

/// Initiate OAuth login flow
pub async fn login(api_client: &KiteConnectClient, config: &mut Config) -> Result<String> {
    // 1. Generate login URL
    let login_url = api_client.login_url();

    println!("========================================");
    println!("  Zerodha Kite Connect Authentication");
    println!("========================================\n");
    println!("Opening browser to login page...");

    // 2. Open browser
    if let Err(e) = webbrowser::open(&login_url) {
        println!("Failed to open browser: {}", e);
        println!("\nPlease open this URL manually in your browser:\n");
        println!("{}\n", login_url);
    } else {
        println!("\nBrowser opened successfully!");
    }

    println!(
        "\nAfter completing login in your browser, you'll be redirected to a page"
    );
    println!("with a 'request_token' parameter in the URL.\n");
    println!("Example URL: https://kite.zerodha.com/connect/login?v=3&api_key=XXX&request_token=abc123\n");

    // 3. Prompt user for request_token
    print!("Enter the 'request_token' from the URL: ");

    // Use tokio for async stdin reading
    let mut request_token = String::new();
    tokio::task::block_in_place(|| {
        std::io::stdin().read_line(&mut request_token)
    })
    .context("Failed to read request token")?;

    let request_token = request_token.trim();

    if request_token.is_empty() {
        anyhow::bail!("Request token cannot be empty");
    }

    // 4. Exchange for access token
    println!("\nExchanging request token for access token...");

    let access_token = api_client
        .exchange_token(request_token)
        .await
        .context("Failed to exchange token. Please check your API credentials and try again.")?;

    // 5. Save to config
    let expiry = chrono::Utc::now() + chrono::Duration::days(1);
    let expiry_str = expiry.to_rfc3339();

    config.api.access_token = Some(access_token.clone());
    config.api.token_expiry = Some(expiry_str);

    config.save().context("Failed to save config")?;

    // 6. Update API client
    api_client.set_access_token(access_token.clone()).await?;

    println!("\n✓ Authentication successful!");
    println!("✓ Access token saved to config");
    println!("✓ Token expires: {}", expiry.format("%Y-%m-%d %H:%M:%S UTC"));

    Ok(access_token)
}

/// Logout and invalidate session
pub fn logout(config: &mut Config) -> Result<()> {
    config.api.access_token = None;
    config.api.token_expiry = None;

    config.save().context("Failed to save config")?;

    println!("✓ Logged out successfully");
    println!("✓ Access token removed from config");

    Ok(())
}

/// Check authentication status
pub fn status(config: &Config) -> AuthStatus {
    if let Some(_token) = &config.api.access_token {
        if config.is_token_valid() {
            AuthStatus::Authenticated {
                expiry: config.api.token_expiry.clone(),
            }
        } else {
            AuthStatus::TokenExpired
        }
    } else {
        AuthStatus::NotAuthenticated
    }
}

/// Print authentication status to console
pub fn print_status(status: AuthStatus) {
    match status {
        AuthStatus::NotAuthenticated => {
            println!("Authentication Status: Not authenticated");
            println!("\nPlease run: kite auth login");
        }
        AuthStatus::Authenticated { expiry } => {
            println!("Authentication Status: ✓ Authenticated");

            if let Some(expiry_str) = expiry {
                if let Ok(expiry) = chrono::DateTime::parse_from_rfc3339(&expiry_str) {
                    let expiry_utc = expiry.with_timezone(&chrono::Utc);
                    let now = chrono::Utc::now();
                    let remaining = expiry_utc - now;

                    if remaining.num_seconds() > 0 {
                        let hours = remaining.num_hours();
                        let minutes = remaining.num_minutes() % 60;

                        println!("Token expires in: {}h {}m", hours, minutes);
                        println!("Expiry time: {}", expiry_utc.format("%Y-%m-%d %H:%M:%S UTC"));
                    } else {
                        println!("Token expired: {}", expiry_utc.format("%Y-%m-%d %H:%M:%S UTC"));
                    }
                }
            }
        }
        AuthStatus::TokenExpired => {
            println!("Authentication Status: Token expired");
            println!("\nPlease run: kite auth login");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_not_authenticated() {
        let config = Config::default();
        let status = status(&config);
        assert!(matches!(status, AuthStatus::NotAuthenticated));
    }

    #[test]
    fn test_status_token_expired() {
        let mut config = Config::default();
        config.api.access_token = Some("test_token".to_string());

        // Set expiry in the past
        let past_expiry = chrono::Utc::now() - chrono::Duration::days(1);
        config.api.token_expiry = Some(past_expiry.to_rfc3339());

        let status = status(&config);
        assert!(matches!(status, AuthStatus::TokenExpired));
    }

    #[test]
    fn test_status_authenticated() {
        let mut config = Config::default();
        config.api.access_token = Some("test_token".to_string());

        // Set expiry in the future
        let future_expiry = chrono::Utc::now() + chrono::Duration::days(1);
        config.api.token_expiry = Some(future_expiry.to_rfc3339());

        let status = status(&config);
        assert!(matches!(status, AuthStatus::Authenticated { .. }));
    }
}
