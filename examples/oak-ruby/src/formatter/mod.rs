#![doc = include_str!("../readme.md")]

/// Ruby Code Formatter
pub struct RubyFormatter {}

impl RubyFormatter {
    /// Create a new Ruby formatter
    pub fn new() -> Self {
        Self {}
    }

    /// Format the given Ruby source code string
    pub fn format(&self, source: &str) -> String {
        source.to_string()
    }
}

impl Default for RubyFormatter {
    fn default() -> Self {
        Self::new()
    }
}
