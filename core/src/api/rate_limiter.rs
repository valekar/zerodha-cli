//! Rate limiter for Kite Connect API

use anyhow::Result;
use governor::{
    clock::DefaultClock,
    state::{InMemoryState, NotKeyed},
    Quota, RateLimiter as GovernorLimiter,
};
use nonzero_ext::nonzero;
use std::time::Duration;

/// Rate limiter enforcing 3 requests per second (Kite Connect limit)
pub struct RateLimiter {
    limiter: GovernorLimiter<NotKeyed, InMemoryState, DefaultClock>,
}

impl RateLimiter {
    /// Create a new rate limiter with 3 req/sec limit
    pub fn new() -> Self {
        // Kite Connect allows 3 requests per second
        let quota = Quota::per_second(nonzero!(3u32));
        let limiter = GovernorLimiter::direct(quota);

        Self { limiter }
    }

    /// Acquire a permit, waiting if necessary
    ///
    /// This will block until a permit is available or timeout is reached
    pub async fn acquire(&self) -> Result<()> {
        // Try to acquire immediately first
        if self.limiter.check().is_ok() {
            return Ok(());
        }

        // If rate limited, wait until we can acquire
        // Maximum wait time: 30 seconds (10 times the expected wait)
        let timeout = Duration::from_secs(30);
        let start = std::time::Instant::now();

        loop {
            if self.limiter.check().is_ok() {
                return Ok(());
            }

            if start.elapsed() > timeout {
                anyhow::bail!("Rate limit timeout: waited more than 30 seconds");
            }

            // Sleep for 100ms and try again
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter_allows_within_limit() {
        let limiter = RateLimiter::new();

        // Should allow 3 requests immediately
        for _ in 0..3 {
            assert!(limiter.acquire().await.is_ok());
        }
    }

    #[tokio::test]
    async fn test_rate_limiter_blocks_excess() {
        let limiter = RateLimiter::new();

        // Use up all 3 permits
        for _ in 0..3 {
            assert!(limiter.acquire().await.is_ok());
        }

        // 4th request should take some time (rate limited)
        let start = std::time::Instant::now();
        assert!(limiter.acquire().await.is_ok());
        let elapsed = start.elapsed();

        // Should have waited at least some time
        assert!(elapsed >= Duration::from_millis(100));
    }
}
