//! Orders commands

use crate::config::Config;
use crate::api::{KiteClient, OrderType, Product, TransactionType, PlaceOrder};
use crate::error::CliError;
use crate::output::OutputFormat;
use std::str::FromStr;

/// Orders subcommands
#[derive(Debug, clap::Subcommand)]
pub enum OrdersCommand {
    /// List orders
    List {
        /// Status filter
        #[arg(short, long)]
        status: Option<String>,
    },
    /// Get order details
    Get {
        /// Order ID
        order_id: String,
    },
    /// Place a limit order
    Place {
        /// Symbol
        #[arg(short, long)]
        symbol: String,
        /// Order type (BUY/SELL)
        #[arg(short, long)]
        order_type: String,
        /// Quantity
        #[arg(short, long)]
        quantity: u32,
        /// Price
        #[arg(short, long)]
        price: f64,
        /// Product (CNC/MIS/NRML/MTF)
        #[arg(short, long, default_value = "CNC")]
        product: String,
    },
    /// Place a market order
    Market {
        /// Symbol
        #[arg(short, long)]
        symbol: String,
        /// Order type
        #[arg(short, long)]
        order_type: String,
        /// Quantity
        #[arg(short, long)]
        quantity: u32,
        /// Product
        #[arg(short, long, default_value = "MIS")]
        product: String,
    },
    /// Modify an order
    Modify {
        /// Order ID
        order_id: String,
        /// New price
        #[arg(short, long)]
        price: Option<f64>,
        /// New quantity
        #[arg(short, long)]
        quantity: Option<u32>,
    },
    /// Cancel an order
    Cancel {
        /// Order ID
        order_id: String,
    },
    /// Cancel all open orders
    CancelAll,
    /// Show executed trades
    Trades {
        /// Order ID
        order_id: Option<String>,
    },
}

/// Execute orders command
pub async fn execute(
    command: OrdersCommand,
    _dry_run: bool,
    _yes: bool,
    _output_format: OutputFormat,
) -> Result<(), CliError> {
    let config = Config::load()?;
    let api_key = config.api.api_key.as_ref().ok_or(CliError::InvalidCredentials)?.clone();
    let token = config.get_access_token()?;

    let client = KiteClient::new(api_key, token);

    match command {
        OrdersCommand::List { status: _ } => {
            let orders = client.get_orders().await?;
            println!("Orders: {:#?}", orders);
            Ok(())
        }
        OrdersCommand::Get { order_id: _ } => {
            println!("Getting order details...");
            Ok(())
        }
        OrdersCommand::Place { symbol, order_type, quantity, price, product } => {
            let parts: Vec<&str> = symbol.split(':').collect();
            if parts.len() != 2 {
                return Err(CliError::Validation("Symbol must be in format EXCHANGE:SYMBOL".to_string()));
            }
            
            let order = PlaceOrder {
                exchange: parts[0].to_string(),
                tradingsymbol: parts[1].to_string(),
                transaction_type: TransactionType::from_str(order_type.to_uppercase().as_str())
                    .map_err(|_| CliError::Validation("Invalid order type".to_string()))?,
                quantity,
                order_type: OrderType::Limit,
                product: Some(Product::from_str(product.to_uppercase().as_str())
                    .map_err(|_| CliError::Validation("Invalid product".to_string()))?),
                price: Some(price),
                trigger_price: None,
                validity: None,
                disclosed_quantity: None,
                squareoff: None,
                stoploss: None,
                trailing_stoploss: None,
                variety: None,
            };
            
            let result = client.place_order(order).await?;
            println!("Order placed: {}", result.order_id);
            Ok(())
        }
        _ => {
            println!("Command not yet implemented");
            Ok(())
        }
    }
}
