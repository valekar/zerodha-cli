//! Domain models for Kite Connect API

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;

// ==================== INSTRUMENTS ====================

/// Instrument (trading symbol)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instrument {
    pub instrument_token: u64,
    pub exchange_token: u64,
    pub tradingsymbol: String,
    pub name: String,
    pub last_price: Option<f64>,
    pub expiry: Option<String>,
    pub strike: Option<f64>,
    pub tick_size: f64,
    pub lot_size: u32,
    pub instrument_type: InstrumentType,
    pub segment: Segment,
    pub exchange: Exchange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstrumentType {
    #[serde(rename = "EQ")]
    Equity,
    #[serde(rename = "CE")]
    CallOption,
    #[serde(rename = "PE")]
    PutOption,
    #[serde(rename = "FUT")]
    Future,
    #[serde(rename = "F")]
    FutureAbbrev,
    #[serde(rename = "O")]
    Option,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Segment {
    #[serde(rename = "NSE")]
    NSE,
    #[serde(rename = "BSE")]
    BSE,
    #[serde(rename = "NFO")]
    NFO,
    #[serde(rename = "BFO")]
    BFO,
    #[serde(rename = "MCX")]
    MCX,
    #[serde(rename = "CDS")]
    CDS,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Exchange {
    #[serde(rename = "NSE")]
    NSE,
    #[serde(rename = "BSE")]
    BSE,
    #[serde(rename = "NFO")]
    NFO,
    #[serde(rename = "BFO")]
    BFO,
    #[serde(rename = "MCX")]
    MCX,
    #[serde(rename = "CDS")]
    CDS,
}

impl Display for Exchange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Exchange::NSE => write!(f, "NSE"),
            Exchange::BSE => write!(f, "BSE"),
            Exchange::NFO => write!(f, "NFO"),
            Exchange::BFO => write!(f, "BFO"),
            Exchange::MCX => write!(f, "MCX"),
            Exchange::CDS => write!(f, "CDS"),
        }
    }
}

// ==================== ORDERS ====================

/// Order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub order_id: String,
    pub exchange_order_id: Option<String>,
    pub parent_order_id: Option<String>,
    pub status: OrderStatus,
    pub status_message: Option<String>,
    pub tradingsymbol: String,
    pub exchange: Exchange,
    pub variety: OrderVariety,
    pub order_type: OrderType,
    pub transaction_type: TransactionType,
    pub validity: Validity,
    pub product: Product,
    pub quantity: i32,
    pub disclosed_quantity: Option<i32>,
    pub price: f64,
    pub trigger_price: Option<f64>,
    pub average_price: Option<f64>,
    pub pending_quantity: i32,
    pub filled_quantity: i32,
    pub cancelled_quantity: i32,
    pub placed_by: String,
    pub order_timestamp: String,
    pub update_timestamp: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    Open,
    Complete,
    Cancelled,
    Rejected,
    #[serde(rename = "TRIGGER PENDING")]
    TriggerPending,
    #[serde(rename = "VALIDATION PENDING")]
    ValidationPending,
}

