//! Instruments command handlers

use anyhow::Result;
use serde_json;
use zerodha_cli_core::{
    api::KiteConnectClient, cache::InstrumentCache, models::Instrument, output::OutputFormatter,
};

use super::InstrumentsCommands;

pub async fn run_instruments(
    cmd: InstrumentsCommands,
    api_client: &KiteConnectClient,
    output_format: &str,
) -> Result<()> {
    match cmd.command {
        super::InstrumentsSubcommands::List { exchange, refresh } => {
            run_instruments_list(exchange, refresh, output_format, api_client).await?
        }
        super::InstrumentsSubcommands::Search { query, exchange } => {
            run_instruments_search(query, exchange, output_format, api_client).await?
        }
        super::InstrumentsSubcommands::Get { symbol } => {
            run_instruments_get(symbol, output_format, api_client).await?
        }
    }
    Ok(())
}

pub async fn run_instruments_list(
    exchange: Option<String>,
    refresh: bool,
    output_format: &str,
    api_client: &KiteConnectClient,
) -> Result<()> {
    let exchange = exchange.unwrap_or_else(|| "NSE".to_string());

    // Check if cache is valid
    let instruments = if !refresh && InstrumentCache::is_valid(&exchange)? {
        println!("Loading instruments from cache...");
        InstrumentCache::load(&exchange)?
    } else {
        println!("Downloading instruments from exchange...");
        let instruments = api_client.list_instruments(Some(exchange.as_str())).await?;
        InstrumentCache::save(&exchange, &instruments)?;
        println!("✓ Downloaded {} instruments", instruments.len());
        instruments
    };

    // Display
    if output_format == "json" {
        instruments.print_json()?;
    } else {
        print_instruments_table(&instruments);
    }

    Ok(())
}

pub async fn run_instruments_search(
    query: String,
    exchange_filter: Option<String>,
    output_format: &str,
    api_client: &KiteConnectClient,
) -> Result<()> {
    let query_lower = query.to_lowercase();

    // Get instruments from cache or API
    let exchange = exchange_filter.as_deref().unwrap_or("NSE");
    let instruments = if InstrumentCache::is_valid(exchange)? {
        InstrumentCache::load(exchange)?
    } else {
        println!("Downloading instruments from exchange...");
        let instruments = api_client.list_instruments(Some(exchange)).await?;
        InstrumentCache::save(exchange, &instruments)?;
        instruments
    };

    // Filter by query
    let filtered: Vec<Instrument> = instruments
        .into_iter()
        .filter(|inst| {
            let matches_symbol = inst.tradingsymbol.to_lowercase().contains(&query_lower);
            let matches_name = inst.name.to_lowercase().contains(&query_lower);
            let matches_exchange = exchange_filter.is_none()
                || format!("{:?}", inst.exchange).to_lowercase() == exchange;
            (matches_symbol || matches_name) && matches_exchange
        })
        .collect();

    if filtered.is_empty() {
        println!("No instruments found matching '{}'", query);
        return Ok(());
    }

    println!("Found {} instruments matching '{}':", filtered.len(), query);

    if output_format == "json" {
        filtered.print_json()?;
    } else {
        print_instruments_table(&filtered);
    }

    Ok(())
}

pub async fn run_instruments_get(
    symbol: String,
    output_format: &str,
    api_client: &KiteConnectClient,
) -> Result<()> {
    let parts: Vec<&str> = symbol.split(':').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid symbol format. Expected: EXCHANGE:SYMBOL (e.g., NSE:INFY)");
    }

    let exchange = parts[0];
    let tradingsymbol = parts[1];

    let instrument = api_client.get_instrument(exchange, tradingsymbol).await?;

    if output_format == "json" {
        println!("{}", serde_json::to_string_pretty(&instrument)?);
    } else {
        print_instrument_details(&instrument);
    }

    Ok(())
}

fn print_instruments_table(instruments: &[Instrument]) {
    use comfy_table::{Cell, ContentArrangement, Table};

    let mut table = Table::new();
    table.set_header(vec![
        "Symbol",
        "Name",
        "Exchange",
        "Type",
        "Lot Size",
        "Tick Size",
    ]);

    for inst in instruments.iter().take(50) {
        // Limit to first 50 for display
        table.add_row(vec![
            Cell::new(&inst.tradingsymbol),
            Cell::new(&inst.name),
            Cell::new(format!("{:?}", inst.exchange)),
            Cell::new(format!("{:?}", inst.instrument_type)),
            Cell::new(inst.lot_size.to_string()),
            Cell::new(inst.tick_size.to_string()),
        ]);
    }

    if instruments.len() > 50 {
        println!("Showing 50 of {} instruments", instruments.len());
    }

    table.set_content_arrangement(ContentArrangement::Dynamic);
    println!("{table}");
}

fn print_instrument_details(inst: &Instrument) {
    println!("Instrument: {}", inst.tradingsymbol);
    println!();
    println!("Name: {}", inst.name);
    println!("Exchange: {:?}", inst.exchange);
    println!("Segment: {:?}", inst.segment);
    println!("Type: {:?}", inst.instrument_type);
    println!("Lot Size: {}", inst.lot_size);
    println!("Tick Size: {}", inst.tick_size);

    if let Some(expiry) = &inst.expiry {
        println!("Expiry: {}", expiry);
    }

    if let Some(strike) = inst.strike {
        println!("Strike Price: {}", strike);
    }

    if let Some(last_price) = inst.last_price {
        println!("Last Price: ₹{:.2}", last_price);
    }
}
