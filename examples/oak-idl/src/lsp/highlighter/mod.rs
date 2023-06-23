#![doc = include_str!("readme.md")]
use oak_core::{Language, errors::ParseResult};
use oak_highlight::{
    highlighter::{HighlightResult, Highlighter, OakHighlighter},
    themes::Theme,
};

pub struct IdlHighlighter {
    use_parser: bool,
}

impl IdlHighlighter {
    pub fn new(use_parser: bool) -> Self {
        Self { use_parser }
    }
}

impl Highlighter for IdlHighlighter {
    fn highlight<'a>(&self, source: &'a str, _language: &str, theme: Theme) -> ParseResult<HighlightResult<'a>> {
        let highlighter = OakHighlighter::new();

        if self.use_parser {
            let config = crate::language::IdlLanguage::default();
            let parser = crate::parser::IdlParser::new(&config);
            let lexer = crate::lexer::IdlLexer::new(&config);
            highlighter.highlight_with_language::<crate::language::IdlLanguage, crate::parser::IdlParser<'_>, crate::lexer::IdlLexer<'_>>(source, theme, &parser, &lexer)
        }
        else {
            highlighter.highlight(source, crate::language::IdlLanguage::NAME, theme)
        }
    }
}
