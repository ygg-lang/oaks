#![doc = include_str!("readme.md")]
//! Smalltalk 语法高亮器
use oak_core::errors::ParseResult;
use oak_highlight::{HighlightResult, Highlighter, themes::Theme};

/// Smalltalk 语法高亮器
pub struct SmalltalkHighlighter;

impl SmalltalkHighlighter {
    /// 创建一个新的 Smalltalk 高亮器实例
    pub fn new() -> Self {
        Self
    }
}

impl Default for SmalltalkHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

impl Highlighter for SmalltalkHighlighter {
    fn highlight<'a>(&self, source: &'a str, _language: &str, _theme: Theme) -> ParseResult<HighlightResult<'a>> {
        // TODO: 实现真正的 Smalltalk 高亮逻辑
        Ok(HighlightResult { segments: Vec::new(), source: std::borrow::Cow::Borrowed(source) })
    }
}
