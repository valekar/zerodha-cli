//! API endpoint definitions

/// Base URL for Kite Connect API
pub const BASE_URL: &str = "https://api.kite.trade";

/// API endpoints
pub mod endpoints {
    pub const SESSION_TOKEN: &str = "/session/token";
    pub const SESSION_LOGOUT: &str = "/session/logout";
    
    pub const INSTRUMENTS: &str = "/instruments";
    pub const INSTRUMENTS_META: &str = "/instruments/metadata";
    
    pub const QUOTE: &str = "/quote";
    pub const QUOTE_LTP: &str = "/quote/ltp";
    pub const QUOTE_OHLC: &str = "/quote/ohlc";
    pub const QUOTE_HISTORICAL: &str = "/quote/historical";
    
    pub const ORDERS: &str = "/orders";
    pub const ORDERS_REGULAR: &str = "/orders/regular";
    pub const ORDERS_AMO: &str = "/orders/amo";
    pub const ORDERS_BO: &str = "/orders/bo";
    pub const ORDERS_CO: &str = "/orders/co";
    
    pub const ORDER_INFO: &str = "/orders";
    pub const ORDER_MODIFY: &str = "/orders";
    pub const ORDER_CANCEL: &str = "/orders";
    
    pub const TRADES: &str = "/trades";
    
    pub const PORTFOLIO_HOLDINGS: &str = "/portfolio/holdings";
    pub const PORTFOLIO_POSITIONS: &str = "/portfolio/positions";
    pub const PORTFOLIO_CONVERT: &str = "/portfolio/convert_position";
    
    pub const MARGINS: &str = "/margins";
    pub const MARGINS_EQUITY: &str = "/margins/equity";
    pub const MARGINS_COMMODITY: &str = "/margins/commodity";
    pub const MARGINS_ORDERS: &str = "/margins/orders";
    
    pub const GTT_TRIGGERS: &str = "/gtt/triggers";
    pub const GTT_PLACE: &str = "/gtt/places";
    pub const GTT_MODIFY: &str = "/gtt/modifies";
    pub const GTT_DELETE: &str = "/gtt/deletes";
    
    pub const USER_PROFILE: &str = "/user/profile";
    pub const USER_MARGINS: &str = "/user/margins";
    pub const USER_ALERTS: &str = "/user/alerts";
}
