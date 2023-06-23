#![doc = include_str!("readme.md")]
use crate::language::TexLanguage;
use oak_core::tree::RedNode;
use oak_pretty_print::{FormatConfig, FormatOutput, FormatResult, Formatter};

/// TeX 语言的格式化器
pub struct TexFormatter {
    inner: Formatter<TexLanguage>,
}

impl TexFormatter {
    pub fn new(config: FormatConfig) -> Self {
        Self { inner: Formatter::new(config) }
    }

    pub fn format(&mut self, root: &RedNode<TexLanguage>, source: &str) -> FormatResult<FormatOutput> {
        self.inner.format(root, source)
    }
}

impl Default for TexFormatter {
    fn default() -> Self {
        Self::new(FormatConfig::default())
    }
}
