#![doc = include_str!("../readme.md")]

use crate::ast::AdaRoot;

/// Ada Code Formatter
pub struct AdaFormatter;

impl AdaFormatter {
    /// Create a new Ada formatter
    pub fn new() -> Self {
        Self
    }

    /// Format the given Ada source code string
    pub fn format(&self, source: &str) -> String {
        self.basic_format(source)
    }

    fn basic_format(&self, source: &str) -> String {
        source.to_string()
    }

    /// Format Ada AST root node
    pub fn format_root(&self, _root: &AdaRoot) -> String {
        "TODO: Implement Ada AST formatting".to_string()
    }
}

impl Default for AdaFormatter {
    fn default() -> Self {
        Self::new()
    }
}
