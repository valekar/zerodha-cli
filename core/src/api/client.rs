//! Kite Connect API Client

use crate::api::rate_limiter::RateLimiter;
use crate::error::ZerodhaError;
use crate::models::*;
use anyhow::{Context, Result};
use reqwest::{Client, Method, RequestBuilder, StatusCode};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// Crypto imports
use hex;
use sha2::Digest;

#[allow(unused_imports)]
use serde::Deserialize;

/// Kite Connect API client
pub struct KiteConnectClient {
    http_client: Client,
    api_key: String,
    api_secret: String,
    access_token: Arc<RwLock<Option<String>>>,
    base_url: String,
    rate_limiter: RateLimiter,
}

impl KiteConnectClient {
    /// Create a new API client
    pub fn new(api_key: String, api_secret: String) -> Self {
        Self {
            http_client: Client::builder()
                .use_rustls_tls()
                .build()
                .expect("Failed to create HTTP client"),
            api_key,
            api_secret,
            access_token: Arc::new(RwLock::new(None)),
            base_url: "https://api.kite.trade".to_string(),
            rate_limiter: RateLimiter::new(),
        }
    }

    /// Set access token after OAuth
    pub async fn set_access_token(&self, token: String) -> Result<()> {
        let mut guard = self.access_token.write().await;
        *guard = Some(token);
        Ok(())
    }

    /// Get current access token
    pub async fn get_access_token(&self) -> Result<String> {
        let guard = self.access_token.read().await;
        guard
            .clone()
            .ok_or_else(|| anyhow::anyhow!("Not authenticated"))
    }

    /// Check if authenticated
    pub async fn is_authenticated(&self) -> bool {
        let guard = self.access_token.read().await;
        guard.is_some()
    }

