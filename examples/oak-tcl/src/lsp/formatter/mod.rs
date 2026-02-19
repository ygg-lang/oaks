#![doc = include_str!("readme.md")]

use crate::ast::TclRoot;

/// Tcl code formatter
pub struct TclFormatter {
    /// Indentation level
    pub indent_level: usize,
    /// Indentation string
    pub indent_str: String,
}

impl TclFormatter {
    /// Creates a new Tcl formatter
    pub fn new() -> Self {
        Self { indent_level: 0, indent_str: "    ".to_string() }
    }

    /// Formats the given Tcl source code string
    pub fn format(&self, source: &str) -> String {
        source.to_string()
    }

    /// Formats the Tcl AST root node
    pub fn format_ast(&self, _root: &TclRoot) -> String {
        String::new()
    }
}

impl Default for TclFormatter {
    fn default() -> Self {
        Self::new()
    }
}
