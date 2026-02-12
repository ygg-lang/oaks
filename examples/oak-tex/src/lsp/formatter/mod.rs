#![doc = include_str!("readme.md")]
use crate::language::TexLanguage;
use oak_core::tree::RedNode;
use oak_pretty_print::{FormatConfig, FormatOutput, FormatResult, Formatter};

/// A formatter for the TeX language.
pub struct TexFormatter {
    inner: Formatter<TexLanguage>,
}

impl TexFormatter {
    /// Creates a new TeX formatter.
    pub fn new(config: FormatConfig) -> Self {
        Self { inner: Formatter::new(config) }
    }

    /// Formats the TeX source code.
    pub fn format(&mut self, root: &RedNode<TexLanguage>, source: &str) -> FormatResult<FormatOutput> {
        self.inner.format(root, source)
    }
}

impl Default for TexFormatter {
    fn default() -> Self {
        Self::new(FormatConfig::default())
    }
}
