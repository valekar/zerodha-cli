//! Kite Connect API client implementation

use crate::error::{CliError, Result};
use crate::http::RateLimitedClient;
use super::types::*;

/// Kite Connect API client
pub struct KiteClient {
    http: RateLimitedClient,
    api_key: String,
    access_token: Option<String>,
    base_url: String,
}

impl KiteClient {
    /// Create a new Kite client
    pub fn new(api_key: String) -> Self {
        Self {
            http: RateLimitedClient::new(),
            api_key,
            access_token: None,
            base_url: "https://api.kite.trade".to_string(),
        }
    }

    /// Set access token
    pub fn with_token(mut self, token: String) -> Self {
        self.access_token = Some(token);
        self
    }

    /// Get access token
    pub fn get_access_token(&self) -> Result<&String> {
        self.access_token.as_ref().ok_or_else(|| {
            CliError::NotAuthenticated
        })
    }

    // ==================== AUTHENTICATION ====================

    /// Generate login URL
    pub fn login_url(&self) -> String {
        format!(
            "https://kite.zerodha.com/connect/login?v=3&api_key={}",
            self.api_key
        )
    }

    /// Generate a new session (exchange request token for access token)
    pub async fn generate_session(
        &self,
        request_token: &str,
        checksum: &str,
    ) -> Result<SessionResponse> {
        let url = format!("{}/session/token", self.base_url);
        let body = serde_json::json!({
            "api_key": self.api_key,
            "request_token": request_token,
            "checksum": checksum,
        });

        let response = self.http.post(&url, body).await?;

        // Check for API errors
        if response.status().is_client_error() || response.status().is_server_error() {
            let status = response.status().as_u16();
            let text = response.text().await?;
            return Err(CliError::ApiError {
                message: text,
                code: Some(status),
            });
        }

        let session: SessionResponse = response.json().await?;
        Ok(session)
    }

    /// Invalidate access token
    pub async fn invalidate_token(&self) -> Result<()> {
        let token = self.get_access_token()?;
        let url = format!("{}/session/token", self.base_url);
        let _response = self
            .http
            .post_with_auth(&url, &self.api_key, token, serde_json::json!({}))
            .await?;
        Ok(())
    }

    // ==================== INSTRUMENTS ====================

    /// Get instruments from exchange (returns CSV)
    pub async fn get_instruments(&self, exchange: Option<&str>) -> Result<Vec<Instrument>> {
        let token = self.get_access_token()?;
        let url = match exchange {
            Some(ex) => format!("{}/instruments/{}", self.base_url, ex),
            None => format!("{}/instruments", self.base_url),
        };

        let response = self.http.get_with_auth(&url, &self.api_key, token).await?;

        // Parse CSV response
        let text = response.text().await?;
        let mut reader = csv::Reader::from_reader(text.as_bytes());
        let mut instruments = Vec::new();

        for result in reader.deserialize() {
            let instrument: Instrument = result?;
            instruments.push(instrument);
        }

        Ok(instruments)
    }

    // ==================== QUOTES ====================

    /// Get quotes for instruments
    pub async fn get_quotes(&self, instruments: &[String]) -> Result<QuoteResponse> {
        let token = self.get_access_token()?;
        let url = format!(
            "{}/quote?i={}",
            self.base_url,
            instruments.join("&i=")
        );

        let response = self.http.get_with_auth(&url, &self.api_key, token).await?;

        // Check for API errors
        if response.status().is_client_error() || response.status().is_server_error() {
            let status = response.status().as_u16();
            let text = response.text().await?;
            return Err(CliError::ApiError {
                message: text,
                code: Some(status),
            });
        }

        let quotes: QuoteResponse = response.json().await?;
        Ok(quotes)
    }

    /// Get OHLC data
    pub async fn get_ohlc(&self, instruments: &[String]) -> Result<OhlcResponse> {
        let token = self.get_access_token()?;
        let url = format!(
            "{}/quote/ohlc?i={}",
            self.base_url,
            instruments.join("&i=")
        );

        let response = self.http.get_with_auth(&url, &self.api_key, token).await?;

        // Check for API errors
        if response.status().is_client_error() || response.status().is_server_error() {
            let status = response.status().as_u16();
            let text = response.text().await?;
            return Err(CliError::ApiError {
                message: text,
                code: Some(status),
            });
        }

        let ohlc: OhlcResponse = response.json().await?;
        Ok(ohlc)
    }

    /// Get LTP (last traded price)
    pub async fn get_ltp(&self, instruments: &[String]) -> Result<LtpResponse> {
        let token = self.get_access_token()?;
        let url = format!(
            "{}/quote/ltp?i={}",
            self.base_url,
            instruments.join("&i=")
        );

        let response = self.http.get_with_auth(&url, &self.api_key, token).await?;

        // Check for API errors
        if response.status().is_client_error() || response.status().is_server_error() {
            let status = response.status().as_u16();
            let text = response.text().await?;
            return Err(CliError::ApiError {
                message: text,
                code: Some(status),
            });
        }

        let ltp: LtpResponse = response.json().await?;
        Ok(ltp)
    }

