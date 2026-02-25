//! Output formatting implementation

use crate::models::{Holding, Order, OrderStatus};
use anyhow::Result;
use comfy_table::{Table, Cell, Color, ContentArrangement};

/// Trait for formatting output
pub trait OutputFormatter {
    fn print(&self) -> anyhow::Result<()>;
    fn print_json(&self) -> anyhow::Result<()>;
}

/// Format holdings as table
impl OutputFormatter for Vec<Holding> {
    fn print(&self) -> Result<()> {
        let mut table = Table::new();
        table.set_header(vec!["Symbol", "Qty", "Avg Price", "LTP", "P&L", "Day Chg%"]);

        for holding in self {
            let pnl_cell = if holding.pnl >= 0.0 {
                Cell::new(format!("₹{:.2}", holding.pnl))
                    .fg(Color::Green)
            } else {
                Cell::new(format!("₹{:.2}", holding.pnl))
                    .fg(Color::Red)
            };

            let chg_cell = if holding.day_change_percentage >= 0.0 {
                Cell::new(format!("{:.2}%", holding.day_change_percentage))
                    .fg(Color::Green)
            } else {
                Cell::new(format!("{:.2}%", holding.day_change_percentage))
                    .fg(Color::Red)
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
        Ok(())
    }

    fn print_json(&self) -> Result<()> {
        println!("{}", serde_json::to_string_pretty(self)?);
        Ok(())
    }
}

/// Format orders as table
impl OutputFormatter for Vec<Order> {
    fn print(&self) -> Result<()> {
        let mut table = Table::new();
        table.set_header(vec!["Order ID", "Symbol", "Type", "Qty", "Price", "Status", "Time"]);

        for order in self {
            let status_cell = match order.status {
                OrderStatus::Complete => Cell::new("COMPLETE").fg(Color::Green),
                OrderStatus::Open => Cell::new("OPEN").fg(Color::Yellow),
                OrderStatus::Cancelled => Cell::new("CANCELLED").fg(Color::Red),
                OrderStatus::Rejected => Cell::new("REJECTED").fg(Color::Red),
                _ => Cell::new(format!("{:?}", order.status)),
            };

            table.add_row(vec![
                Cell::new(&order.order_id),
                Cell::new(&order.tradingsymbol),
                Cell::new(format!("{:?}", order.transaction_type)),
                Cell::new(order.quantity.to_string()),
                Cell::new(format!("₹{:.2}", order.price)),
                status_cell,
                Cell::new(&order.order_timestamp),
            ]);
        }

        table.set_content_arrangement(ContentArrangement::Dynamic);
        println!("{table}");
        Ok(())
    }

    fn print_json(&self) -> Result<()> {
        println!("{}", serde_json::to_string_pretty(self)?);
        Ok(())
    }
}
