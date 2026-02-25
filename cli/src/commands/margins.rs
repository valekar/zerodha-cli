//! Margins command handlers

use anyhow::Result;
use serde_json;
use zerodha_cli_core::api::KiteConnectClient;

use super::MarginsCommands;

pub async fn run_margins(
    cmd: MarginsCommands,
    api_client: &KiteConnectClient,
    output_format: &str,
) -> Result<()> {
    match cmd.command {
        super::MarginsSubcommands::List => run_margins_list(output_format, api_client).await,
        super::MarginsSubcommands::Equity => run_margins_equity(output_format, api_client).await,
        super::MarginsSubcommands::Commodity => {
            run_margins_commodity(output_format, api_client).await
        }
    }
}

pub async fn run_margins_list(output_format: &str, api_client: &KiteConnectClient) -> Result<()> {
    let margins = api_client.get_margins().await?;

    if output_format == "json" {
        println!("{}", serde_json::to_string_pretty(&margins)?);
    } else {
        print_margins(&margins);
    }

    Ok(())
}

pub async fn run_margins_equity(output_format: &str, api_client: &KiteConnectClient) -> Result<()> {
    let equity = api_client.get_equity_margins().await?;

    if output_format == "json" {
        println!("{}", serde_json::to_string_pretty(&equity)?);
    } else {
        print_equity_margins(&equity.equity);
    }

    Ok(())
}

pub async fn run_margins_commodity(
    output_format: &str,
    api_client: &KiteConnectClient,
) -> Result<()> {
    let commodity = api_client.get_commodity_margins().await?;

    if output_format == "json" {
        println!("{}", serde_json::to_string_pretty(&commodity)?);
    } else {
        print_commodity_margins(&commodity.commodity);
    }

    Ok(())
}

fn print_margins(margins: &zerodha_cli_core::models::MarginResponse) {
    use comfy_table::{Cell, ContentArrangement, Table};

    let mut table = Table::new();
    table.set_header(vec!["Segment", "Net", "Available", "Used"]);

    let equity_avail = margins.equity.available.cash
        + margins.equity.available.collateral
        + margins.equity.available.live_balance;
    let equity_used = margins.equity.utilised.debits
        + margins.equity.utilised.exposure
        + margins.equity.utilised.options_premium;

    table.add_row(vec![
        Cell::new("Equity"),
        Cell::new(format!("₹{:.2}", margins.equity.net)),
        Cell::new(format!("₹{:.2}", equity_avail)),
        Cell::new(format!("₹{:.2}", equity_used)),
    ]);

    let commodity_avail = margins.commodity.available.cash
        + margins.commodity.available.collateral
        + margins.commodity.available.live_balance;
    let commodity_used = margins.commodity.utilised.debits
        + margins.commodity.utilised.exposure
        + margins.commodity.utilised.options_premium;

    table.add_row(vec![
        Cell::new("Commodity"),
        Cell::new(format!("₹{:.2}", margins.commodity.net)),
        Cell::new(format!("₹{:.2}", commodity_avail)),
        Cell::new(format!("₹{:.2}", commodity_used)),
    ]);

    table.set_content_arrangement(ContentArrangement::Dynamic);
    println!("{table}");
}

fn print_equity_margins(margin: &zerodha_cli_core::models::Margin) {
    use comfy_table::{Cell, ContentArrangement, Table};

    let available =
        margin.available.cash + margin.available.collateral + margin.available.live_balance;
    let used = margin.utilised.debits + margin.utilised.exposure + margin.utilised.options_premium;

    let mut table = Table::new();
    table.set_header(vec!["Field", "Amount"]);

    table.add_row(vec![
        Cell::new("Net"),
        Cell::new(format!("₹{:.2}", margin.net)),
    ]);
    table.add_row(vec![
        Cell::new("Available"),
        Cell::new(format!("₹{:.2}", available)),
    ]);
    table.add_row(vec![Cell::new("Used"), Cell::new(format!("₹{:.2}", used))]);

    table.add_row(vec![Cell::new(""), Cell::new("".to_string())]);
    table.add_row(vec![
        Cell::new("Cash"),
        Cell::new(format!("₹{:.2}", margin.available.cash)),
    ]);
    table.add_row(vec![
        Cell::new("Opening Balance"),
        Cell::new(format!("₹{:.2}", margin.available.opening_balance)),
    ]);
    table.add_row(vec![
        Cell::new("Live Balance"),
        Cell::new(format!("₹{:.2}", margin.available.live_balance)),
    ]);
    table.add_row(vec![
        Cell::new("Collateral"),
        Cell::new(format!("₹{:.2}", margin.available.collateral)),
    ]);

    table.add_row(vec![Cell::new(""), Cell::new("".to_string())]);
    table.add_row(vec![
        Cell::new("Debits"),
        Cell::new(format!("₹{:.2}", margin.utilised.debits)),
    ]);
    table.add_row(vec![
        Cell::new("Exposure"),
        Cell::new(format!("₹{:.2}", margin.utilised.exposure)),
    ]);
    table.add_row(vec![
        Cell::new("Options Premium"),
        Cell::new(format!("₹{:.2}", margin.utilised.options_premium)),
    ]);

    table.set_content_arrangement(ContentArrangement::Dynamic);
    println!("Equity Margins");
    println!("{table}");
}

fn print_commodity_margins(margin: &zerodha_cli_core::models::Margin) {
    use comfy_table::{Cell, ContentArrangement, Table};

    let available =
        margin.available.cash + margin.available.collateral + margin.available.live_balance;
    let used = margin.utilised.debits + margin.utilised.exposure + margin.utilised.options_premium;

    let mut table = Table::new();
    table.set_header(vec!["Field", "Amount"]);

    table.add_row(vec![
        Cell::new("Net"),
        Cell::new(format!("₹{:.2}", margin.net)),
    ]);
    table.add_row(vec![
        Cell::new("Available"),
        Cell::new(format!("₹{:.2}", available)),
    ]);
    table.add_row(vec![Cell::new("Used"), Cell::new(format!("₹{:.2}", used))]);

    table.add_row(vec![Cell::new(""), Cell::new("".to_string())]);
    table.add_row(vec![
        Cell::new("Cash"),
        Cell::new(format!("₹{:.2}", margin.available.cash)),
    ]);
    table.add_row(vec![
        Cell::new("Opening Balance"),
        Cell::new(format!("₹{:.2}", margin.available.opening_balance)),
    ]);
    table.add_row(vec![
        Cell::new("Live Balance"),
        Cell::new(format!("₹{:.2}", margin.available.live_balance)),
    ]);
    table.add_row(vec![
        Cell::new("Collateral"),
        Cell::new(format!("₹{:.2}", margin.available.collateral)),
    ]);

    table.add_row(vec![Cell::new(""), Cell::new("".to_string())]);
    table.add_row(vec![
        Cell::new("Debits"),
        Cell::new(format!("₹{:.2}", margin.utilised.debits)),
    ]);
    table.add_row(vec![
        Cell::new("Exposure"),
        Cell::new(format!("₹{:.2}", margin.utilised.exposure)),
    ]);
    table.add_row(vec![
        Cell::new("Options Premium"),
        Cell::new(format!("₹{:.2}", margin.utilised.options_premium)),
    ]);
    table.add_row(vec![
        Cell::new("Span"),
        Cell::new(format!("₹{:.2}", margin.utilised.span)),
    ]);

    table.set_content_arrangement(ContentArrangement::Dynamic);
    println!("Commodity Margins");
    println!("{table}");
}
