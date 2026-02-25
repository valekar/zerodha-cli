//! Margins commands

use crate::config::Config;
use crate::api::KiteClient;
use crate::error::CliError;
use crate::output::OutputFormat;

/// Margins subcommands
#[derive(Debug, clap::Subcommand)]
pub enum MarginsCommand {
    /// Show all margins
    List,
    /// Show equity margins
    Equity,
    /// Show commodity margins
    Commodity,
}

/// Execute margins command
pub async fn execute(
    command: MarginsCommand,
    _output_format: OutputFormat,
) -> Result<(), CliError> {
    let config = Config::load()?;
    let api_key = config.api.api_key.as_ref().ok_or(CliError::InvalidCredentials)?.clone();
    let token = config.get_access_token()?;

    let client = KiteClient::new(api_key, token);

    let segment = match command {
        MarginsCommand::List => None,
        MarginsCommand::Equity => Some("equity"),
        MarginsCommand::Commodity => Some("commodity"),
    };

    let margins = client.get_margins(segment).await?;
    println!("Margins: {:#?}", margins);
    Ok(())
}
