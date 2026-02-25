//! Output formatting for CLI commands

use crate::models::{Holding, Instrument, Order, Position};
use comfy_table::{
    modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Attribute, Cell, Color, ContentArrangement,
    Table,
};

/// Trait for formatted output
pub trait OutputFormatter {
    /// Print as table
    fn print(&self) -> anyhow::Result<()>;

    /// Print as JSON
    fn print_json(&self) -> anyhow::Result<()>;
}

impl OutputFormatter for Vec<Holding> {
    fn print(&self) -> anyhow::Result<()> {
        if self.is_empty() {
            println!("No holdings found");
            return Ok(());
        }

        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["Symbol", "Qty", "Avg Price", "LTP", "P&L", "Day Chg%"]);

        for holding in self {
            let pnl_cell = cell_color(format!("₹{:.2}", holding.pnl), holding.pnl >= 0.0, true);

            let chg_cell = cell_color(
                format!("{:.2}%", holding.day_change_percentage),
                holding.day_change_percentage >= 0.0,
                false,
            );

            table.add_row(vec![
                Cell::new(&holding.tradingsymbol),
                Cell::new(holding.quantity.to_string()),
                Cell::new(format!("₹{:.2}", holding.average_price)),
                Cell::new(format!("₹{:.2}", holding.last_price)),
                pnl_cell,
                chg_cell,
            ]);
        }

        println!("{table}");
        Ok(())
    }

    fn print_json(&self) -> anyhow::Result<()> {
        println!("{}", serde_json::to_string_pretty(self)?);
        Ok(())
    }
}

impl OutputFormatter for Vec<Order> {
    fn print(&self) -> anyhow::Result<()> {
        if self.is_empty() {
            println!("No orders found");
            return Ok(());
        }

        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec![
                "Order ID", "Symbol", "Type", "Qty", "Price", "Status", "Time",
            ]);

        for order in self {
            let status_cell = cell_order_status(&order.status);

            table.add_row(vec![
                Cell::new(&order.order_id),
                Cell::new(&order.tradingsymbol),
                Cell::new(format!("{:?}", order.transaction_type)),
                Cell::new(order.quantity.to_string()),
                Cell::new(format!("₹{:.2}", order.price)),
                status_cell,
                Cell::new(format_time(&order.order_timestamp)),
            ]);
        }

        println!("{table}");
        Ok(())
    }

    fn print_json(&self) -> anyhow::Result<()> {
        println!("{}", serde_json::to_string_pretty(self)?);
        Ok(())
    }
}

impl OutputFormatter for Vec<Position> {
    fn print(&self) -> anyhow::Result<()> {
        if self.is_empty() {
            println!("No positions found");
            return Ok(());
        }

        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec![
                "Symbol",
                "Product",
                "Qty",
                "Avg Price",
                "LTP",
                "P&L",
                "Unrealised",
            ]);

        for position in self {
            let pnl_cell = cell_color(format!("₹{:.2}", position.pnl), position.pnl >= 0.0, true);

            let unrealised_cell = cell_color(
                format!("₹{:.2}", position.unrealised),
                position.unrealised >= 0.0,
                true,
            );

            table.add_row(vec![
                Cell::new(&position.tradingsymbol),
                Cell::new(format!("{:?}", position.product)),
                Cell::new(position.quantity.to_string()),
                Cell::new(format!("₹{:.2}", position.average_price)),
                Cell::new(format!("₹{:.2}", position.last_price)),
                pnl_cell,
                unrealised_cell,
            ]);
        }

        println!("{table}");
        Ok(())
    }

    fn print_json(&self) -> anyhow::Result<()> {
        println!("{}", serde_json::to_string_pretty(self)?);
        Ok(())
    }
}

impl OutputFormatter for Vec<Instrument> {
    fn print(&self) -> anyhow::Result<()> {
        if self.is_empty() {
            println!("No instruments found");
            return Ok(());
        }

        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec![
                "Symbol", "Name", "Exchange", "Segment", "Type", "Lot Size",
            ]);

        for instrument in self {
            table.add_row(vec![
                Cell::new(&instrument.tradingsymbol),
                Cell::new(&instrument.name),
                Cell::new(format!("{:?}", instrument.exchange)),
                Cell::new(format!("{:?}", instrument.segment)),
                Cell::new(format!("{:?}", instrument.instrument_type)),
                Cell::new(instrument.lot_size.to_string()),
            ]);
        }

        println!("{table}");
        Ok(())
    }

    fn print_json(&self) -> anyhow::Result<()> {
        println!("{}", serde_json::to_string_pretty(self)?);
        Ok(())
    }
}

/// Create a colored cell based on value
fn cell_color(value: String, is_positive: bool, bold: bool) -> Cell {
    let color = if is_positive {
        Color::Green
    } else {
        Color::Red
    };

    let mut cell = Cell::new(value).fg(color);

    if bold {
        cell = cell.add_attribute(Attribute::Bold);
    }

    cell
}

/// Create a colored cell for order status
fn cell_order_status(status: &crate::models::OrderStatus) -> Cell {
    let (text, color) = match status {
        crate::models::OrderStatus::Complete => ("COMPLETE", Color::Green),
        crate::models::OrderStatus::Open => ("OPEN", Color::Yellow),
        crate::models::OrderStatus::Cancelled => ("CANCELLED", Color::Red),
        crate::models::OrderStatus::Rejected => ("REJECTED", Color::Red),
        crate::models::OrderStatus::TriggerPending => ("TRIGGER PENDING", Color::Cyan),
        crate::models::OrderStatus::ValidationPending => ("VALIDATION PENDING", Color::Cyan),
    };

    Cell::new(text).fg(color)
}

/// Format timestamp for display
fn format_time(timestamp: &str) -> String {
    // Try to parse ISO format and format nicely
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(timestamp) {
        dt.format("%Y-%m-%d %H:%M").to_string()
    } else if let Some(ts) = timestamp.strip_suffix("Z") {
        // Try with Z suffix removed
        if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&format!("{}Z", ts)) {
            dt.format("%Y-%m-%d %H:%M").to_string()
        } else {
            timestamp.to_string()
        }
    } else {
        timestamp.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_time() {
        let result = format_time("2024-02-25T10:30:00+05:30");
        assert_eq!(result, "2024-02-25 10:30");
    }
}
