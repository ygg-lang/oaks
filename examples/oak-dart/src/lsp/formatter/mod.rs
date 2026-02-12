#![doc = include_str!("readme.md")]
use crate::ast::DartRoot;

/// Dart Code Formatter
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct DartFormatter {
    pub indent_size: usize,
}

impl Default for DartFormatter {
    fn default() -> Self {
        Self { indent_size: 2 }
    }
}

impl DartFormatter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn format(&self, source: &str) -> String {
        // Basic implementation for now
        source.to_string()
    }

    pub fn format_ast(&self, _root: &DartRoot) -> String {
        // AST-based formatting
        String::new()
    }
}
