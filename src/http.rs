//! HTTP client wrapper with rate limiting

use governor::{Quota, RateLimiter};
use nonzero_ext::nonzero;
use reqwest::Client;
use std::time::Duration;

/// Rate-limited HTTP client
pub struct RateLimitedClient {
    client: Client,
    limiter: RateLimiter<...>,
    timeout: Duration,
    max_retries: u32,
}

impl RateLimitedClient {
    /// Create a new rate-limited HTTP client
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
            limiter: RateLimiter::direct(Quota::per_second(nonzero!(3u32))), // Kite Connect limit: 3 req/sec
            timeout: Duration::from_secs(30),
            max_retries: 3,
        }
    }

    /// Perform a GET request with rate limiting and retry logic
    pub async fn get(&self, url: &str) -> Result<reqwest::Response, reqwest::Error> {
        self.limiter.until_ready().await;
        let mut retries = 0;
        loop {
            match self.client.get(url).send().await {
                Ok(resp) => return Ok(resp),
                Err(e) if retries < self.max_retries => {
                    retries += 1;
                    tokio::time::sleep(Duration::from_millis(100 * retries as u64)).await;
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// Perform a GET request with authorization header
    pub async fn get_with_auth(
        &self,
        url: &str,
        api_key: &str,
        access_token: &str,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.limiter.until_ready().await;
        let mut retries = 0;
        loop {
            match self.client
                .get(url)
                .header("Authorization", format!("token {}:{}", api_key, access_token))
                .send()
                .await
            {
                Ok(resp) => return Ok(resp),
                Err(e) if retries < self.max_retries => {
                    retries += 1;
                    tokio::time::sleep(Duration::from_millis(100 * retries as u64)).await;
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// Perform a POST request with JSON body
    pub async fn post(
        &self,
        url: &str,
        body: serde_json::Value,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.limiter.until_ready().await;
        let mut retries = 0;
        loop {
            match self.client.post(url).json(&body).send().await {
                Ok(resp) => return Ok(resp),
                Err(e) if retries < self.max_retries => {
                    retries += 1;
                    tokio::time::sleep(Duration::from_millis(100 * retries as u64)).await;
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// Perform a POST request with authorization header
    pub async fn post_with_auth(
        &self,
        url: &str,
        api_key: &str,
        access_token: &str,
        body: serde_json::Value,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.limiter.until_ready().await;
        let mut retries = 0;
        loop {
            match self.client
                .post(url)
                .header("Authorization", format!("token {}:{}", api_key, access_token))
                .json(&body)
                .send()
                .await
            {
                Ok(resp) => return Ok(resp),
                Err(e) if retries < self.max_retries => {
                    retries += 1;
                    tokio::time::sleep(Duration::from_millis(100 * retries as u64)).await;
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// Perform a DELETE request with authorization header
    pub async fn delete_with_auth(
        &self,
        url: &str,
        api_key: &str,
        access_token: &str,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.limiter.until_ready().await;
        let mut retries = 0;
        loop {
            match self.client
                .delete(url)
                .header("Authorization", format!("token {}:{}", api_key, access_token))
                .send()
                .await
            {
                Ok(resp) => return Ok(resp),
                Err(e) if retries < self.max_retries => {
                    retries += 1;
                    tokio::time::sleep(Duration::from_millis(100 * retries as u64)).await;
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// Perform a PUT request with authorization header
    pub async fn put_with_auth(
        &self,
        url: &str,
        api_key: &str,
        access_token: &str,
        body: serde_json::Value,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.limiter.until_ready().await;
        let mut retries = 0;
        loop {
            match self.client
                .put(url)
                .header("Authorization", format!("token {}:{}", api_key, access_token))
                .json(&body)
                .send()
                .await
            {
                Ok(resp) => return Ok(resp),
                Err(e) if retries < self.max_retries => {
                    retries += 1;
                    tokio::time::sleep(Duration::from_millis(100 * retries as u64)).await;
                }
                Err(e) => return Err(e),
            }
        }
    }
}

impl Default for RateLimitedClient {
    fn default() -> Self {
        Self::new()
    }
}
