//! Authentication module

#[allow(clippy::module_inception)]
pub mod auth;
pub use auth::{login, logout, print_status, status, AuthStatus};
