use oak_core::errors::ParseResult;
use oak_highlight::{
    highlighter::{HighlightResult, Highlighter},
    themes::Theme,
};

/// Typst 语法高亮器
pub struct TypstHighlighter;

impl Highlighter for TypstHighlighter {
    fn highlight<'a>(&self, source: &'a str, _language: &str, _theme: Theme) -> ParseResult<HighlightResult<'a>> {
        Ok(HighlightResult { segments: Vec::new(), source: std::borrow::Cow::Borrowed(source) })
    }
}
