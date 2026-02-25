//! CLI module - Command definitions
//!
//! Note: The actual CLI implementation is in the `zerodha-cli` binary crate
//! (cli/src/commands/mod.rs). This module is reserved for future use if the
//! core library needs to provide CLI functionality directly.

use anyhow::Result;

/// Run the CLI
///
/// This function is currently not implemented in the core library.
/// The CLI implementation is in `cli/src/commands/mod.rs`.
#[allow(dead_code)]
pub async fn run() -> Result<()> {
    // The actual CLI runner is implemented in cli/src/commands/mod.rs
    // This module is kept for potential future use
    Err(anyhow::anyhow!("CLI runner is implemented in cli/src/commands/mod.rs"))
}
