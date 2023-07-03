#![doc = include_str!("readme.md")]
use crate::language::ValaLanguage;
use oak_core::tree::RedNode;

/// Formatter for the Vala language.
pub struct ValaFormatter {
    /// Indentation size.
    indent_size: usize,
}

impl ValaFormatter {
    /// Creates a new Vala formatter with default settings.
    pub fn new() -> Self {
        Self { indent_size: 4 }
    }

    /// Formats the given Vala source code.
    pub fn format(&self, root: &RedNode<ValaLanguage>, source: &str) -> String {
        // Simple formatting implementation
        source.to_string()
    }
}
