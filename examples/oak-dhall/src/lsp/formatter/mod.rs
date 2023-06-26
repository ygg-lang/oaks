#![doc = include_str!("readme.md")]

/// Formatter implementation for DHall.
pub struct DHallFormatter {
    _indent_level: usize,
    _indent_str: String,
}

impl DHallFormatter {
    /// Creates a new `DHallFormatter`.
    pub fn new() -> Self {
        Self { _indent_level: 0, _indent_str: "  ".to_string() }
    }

    /// Formats the given source code.
    pub fn format(&self, source: &str) -> String {
        // TODO: Implement real DHall formatting logic
        source.to_string()
    }
}
