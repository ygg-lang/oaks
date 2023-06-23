#![doc = include_str!("readme.md")]

use crate::ast::AplRoot;

/// APL Code Formatter
pub struct AplFormatter;

impl AplFormatter {
    /// Create a new APL formatter
    pub fn new() -> Self {
        Self
    }

    /// Format the given APL source code string
    pub fn format(&self, source: &str) -> String {
        self.basic_format(source)
    }

    fn basic_format(&self, source: &str) -> String {
        source.to_string()
    }

    /// Format APL AST root node
    pub fn format_root(&self, _root: &AplRoot) -> String {
        "TODO: Implement APL AST formatting".to_string()
    }
}

impl Default for AplFormatter {
    fn default() -> Self {
        Self::new()
    }
}
