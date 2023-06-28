#![doc = include_str!("readme.md")]

/// Formatter implementation for Less.
pub struct LessFormatter {}

impl LessFormatter {
    /// Formats the given source.
    pub fn format(&self, source: &str) -> String {
        source.to_string()
    }
}