    // ==================== ORDERS ====================

    /// Place an order
    pub async fn place_order(&self, order: PlaceOrder) -> Result<OrderResponse> {
        let token = self.get_access_token()?;
        let url = format!("{}/orders/regular", self.base_url);

        let response = self
            .http
            .post_with_auth(&url, &self.api_key, token, serde_json::to_value(&order)?)
            .await?;

        // Check for API errors
        if response.status().is_client_error() || response.status().is_server_error() {
            let status = response.status().as_u16();
            let text = response.text().await?;
            return Err(CliError::OrderRejected(text));
        }

        let order_response: OrderResponse = response.json().await?;
        Ok(order_response)
    }

    /// Get orders
    pub async fn get_orders(&self) -> Result<Vec<Order>> {
        let token = self.get_access_token()?;
        let url = format!("{}/orders", self.base_url);

        let response = self.http.get_with_auth(&url, &self.api_key, token).await?;

        let orders: Vec<Order> = response.json().await?;
        Ok(orders)
    }

    /// Get order by ID
    pub async fn get_order(&self, order_id: &str) -> Result<Order> {
        let token = self.get_access_token()?;
        let url = format!("{}/orders/{}", self.base_url, order_id);

        let response = self.http.get_with_auth(&url, &self.api_key, token).await?;

        let orders: Vec<Order> = response.json().await?;
        orders
            .into_iter()
            .next()
            .ok_or_else(|| CliError::Validation(format!("Order {} not found", order_id)))
    }

    /// Modify an order
    pub async fn modify_order(
        &self,
        order_id: &str,
        request: serde_json::Value,
    ) -> Result<OrderResponse> {
        let token = self.get_access_token()?;
        let url = format!("{}/orders/regular/{}", self.base_url, order_id);

        let response = self
            .http
            .put_with_auth(&url, &self.api_key, token, request)
            .await?;

        // Check for API errors
        if response.status().is_client_error() || response.status().is_server_error() {
            let status = response.status().as_u16();
            let text = response.text().await?;
            return Err(CliError::ApiError {
                message: text,
                code: Some(status),
            });
        }

        let order_response: OrderResponse = response.json().await?;
        Ok(order_response)
    }

    /// Cancel an order
    pub async fn cancel_order(&self, order_id: &str, variety: &str) -> Result<OrderResponse> {
        let token = self.get_access_token()?;
        let url = format!("{}/orders/{}/{}", self.base_url, variety, order_id);

        let response = self.http.delete_with_auth(&url, &self.api_key, token).await?;

        // Check for API errors
        if response.status().is_client_error() || response.status().is_server_error() {
            let status = response.status().as_u16();
            let text = response.text().await?;
            return Err(CliError::ApiError {
                message: text,
                code: Some(status),
            });
        }

        let order_response: OrderResponse = response.json().await?;
        Ok(order_response)
    }

    /// Get trade history
    pub async fn get_trades(&self, order_id: Option<&str>) -> Result<Vec<Trade>> {
        let token = self.get_access_token()?;
        let url = if let Some(oid) = order_id {
            format!("{}/orders/{}/trades", self.base_url, oid)
        } else {
            format!("{}/orders/trades", self.base_url)
        };

        let response = self.http.get_with_auth(&url, &self.api_key, token).await?;

        let trades: Vec<Trade> = response.json().await?;
        Ok(trades)
    }

    // ==================== PORTFOLIO ====================

    /// Get holdings
    pub async fn get_holdings(&self) -> Result<Vec<Holding>> {
        let token = self.get_access_token()?;
        let url = format!("{}/portfolio/holdings", self.base_url);

        let response = self.http.get_with_auth(&url, &self.api_key, token).await?;

        let holdings: Vec<Holding> = response.json().await?;
        Ok(holdings)
    }

    /// Get positions
    pub async fn get_positions(&self) -> Result<PositionsResponse> {
        let token = self.get_access_token()?;
        let url = format!("{}/portfolio/positions", self.base_url);

        let response = self.http.get_with_auth(&url, &self.api_key, token).await?;

        let positions: PositionsResponse = response.json().await?;
        Ok(positions)
    }

    /// Convert position
    pub async fn convert_position(&self, request: ConvertPosition) -> Result<()> {
        let token = self.get_access_token()?;
        let url = format!("{}/portfolio/positions", self.base_url);

        let response = self
            .http
            .put_with_auth(&url, &self.api_key, token, serde_json::to_value(&request)?)
            .await?;

        // Check for API errors
        if response.status().is_client_error() || response.status().is_server_error() {
            let status = response.status().as_u16();
            let text = response.text().await?;
            return Err(CliError::ApiError {
                message: text,
                code: Some(status),
            });
        }

        Ok(())
    }

