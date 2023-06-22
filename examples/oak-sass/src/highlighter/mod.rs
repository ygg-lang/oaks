use oak_core::errors::ParseResult;
use oak_highlight::{
    highlighter::{HighlightResult, Highlighter},
    themes::Theme,
};

/// Sass 高亮
pub struct SassHighlighter {}

impl SassHighlighter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Highlighter for SassHighlighter {
    fn highlight<'a>(&self, source: &'a str, _language: &str, _theme: Theme) -> ParseResult<HighlightResult<'a>> {
        // 暂时返回一个简单的结果
        Ok(HighlightResult { segments: vec![], source: std::borrow::Cow::Borrowed(source) })
    }
}
