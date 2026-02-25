//! Portfolio command handlers

use anyhow::Result;
use zerodha_cli_core::api::KiteConnectClient;

use super::PortfolioCommands;

pub async fn run_portfolio(
    cmd: PortfolioCommands,
    api_client: &KiteConnectClient,
    output_format: &str,
) -> Result<()> {
    match cmd.command {
        super::PortfolioSubcommands::Holdings => {
            run_portfolio_holdings(output_format, api_client).await
        }
        super::PortfolioSubcommands::Positions { net, day } => {
            run_portfolio_positions(net, day, output_format, api_client).await
        }
        super::PortfolioSubcommands::Convert {
            symbol,
            order_type,
            quantity,
            from,
            to,
        } => run_portfolio_convert(symbol, order_type, quantity, from, to, api_client).await,
    }
}

pub async fn run_portfolio_holdings(
    output_format: &str,
    api_client: &KiteConnectClient,
) -> Result<()> {
    let holdings = api_client.get_holdings().await?;

    if holdings.is_empty() {
        println!("No holdings found.");
        return Ok(());
    }

    if output_format == "json" {
        println!("{}", serde_json::to_string_pretty(&holdings)?);
    } else {
        print_holdings_table(&holdings);
    }

    Ok(())
}

pub async fn run_portfolio_positions(
    _net: bool,
    _day: bool,
    output_format: &str,
    api_client: &KiteConnectClient,
) -> Result<()> {
    let response = api_client.get_positions().await?;
    let positions = response.net;

    if positions.is_empty() {
        println!("No positions found.");
        return Ok(());
    }

    if output_format == "json" {
        println!("{}", serde_json::to_string_pretty(&positions)?);
    } else {
        print_positions_table(&positions);
    }

    Ok(())
}

pub async fn run_portfolio_convert(
    _symbol: String,
    _order_type: String,
    _quantity: i32,
    _from: String,
    _to: String,
    _api_client: &KiteConnectClient,
) -> Result<()> {
    println!("Position conversion not yet implemented in CLI.");
    Ok(())
}

fn print_holdings_table(holdings: &[zerodha_cli_core::models::Holding]) {
    use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};

    let mut table = Table::new();
    table.set_header(vec!["Symbol", "Qty", "Avg Price", "LTP", "P&L", "Day Chg%"]);

    let mut total_pnl = 0.0;

    for holding in holdings {
        total_pnl += holding.pnl;

        let pnl_cell = if holding.pnl >= 0.0 {
            Cell::new(format!("₹{:.2}", holding.pnl))
                .fg(Color::Green)
                .add_attribute(Attribute::Bold)
        } else {
            Cell::new(format!("₹{:.2}", holding.pnl))
                .fg(Color::Red)
                .add_attribute(Attribute::Bold)
        };

        let chg_cell = if holding.day_change_percentage >= 0.0 {
            Cell::new(format!("{:.2}%", holding.day_change_percentage)).fg(Color::Green)
        } else {
            Cell::new(format!("{:.2}%", holding.day_change_percentage)).fg(Color::Red)
        };

        table.add_row(vec![
            Cell::new(&holding.tradingsymbol),
            Cell::new(holding.quantity.to_string()),
            Cell::new(format!("₹{:.2}", holding.average_price)),
            Cell::new(format!("₹{:.2}", holding.last_price)),
            pnl_cell,
            chg_cell,
        ]);
    }

    table.set_content_arrangement(ContentArrangement::Dynamic);
    println!("{table}");
    println!();
    println!("Total P&L: ₹{:.2}", total_pnl);
}

fn print_positions_table(positions: &[zerodha_cli_core::models::Position]) {
    use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};

    let mut table = Table::new();
    table.set_header(vec!["Symbol", "Qty", "Avg Price", "LTP", "P&L", "M2M"]);

    let mut total_pnl = 0.0;
    let mut total_m2m = 0.0;

    for position in positions {
        total_pnl += position.pnl;
        total_m2m += position.m2m;

        let pnl_cell = if position.pnl >= 0.0 {
            Cell::new(format!("₹{:.2}", position.pnl))
                .fg(Color::Green)
                .add_attribute(Attribute::Bold)
        } else {
            Cell::new(format!("₹{:.2}", position.pnl))
                .fg(Color::Red)
                .add_attribute(Attribute::Bold)
        };

        let m2m_cell = if position.m2m >= 0.0 {
            Cell::new(format!("₹{:.2}", position.m2m)).fg(Color::Green)
        } else {
            Cell::new(format!("₹{:.2}", position.m2m)).fg(Color::Red)
        };

        table.add_row(vec![
            Cell::new(&position.tradingsymbol),
            Cell::new(position.quantity.to_string()),
            Cell::new(format!("₹{:.2}", position.average_price)),
            Cell::new(format!("₹{:.2}", position.last_price)),
            pnl_cell,
            m2m_cell,
        ]);
    }

    table.set_content_arrangement(ContentArrangement::Dynamic);
    println!("{table}");
    println!();
    println!(
        "Total P&L: ₹{:.2} | Total M2M: ₹{:.2}",
        total_pnl, total_m2m
    );
}
