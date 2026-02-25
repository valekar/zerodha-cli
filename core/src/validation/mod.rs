//! Validation module

use crate::models::{OrderType, Product};
use anyhow::{bail, Result};

/// Validate order parameters
pub fn validate_order(
    order_type: OrderType,
    quantity: i32,
    price: f64,
    trigger_price: Option<f64>,
    _product: Product,
) -> Result<()> {
    // Quantity must be positive
    if quantity <= 0 {
        bail!("Quantity must be greater than 0");
    }

    // Price must be positive
    if price <= 0.0 {
        bail!("Price must be greater than 0");
    }

    // Validate order type requirements
    match order_type {
        OrderType::Market => {}
        OrderType::Limit => {}
        OrderType::SL => {
            if trigger_price.is_none() {
                bail!("Stop Loss orders require a trigger price");
            }
        }
        OrderType::SLM => {
            if trigger_price.is_none() {
                bail!("Stop Loss Market orders require a trigger price");
            }
        }
    }

    Ok(())
}

/// Validate symbol format (EXCHANGE:SYMBOL)
pub fn validate_symbol(symbol: &str) -> Result<(String, String)> {
    let parts: Vec<&str> = symbol.split(':').collect();
    if parts.len() != 2 {
        bail!("Invalid symbol format. Expected: EXCHANGE:SYMBOL (e.g., NSE:INFY)");
    }

    let exchange = parts[0].to_uppercase();
    let tradingsymbol = parts[1].to_uppercase();

    // Validate exchange
    match exchange.as_str() {
        "NSE" | "BSE" | "NFO" | "BFO" | "MCX" | "CDS" => {}
        _ => bail!("Invalid exchange. Valid exchanges: NSE, BSE, NFO, BFO, MCX, CDS"),
    }

    Ok((exchange, tradingsymbol))
}
