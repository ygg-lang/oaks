#![doc = include_str!("readme.md")]

/// Formatter implementation for CSS.
pub struct CssFormatter {}

impl CssFormatter {
    /// Formats the given source.
    pub fn format(&self, source: &str) -> String {
        source.to_string()
    }
}
