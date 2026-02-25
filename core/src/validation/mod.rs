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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{OrderType, Product};

    #[test]
    fn test_validate_order_valid_limit() {
        let result = validate_order(
            OrderType::Limit,
            10,
            1400.0,
            None,
            Product::CNC,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_order_valid_market() {
        let result = validate_order(
            OrderType::Market,
            10,
            1000.0, // Price must be > 0 even for market orders per validation logic
            None,
            Product::MIS,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_order_valid_sl() {
        let result = validate_order(
            OrderType::SL,
            10,
            1400.0,
            Some(1395.0),
            Product::NRML,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_order_quantity_zero() {
        let result = validate_order(
            OrderType::Limit,
            0,
            1400.0,
            None,
            Product::CNC,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Quantity must be greater than 0"));
    }

    #[test]
    fn test_validate_order_quantity_negative() {
        let result = validate_order(
            OrderType::Limit,
            -10,
            1400.0,
            None,
            Product::CNC,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Quantity must be greater than 0"));
    }

    #[test]
    fn test_validate_order_price_zero() {
        let result = validate_order(
            OrderType::Limit,
            10,
            0.0,
            None,
            Product::CNC,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Price must be greater than 0"));
    }

    #[test]
    fn test_validate_order_price_negative() {
        let result = validate_order(
            OrderType::Limit,
            10,
            -1400.0,
            None,
            Product::CNC,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Price must be greater than 0"));
    }

    #[test]
    fn test_validate_order_sl_without_trigger() {
        let result = validate_order(
            OrderType::SL,
            10,
            1400.0,
            None,
            Product::CNC,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Stop Loss orders require a trigger price"));
    }

    #[test]
    fn test_validate_order_slm_without_trigger() {
        let result = validate_order(
            OrderType::SLM,
            10,
            1400.0,
            None,
            Product::CNC,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Stop Loss Market orders require a trigger price"));
    }

    #[test]
    fn test_validate_symbol_valid_nse() {
        let result = validate_symbol("NSE:INFY");
        assert!(result.is_ok());
        let (exchange, symbol) = result.unwrap();
        assert_eq!(exchange, "NSE");
        assert_eq!(symbol, "INFY");
    }

    #[test]
    fn test_validate_symbol_valid_bse() {
        let result = validate_symbol("BSE:RELIANCE");
        assert!(result.is_ok());
        let (exchange, symbol) = result.unwrap();
        assert_eq!(exchange, "BSE");
        assert_eq!(symbol, "RELIANCE");
    }

    #[test]
    fn test_validate_symbol_case_insensitive() {
        let result = validate_symbol("nse:infy");
        assert!(result.is_ok());
        let (exchange, symbol) = result.unwrap();
        assert_eq!(exchange, "NSE");
        assert_eq!(symbol, "INFY");
    }

    #[test]
    fn test_validate_symbol_no_colon() {
        let result = validate_symbol("NSEINFY");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid symbol format"));
    }

    #[test]
    fn test_validate_symbol_invalid_exchange() {
        let result = validate_symbol("XYZ:INFY");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid exchange"));
    }

    #[test]
    fn test_validate_symbol_too_many_colons() {
        let result = validate_symbol("NSE:INFY:EXTRA");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid symbol format"));
    }
}
