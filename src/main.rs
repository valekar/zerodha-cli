//! Zerodha Kite Connect CLI
//!
//! A command-line interface for Zerodha's Kite Connect trading platform.

use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

mod commands;
mod output;

use commands::{
    auth, instruments, quotes, orders, portfolio,
    margins, gtt, shell, status,
};
use output::OutputFormat;

#[derive(Parser, Debug)]
#[command(name = "kite")]
#[command(about = "Zerodha Kite Connect CLI - Trade from your terminal", long_about = None)]
#[command(version = "0.1.0")]
struct Cli {
    /// Output format
    #[arg(short, long, value_enum, global = true)]
    output: Option<OutputFormat>,

    /// Dry run (don't execute real orders)
    #[arg(long, global = true)]
    dry_run: bool,

    /// Auto-confirm
    #[arg(long, global = true)]
    yes: bool,

    /// Verbosity
    #[command(flatten)]
    verbose: Verbosity,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Auth {
        #[command(subcommand)]
        command: auth::AuthCommand,
    },
    Instruments {
        #[command(subcommand)]
        command: instruments::InstrumentCommand,
    },
    Quotes {
        #[command(subcommand)]
        command: quotes::QuotesCommand,
    },
    Orders {
        #[command(subcommand)]
        command: orders::OrdersCommand,
    },
    Portfolio {
        #[command(subcommand)]
        command: portfolio::PortfolioCommand,
    },
    Margins {
        #[command(subcommand)]
        command: margins::MarginsCommand,
    },
    Gtt {
        #[command(subcommand)]
        command: gtt::GttCommand,
    },
    Shell,
    Status,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Initialize logging
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("error")
    ).init();

    let output_format = cli.output.unwrap_or(OutputFormat::Table);

    match cli.command {
        Some(Commands::Auth { command }) => {
            auth::execute(command).await?;
        }
        Some(Commands::Instruments { command }) => {
            instruments::execute(command, false, output_format).await?;
        }
        Some(Commands::Quotes { command }) => {
            quotes::execute(command, output_format).await?;
        }
        Some(Commands::Orders { command }) => {
            orders::execute(command, cli.dry_run, cli.yes, output_format).await?;
        }
        Some(Commands::Portfolio { command }) => {
            portfolio::execute(command, output_format).await?;
        }
        Some(Commands::Margins { command }) => {
            margins::execute(command, output_format).await?;
        }
        Some(Commands::Gtt { command }) => {
            gtt::execute(command, cli.yes, output_format).await?;
        }
        Some(Commands::Shell) => {
            shell::execute().await?;
        }
        Some(Commands::Status) => {
            status::execute(output_format).await?;
        }
        None => {
            // No command: show help
            Cli::parse();
        }
    }

    Ok(())
}
