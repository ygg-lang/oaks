#![doc = include_str!("readme.md")]
//! Raku code formatter

use crate::ast::RakuRoot;

/// Raku code formatter
pub struct RakuFormatter {
    /// Indentation level
    pub indent_level: usize,
    /// Indentation string
    pub indent_str: String,
}

impl RakuFormatter {
    /// Creates a new Raku formatter
    pub fn new() -> Self {
        Self { indent_level: 0, indent_str: "    ".to_string() }
    }

    /// Formats the given Raku source code string
    pub fn format(&self, source: &str) -> String {
        source.to_string()
    }

    /// Formats the Raku AST root node
    pub fn format_ast(&self, _root: &RakuRoot) -> String {
        String::new()
    }
}

impl Default for RakuFormatter {
    fn default() -> Self {
        Self::new()
    }
}
