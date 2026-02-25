//! Zerodha Kite Connect CLI
//!
//! A command-line interface for Zerodha's Kite Connect trading platform.
//!
//! # Example
//!
//! ```bash
//! # Authenticate
//! kite auth setup --api-key YOUR_KEY --api-secret YOUR_SECRET
//! kite auth login
//!
//! # Get quotes
//! kite quotes get NSE:INFY
//!
//! # Place order
//! kite orders market --symbol NSE:INFY --type BUY --quantity 10 --product MIS
//! ```

pub mod api;
pub mod config;
pub mod error;
pub mod http;
pub mod cache;
pub mod validation;

pub mod commands;
pub mod output;
pub mod utils;

pub use error::{CliError, Result};