    // ==================== MARGINS ====================

    /// Get all margins
    pub async fn get_margins(&self) -> Result<MarginResponse> {
        let token = self.get_access_token()?;
        let url = format!("{}/margins", self.base_url);

        let response = self.http.get_with_auth(&url, &self.api_key, token).await?;

        let margins: MarginResponse = response.json().await?;
        Ok(margins)
    }

    /// Get equity margins
    pub async fn get_equity_margins(&self) -> Result<Margin> {
        let token = self.get_access_token()?;
        let url = format!("{}/margins/equity", self.base_url);

        let response = self.http.get_with_auth(&url, &self.api_key, token).await?;

        let response_data: MarginResponse = response.json().await?;
        response_data
            .data
            .get("equity")
            .cloned()
            .ok_or_else(|| CliError::ApiError {
                message: "Equity margins not found".to_string(),
                code: None,
            })
    }

    /// Get commodity margins
    pub async fn get_commodity_margins(&self) -> Result<Margin> {
        let token = self.get_access_token()?;
        let url = format!("{}/margins/commodity", self.base_url);

        let response = self.http.get_with_auth(&url, &self.api_key, token).await?;

        let response_data: MarginResponse = response.json().await?;
        response_data
            .data
            .get("commodity")
            .cloned()
            .ok_or_else(|| CliError::ApiError {
                message: "Commodity margins not found".to_string(),
                code: None,
            })
    }

    // ==================== GTT ====================

    /// Get all GTT orders
    pub async fn get_gtt_orders(&self) -> Result<Vec<GttTrigger>> {
        let token = self.get_access_token()?;
        let url = format!("{}/gtt/triggers", self.base_url);

        let response = self.http.get_with_auth(&url, &self.api_key, token).await?;

        let gtt_data: serde_json::Value = response.json().await?;
        let triggers = gtt_data["data"]
            .as_array()
            .ok_or_else(|| CliError::ApiError {
                message: "Invalid GTT response format".to_string(),
                code: None,
            })?;

        serde_json::from_value(triggers.clone()).map_err(|e| CliError::Json(e))
    }

    /// Get GTT order by ID
    pub async fn get_gtt(&self, trigger_id: u64) -> Result<GttTrigger> {
        let token = self.get_access_token()?;
        let url = format!("{}/gtt/triggers/{}", self.base_url, trigger_id);

        let response = self.http.get_with_auth(&url, &self.api_key, token).await?;

        let gtt_data: serde_json::Value = response.json().await?;
        let trigger = gtt_data["data"]
            .as_object()
            .ok_or_else(|| CliError::ApiError {
                message: "Invalid GTT response format".to_string(),
                code: None,
            })?;

        serde_json::from_value(serde_json::to_value(trigger)?).map_err(|e| CliError::Json(e))
    }

    /// Place GTT order
    pub async fn place_gtt(&self, request: PlaceGtt) -> Result<GttResponse> {
        let token = self.get_access_token()?;
        let url = format!("{}/gtt/triggers", self.base_url);

        let response = self
            .http
            .post_with_auth(&url, &self.api_key, token, serde_json::to_value(&request)?)
            .await?;

        // Check for API errors
        if response.status().is_client_error() || response.status().is_server_error() {
            let status = response.status().as_u16();
            let text = response.text().await?;
            return Err(CliError::ApiError {
                message: text,
                code: Some(status),
            });
        }

        let gtt_response: GttResponse = response.json().await?;
        Ok(gtt_response)
    }

    /// Modify GTT order
    pub async fn modify_gtt(
        &self,
        trigger_id: u64,
        request: ModifyGtt,
    ) -> Result<GttResponse> {
        let token = self.get_access_token()?;
        let url = format!("{}/gtt/triggers/{}", self.base_url, trigger_id);

        let response = self
            .http
            .put_with_auth(&url, &self.api_key, token, serde_json::to_value(&request)?)
            .await?;

        // Check for API errors
        if response.status().is_client_error() || response.status().is_server_error() {
            let status = response.status().as_u16();
            let text = response.text().await?;
            return Err(CliError::ApiError {
                message: text,
                code: Some(status),
            });
        }

        let gtt_response: GttResponse = response.json().await?;
        Ok(gtt_response)
    }

    /// Delete GTT order
    pub async fn delete_gtt(&self, trigger_id: u64) -> Result<GttResponse> {
        let token = self.get_access_token()?;
        let url = format!("{}/gtt/triggers/{}", self.base_url, trigger_id);

        let response = self.http.delete_with_auth(&url, &self.api_key, token).await?;

        // Check for API errors
        if response.status().is_client_error() || response.status().is_server_error() {
            let status = response.status().as_u16();
            let text = response.text().await?;
            return Err(CliError::ApiError {
                message: text,
                code: Some(status),
            });
        }

        let gtt_response: GttResponse = response.json().await?;
        Ok(gtt_response)
    }
}
