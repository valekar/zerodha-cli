//! Color utilities for terminal output

/// Terminal colors
#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Reset,
}

impl Color {
    /// Get ANSI escape code
    pub fn code(&self) -> &str {
        match self {
            Color::Black => "\x1b[30m",
            Color::Red => "\x1b[31m",
            Color::Green => "\x1b[32m",
            Color::Yellow => "\x1b[33m",
            Color::Blue => "\x1b[34m",
            Color::Magenta => "\x1b[35m",
            Color::Cyan => "\x1b[36m",
            Color::White => "\x1b[37m",
            Color::Reset => "\x1b[0m",
        }
    }
}

/// Colorize a string
pub fn colorize(s: &str, color: Color) -> String {
    format!("{}{}{}", color.code(), s, Color::Reset.code())
}

/// Format positive value (green)
pub fn positive(s: &str) -> String {
    colorize(s, Color::Green)
}

/// Format negative value (red)
pub fn negative(s: &str) -> String {
    colorize(s, Color::Red)
}

/// Format warning (yellow)
pub fn warning(s: &str) -> String {
    colorize(s, Color::Yellow)
}
