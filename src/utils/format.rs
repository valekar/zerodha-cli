//! Number formatting utilities

/// Format a number with commas
pub fn format_number(value: f64, decimals: usize) -> String {
    let formatted = format!("{:.prec$}", value, prec = decimals);
    
    let parts: Vec<&str> = formatted.split('.').collect();
    let integer_part = parts[0];
    let decimal_part = parts.get(1);
    
    // Add commas to integer part
    let mut result = String::new();
    let integer_str = integer_part.trim_start_matches('-');
    
    for (i, c) in integer_str.chars().enumerate() {
        if i > 0 && (integer_str.len() - i) % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    
    if integer_part.starts_with('-') {
        result = format!("-{}", result);
    }
    
    if let Some(dec) = decimal_part {
        if !dec.is_empty() {
            result = format!("{}.{}", result, dec);
        }
    }
    
    result
}

/// Format currency
pub fn format_currency(value: f64) -> String {
    format!("₹{}", format_number(value, 2))
}

/// Format percentage
pub fn format_percentage(value: f64) -> String {
    format!("{:.2}%", value)
}
