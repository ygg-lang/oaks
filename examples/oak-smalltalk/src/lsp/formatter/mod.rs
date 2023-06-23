#![doc = include_str!("readme.md")]
use crate::language::SmalltalkLanguage;
use oak_core::RedNode;
use oak_pretty_print::{FormatConfig, FormatOutput, FormatResult, Formatter};
/// Smalltalk Code Formatter
pub struct SmalltalkFormatter {
    inner: Formatter<SmalltalkLanguage>,
}

impl SmalltalkFormatter {
    pub fn new(config: FormatConfig) -> Self {
        Self { inner: Formatter::new(config) }
    }

    pub fn format(&mut self, root: &RedNode<SmalltalkLanguage>, source: &str) -> FormatResult<FormatOutput> {
        self.inner.format(root, source)
    }
}
