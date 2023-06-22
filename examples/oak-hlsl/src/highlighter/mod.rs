use oak_core::{Language, errors::ParseResult};
use oak_highlight::{
    highlighter::{HighlightResult, Highlighter, OakHighlighter},
    themes::Theme,
};

pub struct HlslHighlighter {
    use_parser: bool,
}

impl HlslHighlighter {
    pub fn new(use_parser: bool) -> Self {
        Self { use_parser }
    }
}

impl Highlighter for HlslHighlighter {
    fn highlight<'a>(&self, source: &'a str, _language: &str, theme: Theme) -> ParseResult<HighlightResult<'a>> {
        let highlighter = OakHighlighter::new();

        if self.use_parser {
            let config = crate::language::HlslLanguage::default();
            let parser = crate::parser::HlslParser::new(&config);
            let lexer = crate::lexer::HlslLexer::new(&config);
            highlighter.highlight_with_language::<crate::language::HlslLanguage, crate::parser::HlslParser<'_>, crate::lexer::HlslLexer<'_>>(source, theme, &parser, &lexer)
        }
        else {
            highlighter.highlight(source, crate::language::HlslLanguage::NAME, theme)
        }
    }
}
