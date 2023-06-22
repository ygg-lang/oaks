use oak_core::errors::ParseResult;
use oak_highlight::{HighlightResult, highlighter::Highlighter, themes::Theme};

pub struct JavaScriptHighlighter;

impl Highlighter for JavaScriptHighlighter {
    fn highlight<'a>(&self, source: &'a str, _language: &str, _theme: Theme) -> ParseResult<HighlightResult<'a>> {
        Ok(HighlightResult { segments: Vec::new(), source: source.into() })
    }
}
