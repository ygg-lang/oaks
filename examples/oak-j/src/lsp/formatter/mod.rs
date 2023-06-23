#![doc = include_str!("readme.md")]

use crate::ast::JRoot;

/// J Code Formatter
pub struct JFormatter;

impl JFormatter {
    /// Create a new J formatter
    pub fn new() -> Self {
        Self
    }

    /// Format the given J source code string
    pub fn format(&self, source: &str) -> String {
        self.basic_format(source)
    }

    fn basic_format(&self, source: &str) -> String {
        source.to_string()
    }

    /// Format J AST root node
    pub fn format_root(&self, _root: &JRoot) -> String {
        "TODO: Implement J AST formatting".to_string()
    }
}

impl Default for JFormatter {
    fn default() -> Self {
        Self::new()
    }
}
