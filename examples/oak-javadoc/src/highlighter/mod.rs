use oak_core::errors::ParseResult;
use oak_highlight::{HighlightResult, highlighter::Highlighter};

pub struct JavadocHighlighter;

impl Highlighter for JavadocHighlighter {
    fn highlight<'a>(&self, source: &'a str, _language: &str, _theme: oak_highlight::themes::Theme) -> ParseResult<HighlightResult<'a>> {
        // FIXME: Implement real highlighting
        Ok(HighlightResult { segments: vec![], source: std::borrow::Cow::Borrowed(source) })
    }
}
