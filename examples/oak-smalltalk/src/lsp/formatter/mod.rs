#![doc = include_str!("readme.md")]
use crate::language::SmalltalkLanguage;
use oak_core::RedNode;
/// Smalltalk Code Formatter
pub struct SmalltalkFormatter {
    /// Indentation size.
    indent_size: usize,
}

impl SmalltalkFormatter {
    pub fn new() -> Self {
        Self { indent_size: 4 }
    }

    pub fn format(&self, root: &RedNode<SmalltalkLanguage>, source: &str) -> String {
        // Simple formatting implementation
        source.to_string()
    }
}
