//! Instruments commands

use crate::config::Config;
use crate::api::KiteClient;
use crate::error::CliError;

/// Instruments subcommands
#[derive(Debug, clap::Subcommand)]
pub enum InstrumentCommand {
    /// List all instruments
    List {
        /// Exchange filter
        #[arg(short, long)]
        exchange: Option<String>,
        /// Force refresh from API
        #[arg(long)]
        force_refresh: bool,
    },
    /// Search for instruments
    Search {
        /// Search query
        query: String,
    },
    /// Get instrument details
    Get {
        /// Instrument key
        instrument: String,
    },
}

/// Execute instrument command
pub async fn execute(
    command: InstrumentCommand,
    _force_refresh: bool,
    _output_format: OutputFormat,
) -> Result<(), CliError> {
    match command {
        InstrumentCommand::List { exchange, force_refresh: _ } => {
            list_instruments(exchange).await
        }
        InstrumentCommand::Search { query } => {
            search_instruments(query).await
        }
        InstrumentCommand::Get { instrument } => {
            get_instrument(instrument).await
        }
    }
}

async fn list_instruments(_exchange: Option<String>) -> Result<(), CliError> {
    let config = Config::load()?;
    let api_key = config.api.api_key.as_ref().ok_or(CliError::InvalidCredentials)?.clone();
    let token = config.get_access_token()?;

    let client = KiteClient::new(api_key, token);
    let instruments = client.get_instruments(None).await?;

    println!("Total instruments: {}", instruments.len());
    Ok(())
}

async fn search_instruments(_query: String) -> Result<(), CliError> {
    // TODO: Implement search using cached instruments
    println!("Searching instruments...");
    Ok(())
}

async fn get_instrument(_instrument: String) -> Result<(), CliError> {
    // TODO: Implement get instrument
    println!("Getting instrument details...");
    Ok(())
}

use crate::output::OutputFormat;
