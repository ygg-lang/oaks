use crate::language::JinjaLanguage;
/// Formatter module for Jinja
///
/// This module provides code formatting support for Jinja templates.

/// Formatter for Jinja templates
#[derive(Debug, Clone)]
pub struct JinjaFormatter {
    indent_size: usize,
}

impl JinjaFormatter {
    /// Creates a new Jinja formatter
    pub fn new() -> Self {
        Self { indent_size: 4 }
    }

    /// Format the given source code
    pub fn format(&self, source: &str) -> String {
        source.to_string()
    }
}

impl Default for JinjaFormatter {
    fn default() -> Self {
        Self::new()
    }
}
