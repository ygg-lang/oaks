#![doc = include_str!("readme.md")]
use crate::ast::TypstRoot;

/// Typst language formatter
pub struct TypstFormatter;

impl TypstFormatter {
    pub fn new() -> Self {
        Self
    }

    pub fn format(&self, _root: &TypstRoot) -> String {
        // TODO: Implement concrete formatting logic
        String::new()
    }
}
