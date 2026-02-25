//! CLI command definitions and routing

mod auth;
mod gtt;
mod instruments;
mod margins;
mod orders;
mod portfolio;
mod quotes;
mod shell;
mod status;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::sync::Arc;
use zerodha_cli_core::{api::KiteConnectClient, config::Config};

#[derive(Parser)]
#[command(name = "kite")]
#[command(
    about = "Zerodha Kite Connect CLI",
    long_about = "A terminal-based trading tool for Zerodha's Kite Connect API"
)]
#[command(version = "1.0.0")]
#[command(author = "Zerodha CLI Team")]
pub struct Cli {
    /// Output format (table, json)
    #[arg(short, long, global = true, default_value = "table")]
    pub output: String,

    /// Config file path
    #[arg(short, long, global = true)]
    pub config: Option<String>,

    /// Verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Authentication management
    Auth(AuthCommands),

    /// Browse and search instruments
    Instruments(InstrumentsCommands),

    /// Market data and quotes
    Quotes(QuotesCommands),

    /// Order management
    Orders(OrdersCommands),

    /// Holdings and positions
    Portfolio(PortfolioCommands),

    /// Margin and funds information
    Margins(MarginsCommands),

    /// Good Till Triggered orders
    Gtt(GttCommands),

    /// Show system status
    Status,

    /// Interactive REPL mode
    Shell,
}

#[derive(clap::Args, Debug)]
pub struct AuthCommands {
    /// Authenticate with Zerodha (OAuth flow)
    #[command(subcommand)]
    pub command: AuthSubcommands,
}

#[derive(Subcommand, Debug)]
pub enum AuthSubcommands {
    /// Authenticate with Zerodha (OAuth flow)
    Login,

    /// Show authentication status
    Status,

    /// Logout and invalidate session
    Logout,

    /// Configure API credentials
    Setup {
        /// API key
        #[arg(long)]
        api_key: String,

        /// API secret
        #[arg(long)]
        api_secret: String,
    },
}

#[derive(clap::Args, Debug)]
pub struct InstrumentsCommands {
    #[command(subcommand)]
    pub command: InstrumentsSubcommands,
}

#[derive(Subcommand, Debug)]
pub enum InstrumentsSubcommands {
    /// List all instruments from exchange
    List {
        /// Exchange (NSE, BSE, NFO, BFO, MCX, CDS)
        #[arg(short, long)]
        exchange: Option<String>,

        /// Refresh cache (re-download instruments)
        #[arg(short, long)]
        refresh: bool,
    },

    /// Search for instrument by symbol or name
    Search {
        /// Search query (symbol or name)
        query: String,

        /// Exchange filter
        #[arg(short, long)]
        exchange: Option<String>,
    },

    /// Get detailed info for specific instrument
    Get {
        /// Instrument symbol (e.g., NSE:INFY)
        symbol: String,
    },
}

#[derive(clap::Args, Debug)]
pub struct QuotesCommands {
    #[command(subcommand)]
    pub command: QuotesSubcommands,
}

#[derive(Subcommand, Debug)]
pub enum QuotesSubcommands {
    /// Get full quote for one or more instruments
    Get {
        /// Instrument symbols (e.g., NSE:INFY NSE:TCS)
        symbols: Vec<String>,
    },

    /// Get OHLC data only
    Ohlc {
        /// Instrument symbols
        symbols: Vec<String>,
    },

    /// Get last traded price only
    Ltp {
        /// Instrument symbols
        symbols: Vec<String>,
    },
}

#[derive(clap::Args, Debug)]
pub struct OrdersCommands {
    #[command(subcommand)]
    pub command: OrdersSubcommands,
}

#[derive(Subcommand, Debug)]
pub enum OrdersSubcommands {
    /// List all orders
    List {
        /// Filter by status
        #[arg(short, long)]
        status: Option<String>,
    },

    /// Get details for specific order
    Get {
        /// Order ID
        order_id: String,
    },

    /// Place a limit order
    Place {
        /// Instrument symbol (e.g., NSE:INFY)
        #[arg(short, long)]
        symbol: String,

        /// Transaction type (BUY, SELL)
        #[arg(short, long)]
        order_type: String,

        /// Order type (MARKET, LIMIT, SL, SL-M)
        #[arg(long)]
        order_type_enum: Option<String>,

        /// Quantity
        #[arg(short, long)]
        quantity: i32,

        /// Price (for LIMIT orders)
        #[arg(short, long)]
        price: f64,

        /// Product type (CNC, MIS, NRML)
        #[arg(short, long)]
        product: Option<String>,

        /// Validity (DAY, IOC)
        #[arg(short, long)]
        validity: Option<String>,

        /// Dry-run mode (don't actually place order)
        #[arg(long)]
        dry_run: bool,

        /// Variety (regular, amo, co, iceberg)
        #[arg(long, default_value = "regular")]
        variety: String,
    },

    /// Place a market order
    Market {
        /// Instrument symbol
        #[arg(short, long)]
        symbol: String,

        /// Transaction type (BUY, SELL)
        #[arg(short, long)]
        order_type: String,

        /// Quantity
        #[arg(short, long)]
        quantity: i32,

        /// Product type
        #[arg(short, long)]
        product: Option<String>,

        /// Dry-run mode
        #[arg(long)]
        dry_run: bool,
    },

    /// Modify an existing order
    Modify {
        /// Order ID
        order_id: String,

        /// New price
        #[arg(short, long)]
        price: Option<f64>,

        /// New quantity
        #[arg(short, long)]
        quantity: Option<i32>,

        /// New trigger price (for SL orders)
        #[arg(long)]
        trigger_price: Option<f64>,

        /// New validity
        #[arg(short, long)]
        validity: Option<String>,

        /// New disclosed quantity
        #[arg(long)]
        disclosed_quantity: Option<i32>,
    },

