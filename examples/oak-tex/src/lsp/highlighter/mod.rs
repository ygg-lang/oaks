#![doc = include_str!("readme.md")]
use oak_core::errors::ParseResult;
use oak_highlight::{HighlightResult, Highlighter, themes::Theme};
use std::borrow::Cow;

/// TeX 语言的高亮器
#[derive(Default, Clone)]
pub struct TexHighlighter;

impl Highlighter for TexHighlighter {
    fn highlight<'a>(&self, source: &'a str, _language: &str, _theme: Theme) -> ParseResult<HighlightResult<'a>> {
        // TODO: 实现真正的 TeX 高亮逻辑
        Ok(HighlightResult { source: Cow::Borrowed(source), segments: Vec::new() })
    }
}

impl TexHighlighter {
    /// 创建新的 TeX 高亮器
    pub fn new() -> Self {
        Self
    }
}
