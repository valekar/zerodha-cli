//! Portfolio commands

use crate::config::Config;
use crate::api::KiteClient;
use crate::error::CliError;
use crate::output::OutputFormat;

/// Portfolio subcommands
#[derive(Debug, clap::Subcommand)]
pub enum PortfolioCommand {
    /// Show CNC holdings
    Holdings,
    /// Show positions
    Positions,
    /// Convert position type
    Convert {
        /// Symbol
        #[arg(short, long)]
        symbol: String,
        /// Transaction type
        #[arg(short, long)]
        r#type: String,
        /// Quantity
        #[arg(short, long)]
        quantity: u32,
        /// From product
        #[arg(long)]
        from: String,
        /// To product
        #[arg(long)]
        to: String,
    },
}

/// Execute portfolio command
pub async fn execute(
    command: PortfolioCommand,
    _output_format: OutputFormat,
) -> Result<(), CliError> {
    let config = Config::load()?;
    let api_key = config.api.api_key.as_ref().ok_or(CliError::InvalidCredentials)?.clone();
    let token = config.get_access_token()?;

    let client = KiteClient::new(api_key, token);

    match command {
        PortfolioCommand::Holdings => {
            let holdings = client.get_holdings().await?;
            println!("Holdings: {:#?}", holdings);
            Ok(())
        }
        PortfolioCommand::Positions => {
            let positions = client.get_positions().await?;
            println!("Positions: {:#?}", positions);
            Ok(())
        }
        PortfolioCommand::Convert { .. } => {
            println!("Position conversion not yet implemented");
            Ok(())
        }
    }
}
