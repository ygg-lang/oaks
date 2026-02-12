#![doc = include_str!("readme.md")]
mod as_document;

use crate::{ast::RbqRoot, language::RbqLanguage};
use oak_pretty_print::{AsDocument, FormatConfig};

/// RBQ formatter.
pub struct RbqFormatter<'config> {
    config: &'config RbqLanguage,
}

impl<'config> RbqFormatter<'config> {
    /// Creates a new RBQ formatter.
    pub fn new(config: &'config RbqLanguage) -> Self {
        Self { config }
    }

    /// Formats the given node.
    pub fn format(&self, node: &oak_core::tree::RedNode<RbqLanguage>, source: &str) -> String {
        source[node.span()].to_string()
    }
}
