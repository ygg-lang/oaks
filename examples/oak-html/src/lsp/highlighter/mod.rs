#![doc = include_str!("readme.md")]
use crate::language::HtmlLanguage;
use oak_core::{Language, errors::ParseResult};
use oak_highlight::{
    highlighter::{HighlightResult, Highlighter, OakHighlighter},
    themes::Theme,
};

/// A highlighter for HTML source code that supports both lexer-based and parser-based highlighting.
pub struct HtmlHighlighter {
    use_parser: bool,
}

impl HtmlHighlighter {
    /// Creates a new `HtmlHighlighter`.
    ///
    /// If `use_parser` is true, it will use the full parser for more accurate highlighting;
    /// otherwise, it will fall back to a faster lexer-only approach.
    pub fn new(use_parser: bool) -> Self {
        Self { use_parser }
    }
}

impl Highlighter for HtmlHighlighter {
    fn highlight<'a>(&self, source: &'a str, _language: &str, theme: Theme) -> ParseResult<HighlightResult<'a>> {
        let highlighter = OakHighlighter::new();

        if self.use_parser {
            let config = HtmlLanguage::new();
            let parser = crate::parser::HtmlParser::new(config);
            let lexer = crate::lexer::HtmlLexer::new(&config);
            highlighter.highlight_with_language::<HtmlLanguage, crate::parser::HtmlParser, crate::lexer::HtmlLexer<'_>>(source, theme, &parser, &lexer)
        }
        else {
            highlighter.highlight(source, HtmlLanguage::NAME, theme)
        }
    }
}
