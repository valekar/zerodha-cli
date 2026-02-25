//! Interactive shell

use crate::error::CliError;

/// Execute interactive shell
pub async fn execute() -> Result<(), CliError> {
    println!("Kite Connect Interactive Shell");
    println!("Type 'exit' or Ctrl+D to quit");
    println!();

    // Basic interactive loop
    loop {
        print!("kite> ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let mut line = String::new();
        match std::io::stdin().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                if line == "exit" || line == "quit" {
                    println!("Goodbye!");
                    break;
                }
                println!("Executing: {}", line);
            }
        }
    }

    Ok(())
}
