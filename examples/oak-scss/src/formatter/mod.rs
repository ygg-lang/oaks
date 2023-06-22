#![doc = include_str!("../readme.md")]

/// SCSS Code Formatter
pub struct ScssFormatter;

impl ScssFormatter {
    pub fn new() -> Self {
        Self
    }

    pub fn format(&self, source: &str) -> String {
        source.to_string()
    }
}
