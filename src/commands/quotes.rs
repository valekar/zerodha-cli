//! Quotes commands

use crate::config::Config;
use crate::api::KiteClient;
use crate::error::CliError;
use crate::output::OutputFormat;

/// Quotes subcommands
#[derive(Debug, clap::Subcommand)]
pub enum QuotesCommand {
    /// Get full quotes
    Get {
        /// Instrument keys
        #[arg(required = true)]
        instruments: Vec<String>,
    },
    /// Get OHLC data
    Ohlc {
        /// Instrument keys
        #[arg(required = true)]
        instruments: Vec<String>,
    },
    /// Get LTP
    Ltp {
        /// Instrument keys
        #[arg(required = true)]
        instruments: Vec<String>,
    },
}

/// Execute quotes command
pub async fn execute(
    command: QuotesCommand,
    _output_format: OutputFormat,
) -> Result<(), CliError> {
    let config = Config::load()?;
    let api_key = config.api.api_key.as_ref().ok_or(CliError::InvalidCredentials)?.clone();
    let token = config.get_access_token()?;

    let client = KiteClient::new(api_key, token);

    match command {
        QuotesCommand::Get { instruments } => {
            let quotes = client.get_quotes(&instruments).await?;
            println!("{:#?}", quotes);
            Ok(())
        }
        QuotesCommand::Ohlc { instruments } => {
            println!("Getting OHLC for: {:?}", instruments);
            Ok(())
        }
        QuotesCommand::Ltp { instruments } => {
            let ltp = client.get_ltp(&instruments).await?;
            println!("{:#?}", ltp);
            Ok(())
        }
    }
}
