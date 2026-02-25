//! API types for Kite Connect

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Instrument key (e.g., "NSE:INFY")
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct InstrumentKey {
    pub exchange: String,
    pub trading_symbol: String,
}

impl std::fmt::Display for InstrumentKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.exchange, self.trading_symbol)
    }
}

impl std::str::FromStr for InstrumentKey {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err("Invalid instrument key format. Expected: EXCHANGE:SYMBOL".to_string());
        }
        Ok(InstrumentKey {
            exchange: parts[0].to_uppercase(),
            trading_symbol: parts[1].to_uppercase(),
        })
    }
}

/// Instrument from CSV
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instrument {
    pub instrument_token: u64,
    pub exchange_token: String,
    pub tradingsymbol: String,
    pub name: String,
    pub exchange: String,
    pub instrument_type: String,
    pub segment: String,
    pub lot_size: u32,
    pub tick_size: f64,
}

/// Order type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Market,
    Limit,
    StopLoss,
    StopLossMarket,
}

/// Product type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Product {
    CNC,
    MIS,
    NRML,
    MTF,
}

impl std::str::FromStr for Product {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "CNC" => Ok(Product::CNC),
            "MIS" => Ok(Product::MIS),
            "NRML" => Ok(Product::NRML),
            "MTF" => Ok(Product::MTF),
            _ => Err(format!("Invalid product type: {}", s)),
        }
    }
}

/// Validity
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Validity {
    Day,
    IOC,
    TTL,
}

/// Transaction type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionType {
    Buy,
    Sell,
}

impl std::str::FromStr for TransactionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "BUY" => Ok(TransactionType::Buy),
            "SELL" => Ok(TransactionType::Sell),
            _ => Err(format!("Invalid transaction type: {}", s)),
        }
    }
}

/// Order status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    Complete,
    Rejected,
    Cancelled,
    Open,
    TriggerPending,
    ValidityExpired,
}

/// Place order request
#[derive(Debug, Clone, Serialize)]
pub struct PlaceOrder {
    pub exchange: String,
    pub tradingsymbol: String,
    pub transaction_type: TransactionType,
    pub quantity: u32,
    pub order_type: OrderType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<Product>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity: Option<Validity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disclosed_quantity: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub squareoff: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stoploss: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_stoploss: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variety: Option<String>,
}

/// Order response
#[derive(Debug, Clone, Deserialize)]
pub struct OrderResponse {
    pub order_id: String,
    #[serde(rename = "status")]
    pub status: OrderStatus,
    pub status_message: Option<String>,
}

/// Order details
#[derive(Debug, Clone, Deserialize)]
pub struct Order {
    pub order_id: String,
    pub exchange_order_id: Option<String>,
    pub placed_by: String,
    pub order_timestamp: String,
    pub exchange_timestamp: Option<String>,
    pub exchange_update_timestamp: Option<String>,
    pub exchange: String,
    pub tradingsymbol: String,
    pub transaction_type: TransactionType,
    pub validity: Validity,
    pub product: Product,
    pub quantity: u32,
    pub disclosed_quantity: u32,
    pub price: f64,
    pub trigger_price: f64,
    pub average_price: f64,
    pub pending_quantity: u32,
    pub cancelled_quantity: u32,
    pub filled_quantity: u32,
    pub status: OrderStatus,
    pub status_message: Option<String>,
    pub order_type: OrderType,
    pub variety: String,
}

/// Trade
#[derive(Debug, Clone, Deserialize)]
pub struct Trade {
    pub trade_id: String,
    pub order_id: String,
    pub exchange: String,
    pub tradingsymbol: String,
    pub transaction_type: TransactionType,
    pub product: Product,
    pub average_price: f64,
    pub quantity: u32,
    pub fill_timestamp: String,
}

/// Quote
#[derive(Debug, Clone, Deserialize)]
pub struct Quote {
    pub instrument_token: u64,
    pub last_price: f64,
    pub ohlc: Ohlc,
    pub depth: Depth,
}

