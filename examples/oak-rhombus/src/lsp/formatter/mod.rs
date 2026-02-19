#![doc = include_str!("readme.md")]

/// Rhombus Code Formatter
pub struct RhombusFormatter {
    _indent_level: usize,
    _indent_str: String,
}

/// Alias for backwards compatibility
pub type SchemeFormatter = RhombusFormatter;

impl RhombusFormatter {
    /// Create a new Scheme formatter
    pub fn new() -> Self {
        Self {
            _indent_level: 0,
            _indent_str: "  ".to_string(), // 2 spaces for Scheme
        }
    }

    /// Format the given Scheme source code string
    pub fn format(&self, source: &str) -> String {
        source.to_string()
    }
}

impl Default for SchemeFormatter {
    fn default() -> Self {
        Self::new()
    }
}
