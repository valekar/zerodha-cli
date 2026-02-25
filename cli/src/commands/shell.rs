//! Interactive shell command handlers

use anyhow::{Context, Result};
use rustyline::DefaultEditor;
use std::sync::Arc;
use tokio::sync::Mutex;
use zerodha_cli_core::{api::KiteConnectClient, config::Config};

use super::{
    auth, gtt, instruments, margins, orders, portfolio, quotes, status,
    AuthCommands, AuthSubcommands, GttCommands, GttSubcommands, InstrumentsCommands,
    InstrumentsSubcommands, MarginsCommands, MarginsSubcommands, OrdersCommands,
    OrdersSubcommands, PortfolioCommands, PortfolioSubcommands, QuotesCommands,
    QuotesSubcommands,
};

pub async fn run_shell(
    config: Arc<Mutex<Config>>,
    api_client: Arc<KiteConnectClient>,
    default_output_format: &str,
) -> Result<()> {
    println!("Zerodha CLI Shell v{}", env!("CARGO_PKG_VERSION"));
    println!("Type 'help' for commands, 'exit' to quit.");
    println!();

    let mut rl = DefaultEditor::new()?;
    let history_path = zerodha_cli_core::shell::shell_history_path()?;

    // Try to load history
    if history_path.exists() {
        if let Err(e) = rl.load_history(&history_path) {
            eprintln!("Warning: Failed to load history: {}", e);
        }
    }

    // Track commands executed in this session
    let mut commands_executed = 0;

    loop {
        let readline = rl.readline("kite> ");
        match readline {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                // Add to history
                let _ = rl.add_history_entry(line);

                if line == "exit" || line == "quit" {
                    break;
                }

                if line == "help" {
                    print_shell_help();
                    continue;
                }

                // Parse and execute command
                if let Err(e) = execute_shell_command(
                    line,
                    &mut rl,
                    Arc::clone(&config),
                    Arc::clone(&api_client),
                    default_output_format,
                )
                .await
                {
                    eprintln!("Error: {}", e);
                } else {
                    commands_executed += 1;
                }
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                println!("\nUse 'exit' or Ctrl+D to quit.");
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                break;
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    // Save history
    if let Some(parent) = history_path.parent() {
        std::fs::create_dir_all(parent).ok();
    }
    if let Err(e) = rl.save_history(&history_path) {
        eprintln!("Warning: Failed to save history: {}", e);
    }

    println!("Goodbye! (Executed {} command{})", commands_executed, if commands_executed == 1 { "" } else { "s" });
    Ok(())
}

async fn execute_shell_command(
    line: &str,
    _rl: &mut DefaultEditor,
    config: Arc<Mutex<Config>>,
    api_client: Arc<KiteConnectClient>,
    default_output_format: &str,
) -> Result<()> {
    let parts: Vec<&str> = shellwords::split(line)
        .with_context(|| format!("Failed to parse command: {}", line))?;

    if parts.is_empty() {
        return Ok(());
    }

    let cmd = parts[0].to_lowercase();
    let args = &parts[1..];

    match cmd.as_str() {
        "auth" => {
            if args.is_empty() {
                print_shell_help_auth();
                return Ok(());
            }
            let subcmd = args[0].to_lowercase();
            match subcmd.as_str() {
                "login" => {
                    let auth_cmd = AuthCommands {
                        command: AuthSubcommands::Login,
                    };
                    auth::run_auth(auth_cmd, &mut *config.lock().await, &api_client).await?;
                }
                "status" => {
                    let auth_cmd = AuthCommands {
                        command: AuthSubcommands::Status,
                    };
                    auth::run_auth(auth_cmd, &mut *config.lock().await, &api_client).await?;
                }
                "logout" => {
                    let auth_cmd = AuthCommands {
                        command: AuthSubcommands::Logout,
                    };
                    auth::run_auth(auth_cmd, &mut *config.lock().await, &api_client).await?;
                }
                "setup" => {
                    if args.len() < 4 {
                        eprintln!("Usage: auth setup --api-key <KEY> --api-secret <SECRET>");
                        return Ok(());
                    }
                    let api_key = args
                        .iter()
                        .position(|&a| a == "--api-key")
                        .and_then(|i| args.get(i + 1))
                        .ok_or_else(|| anyhow::anyhow!("Missing --api-key"))?;
                    let api_secret = args
                        .iter()
                        .position(|&a| a == "--api-secret")
                        .and_then(|i| args.get(i + 1))
                        .ok_or_else(|| anyhow::anyhow!("Missing --api-secret"))?;
                    let auth_cmd = AuthCommands {
                        command: AuthSubcommands::Setup {
                            api_key: api_key.to_string(),
                            api_secret: api_secret.to_string(),
                        },
                    };
                    auth::run_auth(auth_cmd, &mut *config.lock().await, &api_client).await?;
                }
                _ => {
                    eprintln!("Unknown auth subcommand: {}", subcmd);
                    print_shell_help_auth();
                }
            }
        }
        "instruments" => {
            if args.is_empty() {
                print_shell_help_instruments();
                return Ok(());
            }
            let subcmd = args[0].to_lowercase();
            match subcmd.as_str() {
                "list" => {
                    let exchange = args
                        .iter()
                        .position(|&a| a == "--exchange" || a == "-e")
                        .and_then(|i| args.get(i + 1))
                        .map(|s| s.to_string());
                    let refresh = args.contains(&"--refresh") || args.contains(&"-r");
                    let instruments_cmd = InstrumentsCommands {
                        command: InstrumentsSubcommands::List {
                            exchange,
                            refresh,
                        },
                    };
                    instruments::run_instruments(instruments_cmd, &api_client, default_output_format).await?;
                }
                "search" => {
                    if args.len() < 2 {
                        eprintln!("Usage: instruments search <query> [--exchange <EXCH>]");
                        return Ok(());
                    }
                    let query = args[1].to_string();
                    let exchange = args
                        .iter()
                        .position(|&a| a == "--exchange" || a == "-e")
                        .and_then(|i| args.get(i + 1))
                        .map(|s| s.to_string());
                    let instruments_cmd = InstrumentsCommands {
                        command: InstrumentsSubcommands::Search { query, exchange },
                    };
                    instruments::run_instruments(instruments_cmd, &api_client, default_output_format).await?;
                }
                "get" => {
                    if args.len() < 2 {
                        eprintln!("Usage: instruments get <SYMBOL>");
                        return Ok(());
                    }
                    let symbol = args[1].to_string();
                    let instruments_cmd = InstrumentsCommands {
                        command: InstrumentsSubcommands::Get { symbol },
                    };
                    instruments::run_instruments(instruments_cmd, &api_client, default_output_format).await?;
                }
                _ => {
                    eprintln!("Unknown instruments subcommand: {}", subcmd);
                    print_shell_help_instruments();
                }
            }
        }
        "quotes" => {
            if args.is_empty() {
                print_shell_help_quotes();
                return Ok(());
            }
            let subcmd = args[0].to_lowercase();
            match subcmd.as_str() {
                "get" => {
                    if args.len() < 2 {
                        eprintln!("Usage: quotes get <SYMBOL> [<SYMBOL> ...]");
                        return Ok(());
                    }
                    let symbols = args[1..].iter().map(|s| s.to_string()).collect();
                    let quotes_cmd = QuotesCommands {
                        command: QuotesSubcommands::Get { symbols },
                    };
                    quotes::run_quotes(quotes_cmd, &api_client, default_output_format).await?;
                }
                "ohlc" => {
                    if args.len() < 2 {
                        eprintln!("Usage: quotes ohlc <SYMBOL> [<SYMBOL> ...]");
                        return Ok(());
                    }
                    let symbols = args[1..].iter().map(|s| s.to_string()).collect();
                    let quotes_cmd = QuotesCommands {
                        command: QuotesSubcommands::Ohlc { symbols },
                    };
                    quotes::run_quotes(quotes_cmd, &api_client, default_output_format).await?;
                }
                "ltp" => {
                    if args.len() < 2 {
                        eprintln!("Usage: quotes ltp <SYMBOL> [<SYMBOL> ...]");
                        return Ok(());
                    }
                    let symbols = args[1..].iter().map(|s| s.to_string()).collect();
                    let quotes_cmd = QuotesCommands {
                        command: QuotesSubcommands::Ltp { symbols },
                    };
                    quotes::run_quotes(quotes_cmd, &api_client, default_output_format).await?;
                }
                _ => {
                    eprintln!("Unknown quotes subcommand: {}", subcmd);
                    print_shell_help_quotes();
                }
            }
        }
        "orders" => {
            if args.is_empty() {
                print_shell_help_orders();
                return Ok(());
            }
            let subcmd = args[0].to_lowercase();
            match subcmd.as_str() {
                "list" => {
                    let status = args
                        .iter()
                        .position(|&a| a == "--status" || a == "-s")
                        .and_then(|i| args.get(i + 1))
                        .map(|s| s.to_string());
                    let orders_cmd = OrdersCommands {
                        command: OrdersSubcommands::List { status },
                    };
                    orders::run_orders(orders_cmd, &*config.lock().await, &api_client, default_output_format).await?;
                }
                "get" => {
                    if args.len() < 2 {
                        eprintln!("Usage: orders get <ORDER_ID>");
                        return Ok(());
                    }
                    let order_id = args[1].to_string();
                    let orders_cmd = OrdersCommands {
                        command: OrdersSubcommands::Get { order_id },
                    };
                    orders::run_orders(orders_cmd, &*config.lock().await, &api_client, default_output_format).await?;
                }
                "cancel" => {
                    if args.len() < 2 {
                        eprintln!("Usage: orders cancel <ORDER_ID>");
                        return Ok(());
                    }
                    let order_id = args[1].to_string();
                    let variety = args
                        .iter()
                        .position(|&a| a == "--variety")
                        .and_then(|i| args.get(i + 1))
                        .unwrap_or(&"regular")
                        .to_string();
                    let orders_cmd = OrdersCommands {
                        command: OrdersSubcommands::Cancel {
                            order_id,
                            variety,
                        },
                    };
                    orders::run_orders(orders_cmd, &*config.lock().await, &api_client, default_output_format).await?;
                }
                "trades" => {
                    let order_id = args.get(1).map(|s| s.to_string());
                    let orders_cmd = OrdersCommands {
                        command: OrdersSubcommands::Trades { order_id },
                    };
                    orders::run_orders(orders_cmd, &*config.lock().await, &api_client, default_output_format).await?;
                }
                _ => {
                    eprintln!("Unknown orders subcommand: {}", subcmd);
                    eprintln!("Note: place, market, modify, cancel-all not implemented in shell yet");
                    print_shell_help_orders();
                }
            }
        }
        "portfolio" => {
            if args.is_empty() {
                print_shell_help_portfolio();
                return Ok(());
            }
            let subcmd = args[0].to_lowercase();
            match subcmd.as_str() {
                "holdings" => {
                    let portfolio_cmd = PortfolioCommands {
                        command: PortfolioSubcommands::Holdings,
                    };
                    portfolio::run_portfolio(portfolio_cmd, &api_client, default_output_format).await?;
                }
                "positions" => {
                    let net = args.contains(&"--net");
                    let day = args.contains(&"--day");
                    let portfolio_cmd = PortfolioCommands {
                        command: PortfolioSubcommands::Positions { net, day },
                    };
                    portfolio::run_portfolio(portfolio_cmd, &api_client, default_output_format).await?;
                }
                _ => {
                    eprintln!("Unknown portfolio subcommand: {}", subcmd);
                    eprintln!("Note: convert not implemented in shell yet");
                    print_shell_help_portfolio();
                }
            }
        }
        "margins" => {
            if args.is_empty() {
                print_shell_help_margins();
                return Ok(());
            }
            let subcmd = args[0].to_lowercase();
            match subcmd.as_str() {
                "list" => {
                    let margins_cmd = MarginsCommands {
                        command: MarginsSubcommands::List,
                    };
                    margins::run_margins(margins_cmd, &api_client, default_output_format).await?;
                }
                "equity" => {
                    let margins_cmd = MarginsCommands {
                        command: MarginsSubcommands::Equity,
                    };
                    margins::run_margins(margins_cmd, &api_client, default_output_format).await?;
                }
                "commodity" => {
                    let margins_cmd = MarginsCommands {
                        command: MarginsSubcommands::Commodity,
                    };
                    margins::run_margins(margins_cmd, &api_client, default_output_format).await?;
                }
                _ => {
                    eprintln!("Unknown margins subcommand: {}", subcmd);
                    print_shell_help_margins();
                }
            }
        }
        "gtt" => {
            if args.is_empty() {
                print_shell_help_gtt();
                return Ok(());
            }
            let subcmd = args[0].to_lowercase();
            match subcmd.as_str() {
                "list" => {
                    let gtt_cmd = GttCommands {
                        command: GttSubcommands::List,
                    };
                    gtt::run_gtt(gtt_cmd, &api_client, default_output_format).await?;
                }
                "get" => {
                    if args.len() < 2 {
                        eprintln!("Usage: gtt get <TRIGGER_ID>");
                        return Ok(());
                    }
                    let trigger_id = args[1].to_string();
                    let gtt_cmd = GttCommands {
                        command: GttSubcommands::Get { trigger_id },
                    };
                    gtt::run_gtt(gtt_cmd, &api_client, default_output_format).await?;
                }
                "delete" => {
                    if args.len() < 2 {
                        eprintln!("Usage: gtt delete <TRIGGER_ID>");
                        return Ok(());
                    }
                    let trigger_id = args[1].to_string();
                    let gtt_cmd = GttCommands {
                        command: GttSubcommands::Delete { trigger_id },
                    };
                    gtt::run_gtt(gtt_cmd, &api_client, default_output_format).await?;
                }
                _ => {
                    eprintln!("Unknown GTT subcommand: {}", subcmd);
                    eprintln!("Note: create, modify not implemented in shell yet");
                    print_shell_help_gtt();
                }
            }
        }
        "status" => {
            status::run_status(&*config.lock().await, &api_client).await?;
        }
        _ => {
            eprintln!("Unknown command: {}", cmd);
            print_shell_help();
        }
    }

    Ok(())
}

fn print_shell_help() {
    println!("Available commands:");
    println!("  auth [login|status|logout|setup]  Authentication");
    println!("  instruments [list|search|get]     Browse instruments");
    println!("  quotes [get|ohlc|ltp]             Market data");
    println!("  orders [list|get|cancel|trades]   Order management");
    println!("  portfolio [holdings|positions]   Portfolio");
    println!("  margins [list|equity|commodity]   Margins");
    println!("  gtt [list|get|delete]             GTT orders");
    println!("  status                            System status");
    println!("  help                              Show this help");
    println!("  exit, quit                        Quit shell");
    println!();
    println!("Note: Commands work without 'kite' prefix.");
    println!("      Use 'help <command>' for command-specific help.");
}

fn print_shell_help_auth() {
    println!("Authentication commands:");
    println!("  auth login                                    Start OAuth login flow");
    println!("  auth status                                   Show authentication status");
    println!("  auth logout                                   Logout and invalidate session");
    println!("  auth setup --api-key <KEY> --api-secret <SECRET>  Configure API credentials");
}

fn print_shell_help_instruments() {
    println!("Instruments commands:");
    println!("  instruments list [--exchange <EXCH>] [--refresh]  List instruments");
    println!("  instruments search <query> [--exchange <EXCH>]     Search by symbol/name");
    println!("  instruments get <SYMBOL>                         Get instrument details");
}

fn print_shell_help_quotes() {
    println!("Quotes commands:");
    println!("  quotes get <SYMBOL> [<SYMBOL> ...]   Get full quotes");
    println!("  quotes ohlc <SYMBOL> [<SYMBOL> ...]  Get OHLC data");
    println!("  quotes ltp <SYMBOL> [<SYMBOL> ...]   Get last traded price");
}

fn print_shell_help_orders() {
    println!("Orders commands:");
    println!("  orders list [--status <STATUS>]           List orders");
    println!("  orders get <ORDER_ID>                     Get order details");
    println!("  orders cancel <ORDER_ID>                  Cancel order");
    println!("  orders trades [ORDER_ID]                  View trade history");
}

fn print_shell_help_portfolio() {
    println!("Portfolio commands:");
    println!("  portfolio holdings               View holdings (long-term)");
    println!("  portfolio positions [--net|--day] View positions");
}

fn print_shell_help_margins() {
    println!("Margins commands:");
    println!("  margins list        View all margin segments");
    println!("  margins equity       View equity margins");
    println!("  margins commodity   View commodity margins");
}

fn print_shell_help_gtt() {
    println!("GTT (Good Till Triggered) commands:");
    println!("  gtt list              List all GTT orders");
    println!("  gtt get <TRIGGER_ID>  Get GTT details");
    println!("  gtt delete <TRIGGER_ID>  Delete GTT order");
}
