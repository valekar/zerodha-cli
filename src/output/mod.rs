//! Output formatting

pub mod table;
pub mod json;

pub use table::TableFormatter;
pub use json::JsonFormatter;

/// Output format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Table,
    Json,
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Table
    }
}
