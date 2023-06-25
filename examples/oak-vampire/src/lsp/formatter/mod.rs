#![doc = include_str!("readme.md")]
use crate::language::VampireLanguage;
use oak_core::tree::RedNode;
use oak_pretty_print::{FormatConfig, FormatOutput, FormatResult, Formatter};

/// Vampire language formatter
pub struct VampireFormatter {
    inner: Formatter<VampireLanguage>,
}

impl VampireFormatter {
    pub fn new(config: FormatConfig) -> Self {
        Self { inner: Formatter::new(config) }
    }

    pub fn format(&mut self, root: &RedNode<VampireLanguage>, source: &str) -> FormatResult<FormatOutput> {
        self.inner.format(root, source)
    }
}
