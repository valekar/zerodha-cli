//! Table formatter

use comfy_table::{Table, Cell, Color};
use crate::api::types::*;

/// Table formatter
pub struct TableFormatter {
    color: bool,
}

impl TableFormatter {
    pub fn new(color: bool) -> Self {
        TableFormatter { color }
    }

    pub fn format_quotes(&self, _quotes: &std::collections::HashMap<String, Quote>) -> String {
        let mut table = Table::new();
        table.set_header(vec!["Symbol", "LTP", "Change", "%", "Open", "High", "Low", "Close"]);
        // TODO: Implement actual formatting
        table.to_string()
    }

    pub fn format_holdings(&self, holdings: &[Holding]) -> String {
        let mut table = Table::new();
        table.set_header(vec!["Symbol", "Qty", "Avg", "LTP", "P&L", "Day Change", "%"]);

        for h in holdings {
            let pnl_color = if h.pnl >= 0.0 { Color::Green } else { Color::Red };
            let day_color = if h.day_change >= 0.0 { Color::Green } else { Color::Red };

            table.add_row(vec![
                Cell::new(&h.tradingsymbol),
                Cell::new(format!("{}", h.quantity)),
                Cell::new(format!("{:.2}", h.average_price)),
                Cell::new(format!("{:.2}", h.last_price)),
                Cell::new(format!("{:.2}", h.pnl)).fg(pnl_color),
                Cell::new(format!("{:.2}", h.day_change)).fg(day_color),
                Cell::new(format!("{:.2}%", h.day_change_percentage)).fg(day_color),
            ]);
        }

        table.to_string()
    }

    pub fn format_orders(&self, orders: &[Order]) -> String {
        let mut table = Table::new();
        table.set_header(vec!["Order ID", "Symbol", "Type", "Qty", "Price", "Status"]);

        for o in orders {
            table.add_row(vec![
                Cell::new(&o.order_id),
                Cell::new(&o.tradingsymbol),
                Cell::new(format!("{:?}", o.transaction_type)),
                Cell::new(format!("{}/{}", o.filled_quantity, o.quantity)),
                Cell::new(format!("{:.2}", o.price)),
                Cell::new(format!("{:?}", o.status)),
            ]);
        }

        table.to_string()
    }
}
