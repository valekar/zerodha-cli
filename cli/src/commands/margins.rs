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
        match &equity.equity {
            Some(margin) => print_equity_margins(margin),
            None => println!("No equity margin data available"),
        }
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
        match &commodity.commodity {
            Some(margin) => print_commodity_margins(margin),
            None => println!("No commodity margin data available"),
        }
    }

    Ok(())
}

fn print_margins(margins: &zerodha_cli_core::models::MarginResponse) {
    use comfy_table::{Cell, ContentArrangement, Table};

    let mut table = Table::new();
    table.set_header(vec!["Segment", "Net", "Available", "Used"]);

    if let Some(ref equity) = margins.equity {
        let equity_avail = equity.available.cash
            + equity.available.collateral
            + equity.available.live_balance;
        let equity_used = equity.utilised.debits
            + equity.utilised.exposure
            + equity.utilised.options_premium;

        table.add_row(vec![
            Cell::new("Equity"),
            Cell::new(format!("₹{:.2}", equity.net)),
            Cell::new(format!("₹{:.2}", equity_avail)),
            Cell::new(format!("₹{:.2}", equity_used)),
        ]);
    }

    if let Some(ref commodity) = margins.commodity {
        let commodity_avail = commodity.available.cash
            + commodity.available.collateral
            + commodity.available.live_balance;
        let commodity_used = commodity.utilised.debits
            + commodity.utilised.exposure
            + commodity.utilised.options_premium;

        table.add_row(vec![
            Cell::new("Commodity"),
            Cell::new(format!("₹{:.2}", commodity.net)),
            Cell::new(format!("₹{:.2}", commodity_avail)),
            Cell::new(format!("₹{:.2}", commodity_used)),
        ]);
    }

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