    /// Build an authenticated request
    fn build_request(&self, method: Method, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.base_url, path);
        self.http_client
            .request(method, &url)
            .header("X-Kite-Version", "3")
            .header("User-Agent", "zerodha-cli/1.0.0")
    }

    /// Build an authenticated request with access token
    async fn build_auth_request(&self, method: Method, path: &str) -> Result<RequestBuilder> {
        let access_token = self.get_access_token().await?;
        let url = format!("{}{}", self.base_url, path);

        Ok(self
            .http_client
            .request(method, &url)
            .header("X-Kite-Version", "3")
            .header(
                "Authorization",
                format!("token {}:{}", self.api_key, access_token),
            )
            .header("User-Agent", "zerodha-cli/1.0.0"))
    }

    /// Execute a request with rate limiting and error handling
    async fn execute<T: DeserializeOwned>(&self, req_builder: RequestBuilder) -> Result<T> {
        // Acquire rate limit permit
        self.rate_limiter.acquire().await?;

        // Send request
        let response = req_builder.send().await.context("Failed to send request")?;

        let status = response.status();

        // Handle error responses
        if !status.is_success() {
            return self.handle_error(status, response).await;
        }

        // Parse response
        let text = response
            .text()
            .await
            .context("Failed to read response text")?;

        serde_json::from_str(&text).context("Failed to parse response JSON")
    }

    /// Handle API error responses
    async fn handle_error<T>(&self, status: StatusCode, response: reqwest::Response) -> Result<T> {
        let text = response
            .text()
            .await
            .unwrap_or_else(|_| String::from("Failed to read error response"));

        let status_code = status.as_u16();

        // Redact sensitive information from error messages
        let redacted_text = redact_secrets(&text);

        match status_code {
            401 => Err(anyhow::anyhow!(
                "Authentication failed: {}. Please run 'kite auth login'",
                redacted_text
            )),
            403 => Err(anyhow::anyhow!(
                "Forbidden: {}. Access denied",
                redacted_text
            )),
            429 => Err(ZerodhaError::RateLimit.into()),
            400..=499 => Err(anyhow::anyhow!("Client error: {}", redacted_text)),
            500..=599 => Err(anyhow::anyhow!(
                "Server error: {}. Please try again later",
                redacted_text
            )),
            _ => Err(anyhow::anyhow!("Unexpected error: {}", redacted_text)),
        }
    }

    // ==================== AUTH API ====================

    /// Generate login URL for OAuth flow
    pub fn login_url(&self) -> String {
        format!(
            "https://kite.zerodha.com/connect/login?v=3&api_key={}",
            self.api_key
        )
    }

    /// Exchange request token for access token
    pub async fn exchange_token(&self, request_token: &str) -> Result<String> {
        // Generate checksum: SHA256(api_key + request_token + api_secret)
        let checksum_input = format!("{}{}{}", self.api_key, request_token, self.api_secret);
        let checksum = sha256_digest(&checksum_input);

        let body = serde_json::json!({
            "api_key": self.api_key,
            "request_token": request_token,
            "checksum": checksum,
        });

        let req = self
            .build_request(Method::POST, "/session/token")
            .json(&body);

        #[derive(Deserialize)]
        #[allow(dead_code)]
        struct SessionData {
            #[allow(dead_code)]
            user_id: String,
            access_token: String,
        }

        let response: SessionData = self.execute(req).await?;

        // Store access token
        self.set_access_token(response.access_token.clone()).await?;

        Ok(response.access_token)
    }

    // ==================== INSTRUMENTS API ====================

    /// List all instruments from exchange
    pub async fn list_instruments(&self, exchange: Option<&str>) -> Result<Vec<Instrument>> {
        let path = match exchange {
            Some(ex) => format!("/instruments/{}", ex.to_lowercase()),
            None => "/instruments".to_string(),
        };

        let req = self.build_auth_request(Method::GET, &path).await?;

        // Instruments are returned as CSV text
        self.rate_limiter.acquire().await?;
        let response = req.send().await.context("Failed to fetch instruments")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "Failed to fetch instruments: {} - {}",
                status,
                text
            ));
        }

        let text = response
            .text()
            .await
            .context("Failed to read instruments CSV")?;
        let mut rdr = csv::Reader::from_reader(text.as_bytes());
        let mut instruments = Vec::new();

        for result in rdr.deserialize() {
            let instrument: Instrument = result.context("Failed to parse instrument")?;
            instruments.push(instrument);
        }

        Ok(instruments)
    }

    /// Get specific instrument by exchange and symbol
    pub async fn get_instrument(&self, exchange: &str, symbol: &str) -> Result<Instrument> {
        let instruments = self.list_instruments(Some(exchange)).await?;

        instruments
            .into_iter()
            .find(|inst| inst.tradingsymbol.eq_ignore_ascii_case(symbol))
            .ok_or_else(|| anyhow::anyhow!("Instrument not found: {}:{}", exchange, symbol))
    }

    // ==================== QUOTES API ====================

    /// Get quotes for symbols
    pub async fn get_quotes(&self, symbols: &[&str]) -> Result<QuoteResponse> {
        if symbols.is_empty() {
            return Ok(QuoteResponse {
                data: HashMap::new(),
            });
        }

        let symbols_str = symbols.join(",");
        let path = format!("/quote/{}", symbols_str);

        let req = self.build_auth_request(Method::GET, &path).await?;
        self.execute(req).await
    }

    /// Get OHLC data for symbols
    pub async fn get_ohlc(&self, symbols: &[&str]) -> Result<OHLCResponse> {
        if symbols.is_empty() {
            return Ok(OHLCResponse {
                data: HashMap::new(),
            });
        }

        let symbols_str = symbols.join(",");
        let path = format!("/quote/ohlc?i={}", symbols_str);

        let req = self.build_auth_request(Method::GET, &path).await?;
        self.execute(req).await
    }

    /// Get LTP (last traded price) for symbols
    pub async fn get_ltp(&self, symbols: &[&str]) -> Result<LTPResponse> {
        if symbols.is_empty() {
            return Ok(LTPResponse {
                data: HashMap::new(),
            });
        }

        let symbols_str = symbols.join(",");
        let path = format!("/quote/ltp?i={}", symbols_str);

        let req = self.build_auth_request(Method::GET, &path).await?;
        self.execute(req).await
    }

    // ==================== ORDERS API ====================

    /// List all orders
    pub async fn list_orders(&self) -> Result<Vec<Order>> {
        let req = self.build_auth_request(Method::GET, "/orders").await?;

        #[derive(Deserialize)]
        struct OrdersResponse {
            data: Vec<Order>,
        }

        let response: OrdersResponse = self.execute(req).await?;
        Ok(response.data)
    }

    /// Get order details
    pub async fn get_order(&self, order_id: &str) -> Result<Order> {
        let path = format!("/orders/{}", order_id);
        let req = self.build_auth_request(Method::GET, &path).await?;

        #[derive(Deserialize)]
        struct OrderResponse {
            data: Vec<Order>,
        }

        let response: OrderResponse = self.execute(req).await?;
        response
            .data
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("Order not found: {}", order_id))
    }

    /// Place a new order
    pub async fn place_order(&self, order: &PlaceOrder) -> Result<OrderResponse> {
        let req = self
            .build_auth_request(Method::POST, "/orders/regular")
            .await?
            .json(order);
        self.execute(req).await
    }

    /// Modify an existing order
    pub async fn modify_order(&self, order_id: &str, order: &ModifyOrder) -> Result<OrderResponse> {
        let path = format!("/orders/regular/{}", order_id);
        let req = self
            .build_auth_request(Method::PUT, &path)
            .await?
            .json(order);
        self.execute(req).await
    }

    /// Cancel an order
    pub async fn cancel_order(&self, order_id: &str, variety: &str) -> Result<CancelResponse> {
        let path = format!("/orders/{}/{}", variety, order_id);
        let req = self.build_auth_request(Method::DELETE, &path).await?;
        self.execute(req).await
    }

    /// List trades
    pub async fn list_trades(&self, order_id: Option<&str>) -> Result<Vec<Trade>> {
        let path = match order_id {
            Some(id) => format!("/orders/{}{}", id, "/trades"),
            None => "/orders/trades".to_string(),
        };

        let req = self.build_auth_request(Method::GET, &path).await?;

        #[derive(Deserialize)]
        struct TradesResponse {
            data: Vec<Trade>,
        }

        let response: TradesResponse = self.execute(req).await?;
        Ok(response.data)
    }

    // ==================== PORTFOLIO API ====================

    /// Get holdings
    pub async fn get_holdings(&self) -> Result<Vec<Holding>> {
        let req = self
            .build_auth_request(Method::GET, "/portfolio/holdings")
            .await?;

        #[derive(Deserialize)]
        struct HoldingsResponse {
            data: Vec<Holding>,
        }

        let response: HoldingsResponse = self.execute(req).await?;
        Ok(response.data)
    }

    /// Get positions
    pub async fn get_positions(&self) -> Result<PositionsResponse> {
        let req = self
            .build_auth_request(Method::GET, "/portfolio/positions")
            .await?;
        self.execute(req).await
    }

    /// Convert position
    pub async fn convert_position(&self, req: &ConvertPosition) -> Result<()> {
        let http_req = self
            .build_auth_request(Method::PUT, "/portfolio/positions")
            .await?
            .json(req);
        self.execute(http_req).await
    }

    // ==================== MARGINS API ====================

    /// Get margins
    pub async fn get_margins(&self) -> Result<MarginResponse> {
        let req = self
            .build_auth_request(Method::GET, "/user/margins")
            .await?;
        self.execute(req).await
    }

    /// Get equity margins
    pub async fn get_equity_margins(&self) -> Result<EquityMargins> {
        let req = self
            .build_auth_request(Method::GET, "/user/margins/equity")
            .await?;
        self.execute(req).await
    }

    /// Get commodity margins
    pub async fn get_commodity_margins(&self) -> Result<CommodityMargins> {
        let req = self
            .build_auth_request(Method::GET, "/user/margins/commodity")
            .await?;
        self.execute(req).await
    }

    // ==================== GTT API ====================

    /// List GTT orders
    pub async fn list_gtt(&self) -> Result<Vec<GTTTrigger>> {
        let req = self
            .build_auth_request(Method::GET, "/gtt/triggers")
            .await?;

        #[derive(Deserialize)]
        struct GTTResponse {
            data: Vec<GTTTrigger>,
        }

        let response: GTTResponse = self.execute(req).await?;
        Ok(response.data)
    }

    /// Get GTT order details
    pub async fn get_gtt(&self, trigger_id: u64) -> Result<GTTTrigger> {
        let path = format!("/gtt/triggers/{}", trigger_id);
        let req = self.build_auth_request(Method::GET, &path).await?;

        #[derive(Deserialize)]
        struct GTTResponse {
            data: GTTTrigger,
        }

        let response: GTTResponse = self.execute(req).await?;
        Ok(response.data)
    }

    /// Create GTT order
    pub async fn create_gtt(&self, req: &PlaceGTT) -> Result<GTTResponse> {
        let http_req = self
            .build_auth_request(Method::POST, "/gtt/triggers")
            .await?
            .json(req);
        self.execute(http_req).await
    }

    /// Modify GTT order
    pub async fn modify_gtt(&self, trigger_id: u64, req: &ModifyGTT) -> Result<GTTResponse> {
        let path = format!("/gtt/triggers/{}", trigger_id);
        let http_req = self.build_auth_request(Method::PUT, &path).await?.json(req);
        self.execute(http_req).await
    }

    /// Delete GTT order
    pub async fn delete_gtt(&self, trigger_id: u64) -> Result<()> {
        let path = format!("/gtt/triggers/{}", trigger_id);
        let http_req = self.build_auth_request(Method::DELETE, &path).await?;
        self.execute(http_req).await
    }
}

/// Redact sensitive information from error messages
fn redact_secrets(text: &str) -> String {
    let mut redacted = text.to_string();

    // Redact API secret (typically 16 character alphanumeric)
    if let Some(secret_pos) = redacted.find(&"api_secret\":".to_string()) {
        redacted.replace_range(secret_pos..secret_pos + 20, "api_secret\":\"***\"");
    }

    // Redact access token
    if let Some(token_pos) = redacted.find(&"access_token\":".to_string()) {
        redacted.replace_range(token_pos..token_pos + 30, "access_token\":\"***\"");
    }

    // Redact authorization header
    redacted = redacted.replace("token", "token ***");
    redacted
}

// SHA256 digest using sha2 crate
fn sha256_digest(input: &str) -> String {
    let mut hasher = sha2::Sha256::new();
    hasher.update(input.as_bytes());
    hex::encode(hasher.finalize())
}
