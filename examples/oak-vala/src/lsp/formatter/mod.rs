#![doc = include_str!("readme.md")]
use crate::language::ValaLanguage;
use oak_core::tree::RedNode;
use oak_pretty_print::{FormatConfig, FormatOutput, FormatResult, Formatter};

/// Vala 语言的格式化器
pub struct ValaFormatter {
    inner: Formatter<ValaLanguage>,
}

impl ValaFormatter {
    pub fn new(config: FormatConfig) -> Self {
        Self { inner: Formatter::new(config) }
    }

    pub fn format(&mut self, root: &RedNode<ValaLanguage>, source: &str) -> FormatResult<FormatOutput> {
        self.inner.format(root, source)
    }
}
