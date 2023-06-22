use crate::language::HtmlLanguage;
use oak_core::{Language, errors::ParseResult};
use oak_highlight::{
    highlighter::{HighlightResult, Highlighter, OakHighlighter},
    themes::Theme,
};

/// HTML Highlighter
pub struct HtmlHighlighter {
    use_parser: bool,
}

impl HtmlHighlighter {
    pub fn new(use_parser: bool) -> Self {
        Self { use_parser }
    }
}

impl Highlighter for HtmlHighlighter {
    fn highlight<'a>(&self, source: &'a str, _language: &str, theme: Theme) -> ParseResult<HighlightResult<'a>> {
        let highlighter = OakHighlighter::new();

        if self.use_parser {
            let config = HtmlLanguage::new();
            let parser = crate::parser::HtmlParser::new(&config);
            let lexer = crate::lexer::HtmlLexer::new(&config);
            highlighter.highlight_with_language::<HtmlLanguage, crate::parser::HtmlParser<'_>, crate::lexer::HtmlLexer<'_>>(source, theme, &parser, &lexer)
        }
        else {
            highlighter.highlight(source, HtmlLanguage::NAME, theme)
        }
    }
}
