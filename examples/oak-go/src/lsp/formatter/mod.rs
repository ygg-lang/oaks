#![doc = include_str!("readme.md")]
//! Go formatter.

use oak_pretty_print::Document;

/// Formatter trait.
#[allow(dead_code)]
pub trait Formatter {
    /// Formats the given AST.
    fn format(&self, code: &str) -> Document<'_>;
}

/// Go formatter.
pub struct GoFormatter;

impl GoFormatter {
    pub fn new() -> Self {
        Self
    }
}

impl Formatter for GoFormatter {
    fn format(&self, _text: &str) -> Document<'_> {
        Document::Nil
    }
}