    /// Cancel an order
    Cancel {
        /// Order ID
        order_id: String,

        /// Variety (regular, amo, co, iceberg)
        #[arg(long, default_value = "regular")]
        variety: String,
    },

    /// Cancel all open orders
    CancelAll,

    /// View trade history
    Trades {
        /// Order ID (optional)
        order_id: Option<String>,
    },
}

#[derive(clap::Args, Debug)]
pub struct PortfolioCommands {
    #[command(subcommand)]
    pub command: PortfolioSubcommands,
}

#[derive(Subcommand, Debug)]
pub enum PortfolioSubcommands {
    /// View holdings (long-term equity)
    Holdings,

    /// View positions (intraday/F&O)
    Positions {
        /// Show net positions (default)
        #[arg(long)]
        net: bool,

        /// Show day positions only
        #[arg(long)]
        day: bool,
    },

    /// Convert position type
    Convert {
        /// Instrument symbol
        #[arg(short, long)]
        symbol: String,

        /// Transaction type (BUY, SELL)
        #[arg(short, long)]
        order_type: String,

        /// Quantity
        #[arg(short, long)]
        quantity: i32,

        /// From product type
        #[arg(long)]
        from: String,

        /// To product type
        #[arg(long)]
        to: String,
    },
}

#[derive(clap::Args, Debug)]
pub struct MarginsCommands {
    #[command(subcommand)]
    pub command: MarginsSubcommands,
}

#[derive(Subcommand, Debug)]
pub enum MarginsSubcommands {
    /// View all margin segments
    List,

    /// View equity margins
    Equity,

    /// View commodity margins
    Commodity,
}

#[derive(clap::Args, Debug)]
pub struct GttCommands {
    #[command(subcommand)]
    pub command: GttSubcommands,
}

#[derive(Subcommand, Debug)]
pub enum GttSubcommands {
    /// List all GTT orders
    List,

    /// Get details for specific GTT
    Get {
        /// Trigger ID
        trigger_id: String,
    },

    /// Create a GTT order
    Create {
        /// Instrument symbol
        #[arg(short, long)]
        symbol: String,

        /// Transaction type (BUY, SELL)
        #[arg(short, long)]
        order_type: String,

        /// Quantity
        #[arg(short, long)]
        quantity: i32,

        /// Order price
        #[arg(short, long)]
        price: f64,

        /// Trigger price
        #[arg(short, long)]
        trigger_price: f64,

        /// Trigger type (single, two-leg)
        #[arg(long)]
        trigger_type: String,

        /// Order type (MARKET, LIMIT)
        #[arg(long)]
        order_type_enum: Option<String>,

        /// Product type
        #[arg(short, long)]
        product: Option<String>,
    },

    /// Modify an existing GTT
    Modify {
        /// Trigger ID
        trigger_id: String,

        /// New order price
        #[arg(short, long)]
        price: Option<f64>,

        /// New trigger price
        #[arg(short, long)]
        trigger_price: Option<f64>,
    },

    /// Delete a GTT order
    Delete {
        /// Trigger ID
        trigger_id: String,
    },
}

/// Run the CLI
pub async fn run() -> Result<()> {
    let cli = Cli::parse();

    // Load config
    let mut config = if let Some(ref path) = cli.config {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config from {}", path))?;
        toml::from_str(&content).with_context(|| "Failed to parse config file")?
    } else {
        Config::load().with_context(|| {
            "Failed to load config. Run 'kite auth setup' to configure API credentials."
        })?
    };

    // Create API client
    let api_client =
        KiteConnectClient::new(config.api.api_key.clone(), config.api.api_secret.clone());

    // Set access token if available
    if let Some(ref token) = config.api.access_token {
        eprintln!("Debug: API key from config ({} chars): {}", config.api.api_key.len(), &config.api.api_key[..8.min(config.api.api_key.len())]);
        eprintln!("Debug: Access token from config ({} chars): {}...", token.len(), &token[..16.min(token.len())]);
        api_client.set_access_token(token.clone()).await?;
    } else {
        eprintln!("Debug: No access token found in config");
    }

    // Execute command
    match cli.command {
        Commands::Auth(auth_cmd) => auth::run_auth(auth_cmd, &mut config, &api_client).await?,
        Commands::Instruments(instruments_cmd) => {
            instruments::run_instruments(instruments_cmd, &api_client, &cli.output).await?
        }
        Commands::Quotes(quotes_cmd) => {
            quotes::run_quotes(quotes_cmd, &api_client, &cli.output).await?
        }
        Commands::Orders(orders_cmd) => {
            orders::run_orders(orders_cmd, &config, &api_client, &cli.output).await?
        }
        Commands::Portfolio(portfolio_cmd) => {
            portfolio::run_portfolio(portfolio_cmd, &api_client, &cli.output).await?
        }
        Commands::Margins(margins_cmd) => {
            margins::run_margins(margins_cmd, &api_client, &cli.output).await?
        }
        Commands::Gtt(gtt_cmd) => gtt::run_gtt(gtt_cmd, &api_client, &cli.output).await?,
        Commands::Status => status::run_status(&config, &api_client).await?,
        Commands::Shell => {
            let config_arc = Arc::new(tokio::sync::Mutex::new(config));
            let api_client_arc = Arc::new(api_client);
            shell::run_shell(config_arc, api_client_arc, &cli.output).await?
        }
    }

    Ok(())
}