impl Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderStatus::Open => write!(f, "OPEN"),
            OrderStatus::Complete => write!(f, "COMPLETE"),
            OrderStatus::Cancelled => write!(f, "CANCELLED"),
            OrderStatus::Rejected => write!(f, "REJECTED"),
            OrderStatus::TriggerPending => write!(f, "TRIGGER PENDING"),
            OrderStatus::ValidationPending => write!(f, "VALIDATION PENDING"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderVariety {
    Regular,
    AMO,
    CO,
    Iceberg,
}

impl Display for OrderVariety {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderVariety::Regular => write!(f, "regular"),
            OrderVariety::AMO => write!(f, "amo"),
            OrderVariety::CO => write!(f, "co"),
            OrderVariety::Iceberg => write!(f, "iceberg"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Market,
    Limit,
    SL,
    #[serde(rename = "SL-M")]
    SLM,
}

impl Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderType::Market => write!(f, "MARKET"),
            OrderType::Limit => write!(f, "LIMIT"),
            OrderType::SL => write!(f, "SL"),
            OrderType::SLM => write!(f, "SL-M"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionType {
    Buy,
    Sell,
}

impl Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionType::Buy => write!(f, "BUY"),
            TransactionType::Sell => write!(f, "SELL"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Validity {
    Day,
    IOC,
    TTL,
}

impl Display for Validity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Validity::Day => write!(f, "DAY"),
            Validity::IOC => write!(f, "IOC"),
            Validity::TTL => write!(f, "TTL"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Product {
    CNC,
    MIS,
    NRML,
    MTF,
    BO,
}

impl Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Product::CNC => write!(f, "CNC"),
            Product::MIS => write!(f, "MIS"),
            Product::NRML => write!(f, "NRML"),
            Product::MTF => write!(f, "MTF"),
            Product::BO => write!(f, "BO"),
        }
    }
}

// ==================== TRADES ====================

/// Trade
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub trade_id: String,
    pub order_id: String,
    pub exchange_order_id: Option<String>,
    pub tradingsymbol: String,
    pub exchange: Exchange,
    pub transaction_type: TransactionType,
    pub product: Product,
    pub average_price: f64,
    pub quantity: i32,
    pub fill_timestamp: String,
    #[serde(rename = "trade_timestamp")]
    pub trade_timestamp: Option<String>,
}

// ==================== QUOTES ====================

/// Quote
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub instrument_token: u64,
    pub last_price: f64,
    pub ohlc: OHLC,
    pub depth: Depth,
    pub oi: Option<i64>,
    pub oi_day_high: Option<i64>,
    pub oi_day_low: Option<i64>,
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OHLC {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepthEntry {
    pub quantity: i32,
    pub price: f64,
    pub orders: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Depth {
    pub buy: Vec<DepthEntry>,
    pub sell: Vec<DepthEntry>,
}

/// Quote response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteResponse {
    pub data: HashMap<String, Quote>,
}

/// OHLC response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OHLCResponse {
    pub data: HashMap<String, OHLCData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OHLCData {
    pub instrument_token: u64,
    pub last_price: f64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

/// LTP response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LTPResponse {
    pub data: HashMap<String, LTPData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LTPData {
    pub instrument_token: u64,
    pub last_price: f64,
}

// ==================== PORTFOLIO ====================

/// Holding (long-term equity)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Holding {
    pub tradingsymbol: String,
    pub exchange: Exchange,
    pub instrument_token: u64,
    pub isin: String,
    pub quantity: i32,
    pub authorised_quantity: i32,
    pub average_price: f64,
    pub last_price: f64,
    pub close_price: f64,
    pub pnl: f64,
    pub day_change: f64,
    pub day_change_percentage: f64,
}

/// Position (intraday/F&O)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub tradingsymbol: String,
    pub exchange: Exchange,
    pub instrument_token: u64,
    pub product: Product,
    pub quantity: i32,
    pub overnight_quantity: i32,
    pub multiplier: i32,
    pub average_price: f64,
    pub close_price: f64,
    pub last_price: f64,
    pub pnl: f64,
    pub m2m: f64,
    pub unrealised: f64,
    pub realised: f64,
    pub buy_quantity: i32,
    pub buy_price: f64,
    pub buy_value: f64,
    pub buy_m2m: f64,
    pub sell_quantity: i32,
    pub sell_price: f64,
    pub sell_value: f64,
    pub sell_m2m: f64,
    pub day_buy_quantity: i32,
    pub day_buy_price: f64,
    pub day_buy_value: f64,
    pub day_sell_quantity: i32,
    pub day_sell_price: f64,
    pub day_sell_value: f64,
}

/// Positions response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionsResponse {
    pub net: Vec<Position>,
    pub day: Vec<Position>,
}

// ==================== MARGINS ====================

/// Margin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Margin {
    pub enabled: bool,
    pub net: f64,
    pub available: MarginDetail,
    pub utilised: MarginUtilised,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginDetail {
    pub cash: f64,
    pub opening_balance: f64,
    pub live_balance: f64,
    pub collateral: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginUtilised {
    pub debits: f64,
    pub exposure: f64,
    pub options_premium: f64,
    pub payout: f64,
    pub span: f64,
    pub holding_sales: f64,
    pub turnaround: f64,
    pub m2m_unrealised: f64,
    pub m2m_realised: f64,
    pub stock_collateral: f64,
}

/// Margin response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginResponse {
    pub equity: Margin,
    pub commodity: Margin,
}

/// Equity margins
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquityMargins {
    pub equity: Margin,
}

/// Commodity margins
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommodityMargins {
    pub commodity: Margin,
}

// ==================== GTT ====================

/// GTT (Good Till Triggered)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GTTTrigger {
    pub id: u64,
    pub user_id: String,
    pub tradingsymbol: String,
    pub exchange: Exchange,
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
    pub updated_at: Option<String>,
    pub expires_at: Option<String>,
    pub status: String,
}

// ==================== REQUEST/RESPONSE ====================

/// Place order request
#[derive(Debug, Clone, Serialize)]
pub struct PlaceOrder {
    pub exchange: String,
    pub tradingsymbol: String,
    pub transaction_type: TransactionType,
    pub quantity: u32,
    pub order_type: OrderType,
    pub product: Product,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity: Option<Validity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disclosed_quantity: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variety: Option<String>,
}

/// Place order response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    pub order_id: String,
    pub status: OrderStatus,
    pub status_message: Option<String>,
}

/// Cancel order response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelResponse {
    pub order_id: String,
    pub status: String,
}

/// Modify order request
#[derive(Debug, Clone, Serialize)]
pub struct ModifyOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity: Option<Validity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disclosed_quantity: Option<u32>,
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

/// Place GTT request
#[derive(Debug, Clone, Serialize)]
pub struct PlaceGTT {
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
pub struct ModifyGTT {
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GTTResponse {
    pub trigger_id: u64,
    pub status: String,
}

// ==================== SESSION ====================

/// Session response
#[derive(Debug, Clone, Deserialize)]
pub struct SessionResponse {
    pub user_id: String,
    pub access_token: String,
    pub enctoken: Option<String>,
    pub public_token: Option<String>,
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
