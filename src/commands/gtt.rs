//! GTT commands

use crate::error::CliError;
use crate::output::OutputFormat;

/// GTT subcommands
#[derive(Debug, clap::Subcommand)]
pub enum GttCommand {
    /// List GTT triggers
    List,
    /// Get GTT details
    Get {
        /// Trigger ID
        trigger_id: String,
    },
    /// Create a GTT
    Create {
        /// Symbol
        #[arg(short, long)]
        symbol: String,
        /// Order type
        #[arg(short, long)]
        r#type: String,
        /// Quantity
        #[arg(short, long)]
        quantity: u32,
        /// Price
        #[arg(short, long)]
        price: f64,
        /// Trigger price
        #[arg(short, long)]
        trigger_price: f64,
    },
    /// Modify a GTT
    Modify {
        /// Trigger ID
        trigger_id: String,
        /// New price
        #[arg(short, long)]
        price: Option<f64>,
    },
    /// Delete a GTT
    Delete {
        /// Trigger ID
        trigger_id: String,
    },
}

/// Execute GTT command
pub async fn execute(
    command: GttCommand,
    _yes: bool,
    _output_format: OutputFormat,
) -> Result<(), CliError> {
    match command {
        GttCommand::List => {
            println!("Listing GTT triggers...");
            Ok(())
        }
        GttCommand::Get { trigger_id } => {
            println!("Getting GTT: {}", trigger_id);
            Ok(())
        }
        GttCommand::Create { .. } => {
            println!("Creating GTT...");
            Ok(())
        }
        GttCommand::Modify { trigger_id, .. } => {
            println!("Modifying GTT: {}", trigger_id);
            Ok(())
        }
        GttCommand::Delete { trigger_id } => {
            println!("Deleting GTT: {}", trigger_id);
            Ok(())
        }
    }
}
