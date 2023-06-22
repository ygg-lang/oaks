use oak_core::errors::ParseResult;
use oak_highlight::{HighlightResult, Highlighter, OakHighlighter, themes::Theme};

/// Vampire 语法高亮器
pub struct VampireHighlighter;

impl Highlighter for VampireHighlighter {
    fn highlight<'a>(&self, source: &'a str, language: &str, theme: Theme) -> ParseResult<HighlightResult<'a>> {
        let highlighter = OakHighlighter::new().theme(theme);
        highlighter.highlight(source, language, theme)
    }
}
