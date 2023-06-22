use oak_core::errors::ParseResult;
use oak_highlight::{
    highlighter::{HighlightResult, Highlighter},
    themes::Theme,
};

/// SCSS 高亮
pub struct ScssHighlighter;

impl ScssHighlighter {
    pub fn new() -> Self {
        Self
    }
}

impl Highlighter for ScssHighlighter {
    fn highlight<'a>(&self, source: &'a str, _language: &str, _theme: Theme) -> ParseResult<HighlightResult<'a>> {
        // 暂时返回一个简单的结果
        Ok(HighlightResult { segments: vec![], source: std::borrow::Cow::Borrowed(source) })
    }
}
