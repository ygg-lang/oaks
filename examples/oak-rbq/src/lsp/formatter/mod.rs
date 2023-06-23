#![doc = include_str!("readme.md")]
// mod as_document;

use crate::{ast::RbqRoot, language::RbqLanguage};
use oak_pretty_print::{AsDocument, FormatConfig};

pub struct RbqFormatter<'config> {
    _config: &'config RbqLanguage,
}

impl<'config> RbqFormatter<'config> {
    pub fn new(config: &'config RbqLanguage) -> Self {
        Self { _config: config }
    }

    pub fn format(&self, node: &oak_core::tree::RedNode<RbqLanguage>, source: &str) -> String {
        source[node.span()].to_string()
    }
}