/// OHLC
#[derive(Debug, Clone, Deserialize)]
pub struct Ohlc {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

/// Depth
#[derive(Debug, Clone, Deserialize)]
pub struct Depth {
    pub buy: Vec<[f64; 5]>,
    pub sell: Vec<[f64; 5]>,
}

/// Quote response
#[derive(Debug, Clone, Deserialize)]
pub struct QuoteResponse {
    pub status: String,
    pub data: HashMap<String, Quote>,
}

/// LTP response
#[derive(Debug, Clone, Deserialize)]
pub struct LtpResponse {
    pub status: String,
    pub data: HashMap<String, LtpData>,
}

/// LTP data
#[derive(Debug, Clone, Deserialize)]
pub struct LtpData {
    pub instrument_token: u64,
    pub last_price: f64,
}

/// OHLC response
#[derive(Debug, Clone, Deserialize)]
pub struct OhlcResponse {
    pub status: String,
    pub data: HashMap<String, Ohlc>,
}

/// Holding
#[derive(Debug, Clone, Deserialize)]
pub struct Holding {
    pub tradingsymbol: String,
    pub exchange: String,
    pub instrument_token: u64,
    pub isin: String,
    pub quantity: u32,
    pub authorised_quantity: u32,
    pub price: f64,
    pub last_price: f64,
    pub average_price: f64,
    pub pnl: f64,
    pub day_change: f64,
    pub day_change_percentage: f64,
}

/// Position
#[derive(Debug, Clone, Deserialize)]
pub struct Position {
    pub tradingsymbol: String,
    pub exchange: String,
    pub instrument_token: u64,
    pub product: Product,
    pub quantity: i32,
    pub overnight_quantity: i32,
    pub multiplier: u32,
    pub average_price: f64,
    pub last_price: f64,
    pub unrealised: f64,
    pub realised: f64,
    pub buy_price: f64,
    pub sell_price: f64,
    pub buy_quantity: u32,
    pub sell_quantity: u32,
}

/// Positions response
#[derive(Debug, Clone, Deserialize)]
pub struct PositionsResponse {
    pub net: Vec<Position>,
    pub day: Vec<Position>,
}

/// Convert position request
#[derive(Debug, Clone, Serialize)]
pub struct ConvertPosition {
    pub exchange: String,
    pub tradingsymbol: String,
    pub transaction_type: TransactionType,
    pub quantity: u32,
    pub from_product: Product,
    pub to_product: Product,
}

/// Margin
#[derive(Debug, Clone, Deserialize)]
pub struct Margin {
    pub enabled: bool,
    pub net: f64,
    pub available: f64,
    pub used: f64,
    pub opening_balance: f64,
    pub closing_balance: f64,
    pub deployed: f64,
    pub intraday_payin: f64,
}

/// Margin response
#[derive(Debug, Clone, Deserialize)]
pub struct MarginResponse {
    pub status: String,
    pub data: HashMap<String, Margin>,
}

/// GTT trigger
#[derive(Debug, Clone, Deserialize)]
pub struct GttTrigger {
    pub id: u64,
    pub user_id: String,
    pub tradingsymbol: String,
    pub exchange: String,
    pub transaction_type: TransactionType,
    pub product: Product,
    pub order_type: OrderType,
    pub quantity: u32,
    pub price: f64,
    pub trigger_price: f64,
    pub last_price: f64,
    pub trailing_stoploss: Option<f64>,
    pub stoploss: Option<f64>,
    pub squareoff: Option<f64>,
    pub generated_at: String,
    pub updated_at: String,
    pub expires_at: Option<String>,
    pub status: String,
}

/// Place GTT request
#[derive(Debug, Clone, Serialize)]
pub struct PlaceGtt {
    pub tradingsymbol: String,
    pub exchange: String,
    pub transaction_type: TransactionType,
    pub product: Product,
    pub order_type: OrderType,
    pub quantity: u32,
    pub price: f64,
    pub trigger_price: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_stoploss: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stoploss: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub squareoff: Option<f64>,
}

/// Modify GTT request
#[derive(Debug, Clone, Serialize)]
pub struct ModifyGtt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<OrderType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_stoploss: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stoploss: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub squareoff: Option<f64>,
}

/// GTT response
#[derive(Debug, Clone, Deserialize)]
pub struct GttResponse {
    pub trigger_id: u64,
    pub status: String,
}

/// Session response
#[derive(Debug, Clone, Deserialize)]
pub struct SessionResponse {
    pub user_id: String,
    pub access_token: String,
    pub enctoken: Option<String>,
    pub public_token: String,
    pub refresh_token: Option<String>,
    pub login_time: String,
    pub user_name: String,
    pub user_type: String,
    pub avatar_url: Option<String>,
    pub broker: String,
    pub exchanges: Vec<String>,
    pub products: Vec<String>,
    pub order_types: Vec<String>,
}
