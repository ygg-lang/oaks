use crate::language::StylusLanguage;
use oak_core::{Language, errors::ParseResult};
use oak_highlight::{
    highlighter::{HighlightResult, Highlighter, OakHighlighter},
    themes::Theme,
};

pub struct StylusHighlighter {
    use_parser: bool,
}

impl StylusHighlighter {
    pub fn new(use_parser: bool) -> Self {
        Self { use_parser }
    }
}

impl Highlighter for StylusHighlighter {
    fn highlight<'a>(&self, source: &'a str, _language: &str, theme: Theme) -> ParseResult<HighlightResult<'a>> {
        let highlighter = OakHighlighter::new();

        if self.use_parser {
            let config = StylusLanguage::new();
            let parser = crate::parser::StylusParser::new(&config);
            let lexer = crate::lexer::StylusLexer::new(&config);
            highlighter.highlight_with_language::<StylusLanguage, crate::parser::StylusParser<'_>, crate::lexer::StylusLexer>(source, theme, &parser, &lexer)
        }
        else {
            highlighter.highlight(source, StylusLanguage::NAME, theme)
        }
    }
}
