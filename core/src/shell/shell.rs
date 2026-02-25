//! Interactive shell implementation

use crate::api::KiteConnectClient;
use crate::config::Config;
use anyhow::Result;
use rustyline::history::DefaultHistory;
use rustyline::{CompletionType, Config as RLConfig, Editor};

/// Run interactive shell
pub async fn run(_config: &Config, _api_client: &KiteConnectClient) -> Result<()> {
    let rl_config = RLConfig::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .build();

    let _rl: Editor<(), DefaultHistory> = Editor::with_config(rl_config)?;

    println!("Zerodha CLI Shell v1.0.0");
    println!("Type 'help' for commands, 'exit' to quit.\n");

    // TODO: Implement REPL
    // - Load history
    // - Parse and execute commands
    // - Save history on exit

    println!("Shell not yet implemented - use CLI commands instead.");
    Ok(())
}
