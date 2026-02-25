//! Input validation for orders and symbols

use crate::api::types::{OrderType, Product, TransactionType, Validity};
use crate::error::CliError;

/// Validate order parameters
pub fn validate_order(
    order_type: OrderType,
    quantity: u32,
    price: Option<f64>,
    trigger_price: Option<f64>,
) -> Result<(), CliError> {
    // Quantity must be positive
    if quantity == 0 {
        return Err(CliError::Validation("Quantity must be greater than 0".to_string()));
    }

    // Validate order type and required fields
    match order_type {
        OrderType::Market => {
            // Market orders don't need price or trigger_price
        }
        OrderType::Limit => {
            // Limit orders need price
            if price.is_none() || price.unwrap() <= 0.0 {
                return Err(CliError::Validation(
                    "Limit orders require a valid price (> 0)".to_string(),
                ));
            }
        }
        OrderType::StopLoss => {
            // SL orders need both price and trigger_price
            if price.is_none() || price.unwrap() <= 0.0 {
                return Err(CliError::Validation(
                    "Stop Loss orders require a valid price (> 0)".to_string(),
                ));
            }
            if trigger_price.is_none() || trigger_price.unwrap() <= 0.0 {
                return Err(CliError::Validation(
                    "Stop Loss orders require a valid trigger price (> 0)".to_string(),
                ));
            }
        }
        OrderType::StopLossMarket => {
            // SL-M orders need trigger_price
            if trigger_price.is_none() || trigger_price.unwrap() <= 0.0 {
                return Err(CliError::Validation(
                    "Stop Loss Market orders require a valid trigger price (> 0)".to_string(),
                ));
            }
        }
    }

    Ok(())
}

/// Validate symbol format (EXCHANGE:SYMBOL)
pub fn validate_symbol(symbol: &str) -> Result<(String, String), CliError> {
    let parts: Vec<&str> = symbol.split(':').collect();
    if parts.len() != 2 {
        return Err(CliError::Validation(
            "Invalid symbol format. Expected: EXCHANGE:SYMBOL (e.g., NSE:INFY)".to_string(),
        ));
    }

    let exchange = parts[0].to_uppercase();
    let tradingsymbol = parts[1].to_uppercase();

    if tradingsymbol.is_empty() {
        return Err(CliError::Validation(
            "Symbol cannot be empty".to_string(),
        ));
    }

    // Validate exchange
    match exchange.as_str() {
        "NSE" | "BSE" | "NFO" | "BFO" | "MCX" | "CDS" => {}
        _ => {
            return Err(CliError::Validation(format!(
                "Invalid exchange '{}'. Valid exchanges: NSE, BSE, NFO, BFO, MCX, CDS",
                exchange
            )))
        }
    }

    Ok((exchange, tradingsymbol))
}

/// Parse transaction type string
pub fn parse_transaction_type(s: &str) -> Result<TransactionType, CliError> {
    match s.to_uppercase().as_str() {
        "BUY" => Ok(TransactionType::Buy),
        "SELL" => Ok(TransactionType::Sell),
        _ => Err(CliError::Validation(format!(
            "Invalid transaction type '{}'. Valid types: BUY, SELL",
            s
        ))),
    }
}

/// Parse product type string
pub fn parse_product_type(s: &str) -> Result<Product, CliError> {
    match s.to_uppercase().as_str() {
        "CNC" => Ok(Product::CNC),
        "MIS" => Ok(Product::MIS),
        "NRML" => Ok(Product::NRML),
        "MTF" => Ok(Product::MTF),
        _ => Err(CliError::Validation(format!(
            "Invalid product type '{}'. Valid types: CNC, MIS, NRML, MTF",
            s
        ))),
    }
}

/// Parse order type string
pub fn parse_order_type(s: &str) -> Result<OrderType, CliError> {
    match s.to_uppercase().as_str() {
        "MARKET" => Ok(OrderType::Market),
        "LIMIT" => Ok(OrderType::Limit),
        "SL" => Ok(OrderType::StopLoss),
        "SL-M" => Ok(OrderType::StopLossMarket),
        _ => Err(CliError::Validation(format!(
            "Invalid order type '{}'. Valid types: MARKET, LIMIT, SL, SL-M",
            s
        ))),
    }
}

/// Parse validity string
pub fn parse_validity(s: &str) -> Result<Validity, CliError> {
    match s.to_uppercase().as_str() {
        "DAY" => Ok(Validity::Day),
        "IOC" => Ok(Validity::IOC),
        "TTL" => Ok(Validity::TTL),
        _ => Err(CliError::Validation(format!(
            "Invalid validity '{}'. Valid types: DAY, IOC, TTL",
            s
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_symbol_valid() {
        let result = validate_symbol("NSE:INFY");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ("NSE".to_string(), "INFY".to_string()));
    }

    #[test]
    fn test_validate_symbol_invalid_format() {
        let result = validate_symbol("INFY");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_symbol_invalid_exchange() {
        let result = validate_symbol("INVALID:INFY");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_order_market() {
        let result = validate_order(OrderType::Market, 10, None, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_order_limit_with_price() {
        let result = validate_order(OrderType::Limit, 10, Some(100.0), None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_order_limit_without_price() {
        let result = validate_order(OrderType::Limit, 10, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_order_zero_quantity() {
        let result = validate_order(OrderType::Market, 0, None, None);
        assert!(result.is_err());
    }
}
