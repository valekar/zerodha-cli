//! Quotes command handlers

use anyhow::Result;
use serde_json;
use zerodha_cli_core::api::KiteConnectClient;

use super::QuotesCommands;

pub async fn run_quotes(
    cmd: QuotesCommands,
    api_client: &KiteConnectClient,
    output_format: &str,
) -> Result<()> {
    match cmd.command {
        super::QuotesSubcommands::Get { symbols } => {
            run_quotes_get(symbols, output_format, api_client).await?
        }
        super::QuotesSubcommands::Ohlc { symbols } => {
            run_quotes_ohlc(symbols, output_format, api_client).await?
        }
        super::QuotesSubcommands::Ltp { symbols } => {
            run_quotes_ltp(symbols, output_format, api_client).await?
        }
    }
    Ok(())
}

pub async fn run_quotes_get(
    symbols: Vec<String>,
    output_format: &str,
    api_client: &KiteConnectClient,
) -> Result<()> {
    if symbols.is_empty() {
        anyhow::bail!("No symbols provided. Use: kite quotes get SYMBOL1 SYMBOL2 ...");
    }

    let symbols_refs: Vec<&str> = symbols.iter().map(|s| s.as_str()).collect();
    let quotes_response = api_client.get_quotes(&symbols_refs).await?;

    // Display
    if output_format == "json" {
        // QuoteResponse doesn't implement Serialize, so serialize each quote individually
        for (symbol, quote) in quotes_response.data {
            let json = serde_json::json!({
                symbol: quote
            });
            println!("{}", serde_json::to_string_pretty(&json)?);
        }
    } else {
        for (symbol, quote) in quotes_response.data {
            print_quote(&symbol, &quote);
        }
    }

    Ok(())
}

pub async fn run_quotes_ohlc(
    symbols: Vec<String>,
    output_format: &str,
    api_client: &KiteConnectClient,
) -> Result<()> {
    if symbols.is_empty() {
        anyhow::bail!("No symbols provided. Use: kite quotes ohlc SYMBOL1 SYMBOL2 ...");
    }

    let symbols_refs: Vec<&str> = symbols.iter().map(|s| s.as_str()).collect();
    let ohlc_response = api_client.get_ohlc(&symbols_refs).await?;

    // Display
    if output_format == "json" {
        println!("{}", serde_json::to_string_pretty(&ohlc_response)?);
    } else {
        for (symbol, ohlc) in ohlc_response.data {
            print_ohlc(&symbol, &ohlc);
        }
    }

    Ok(())
}

pub async fn run_quotes_ltp(
    symbols: Vec<String>,
    output_format: &str,
    api_client: &KiteConnectClient,
) -> Result<()> {
    if symbols.is_empty() {
        anyhow::bail!("No symbols provided. Use: kite quotes ltp SYMBOL1 SYMBOL2 ...");
    }

    let symbols_refs: Vec<&str> = symbols.iter().map(|s| s.as_str()).collect();
    let ltp_response = api_client.get_ltp(&symbols_refs).await?;

    // Display
    if output_format == "json" {
        println!("{}", serde_json::to_string_pretty(&ltp_response)?);
    } else {
        use comfy_table::{Cell, ContentArrangement, Table};

        let mut table = Table::new();
        table.set_header(vec!["Symbol", "Last Price"]);

        for (symbol, ltp_data) in ltp_response.data {
            table.add_row(vec![
                Cell::new(symbol),
                Cell::new(format!("₹{:.2}", ltp_data.last_price)),
            ]);
        }

        table.set_content_arrangement(ContentArrangement::Dynamic);
        println!("{table}");
    }

    Ok(())
}

fn print_quote(symbol: &str, quote: &zerodha_cli_core::models::Quote) {
    println!("Quote: {}", symbol);
    println!();
    println!("Last Price: ₹{:.2}", quote.last_price);

    let ohlc = &quote.ohlc;
    println!(
        "OHLC: O: ₹{:.2} | H: ₹{:.2} | L: ₹{:.2} | C: ₹{:.2}",
        ohlc.open, ohlc.high, ohlc.low, ohlc.close
    );

    if let Some(oi) = quote.oi {
        println!("Open Interest: {}", oi);
    }

    // Depth
    if !quote.depth.buy.is_empty() {
        println!();
        println!("Buy Orders:");
        for (i, entry) in quote.depth.buy.iter().enumerate().take(5) {
            println!(
                "  {}: {} @ ₹{:.2} ({} orders)",
                i + 1,
                entry.quantity,
                entry.price,
                entry.orders
            );
        }
    }

    if !quote.depth.sell.is_empty() {
        println!();
        println!("Sell Orders:");
        for (i, entry) in quote.depth.sell.iter().enumerate().take(5) {
            println!(
                "  {}: {} @ ₹{:.2} ({} orders)",
                i + 1,
                entry.quantity,
                entry.price,
                entry.orders
            );
        }
    }

    println!();
}

fn print_ohlc(symbol: &str, ohlc: &zerodha_cli_core::models::OHLCData) {
    use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};

    let change = ohlc.close - ohlc.open;
    let change_pct = (change / ohlc.open) * 100.0;

    let change_cell = if change >= 0.0 {
        Cell::new(format!("+₹{:.2} ({:.2}%)", change, change_pct))
            .fg(Color::Green)
            .add_attribute(Attribute::Bold)
    } else {
        Cell::new(format!("₹{:.2} ({:.2}%)", change, change_pct))
            .fg(Color::Red)
            .add_attribute(Attribute::Bold)
    };

    let mut table = Table::new();
    table.set_header(vec!["Symbol", "Open", "High", "Low", "Close", "Change"]);

    table.add_row(vec![
        Cell::new(symbol),
        Cell::new(format!("₹{:.2}", ohlc.open)),
        Cell::new(format!("₹{:.2}", ohlc.high)),
        Cell::new(format!("₹{:.2}", ohlc.low)),
        Cell::new(format!("₹{:.2}", ohlc.close)),
        change_cell,
    ]);

    table.set_content_arrangement(ContentArrangement::Dynamic);
    println!("{table}");
}
