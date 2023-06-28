use crate::language::LiquidLanguage;
/// Formatter module for Liquid
///
/// This module provides code formatting support for Liquid templates.

/// Formatter for Liquid templates
#[derive(Debug, Clone)]
pub struct LiquidFormatter {
    indent_size: usize,
}

impl LiquidFormatter {
    /// Creates a new Liquid formatter
    pub fn new() -> Self {
        Self { indent_size: 4 }
    }

    /// Format the given source code
    pub fn format(&self, source: &str) -> String {
        source.to_string()
    }
}

impl Default for LiquidFormatter {
    fn default() -> Self {
        Self::new()
    }
}
