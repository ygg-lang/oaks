#![doc = include_str!("readme.md")]
use crate::language::ValaLanguage;
use oak_core::tree::RedNode;
use oak_pretty_print::{FormatConfig, FormatOutput, FormatResult, Formatter};

/// Formatter for the Vala language.
pub struct ValaFormatter {
    inner: Formatter<ValaLanguage>,
}

impl ValaFormatter {
    /// Creates a new Vala formatter with the given configuration.
    pub fn new(config: FormatConfig) -> Self {
        Self { inner: Formatter::new(config) }
    }

    /// Formats the given Vala source code.
    pub fn format(&mut self, root: &RedNode<ValaLanguage>, source: &str) -> FormatResult<FormatOutput> {
        self.inner.format(root, source)
    }
}
