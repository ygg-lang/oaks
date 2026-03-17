#![doc = include_str!("readme.md")]
use crate::language::TexLanguage;
use oak_core::tree::RedNode;

/// A formatter for the TeX language.
pub struct TexFormatter {
    /// Indentation level
    indent_level: usize,
    /// Indentation string
    indent_str: String,
}

impl TexFormatter {
    /// Creates a new TeX formatter.
    pub fn new() -> Self {
        Self { indent_level: 0, indent_str: "    ".to_string() }
    }

    /// Formats the TeX source code.
    pub fn format(&self, root: &RedNode<TexLanguage>, source: &str) -> String {
        // Simple formatting implementation
        source.to_string()
    }
}

impl Default for TexFormatter {
    fn default() -> Self {
        Self::new()
    }
}
