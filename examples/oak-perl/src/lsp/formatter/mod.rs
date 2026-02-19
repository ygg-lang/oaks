#![doc = include_str!("readme.md")]

use crate::ast::PerlRoot;

/// Perl code formatter
pub struct PerlFormatter {
    /// Indentation level
    pub indent_level: usize,
    /// Indentation string
    pub indent_str: String,
}

impl PerlFormatter {
    /// Creates a new Perl formatter
    pub fn new() -> Self {
        Self { indent_level: 0, indent_str: "    ".to_string() }
    }

    /// Formats the given Perl source code string
    pub fn format(&self, source: &str) -> String {
        source.to_string()
    }

    /// Formats the Perl AST root node
    pub fn format_ast(&self, _root: &PerlRoot) -> String {
        String::new()
    }
}

impl Default for PerlFormatter {
    fn default() -> Self {
        Self::new()
    }
}
