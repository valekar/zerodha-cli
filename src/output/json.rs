//! JSON formatter

use serde_json;

/// JSON formatter
pub struct JsonFormatter;

impl JsonFormatter {
    pub fn new() -> Self {
        JsonFormatter
    }

    pub fn format<T: serde::Serialize>(&self, data: &T) -> String {
        serde_json::to_string_pretty(data).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn format_error(&self, message: &str) -> String {
        serde_json::json!({ "error": message }).to_string()
    }
}

impl Default for JsonFormatter {
    fn default() -> Self {
        Self::new()
    }
}
