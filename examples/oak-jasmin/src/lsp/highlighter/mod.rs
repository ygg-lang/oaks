#![doc = include_str!("readme.md")]
use crate::language::JasminLanguage;
use oak_core::errors::ParseResult;
use oak_highlight::{
    highlighter::{HighlightResult, Highlighter, OakHighlighter},
    themes::Theme,
};

pub struct JasminHighlighter;

impl JasminHighlighter {
    pub fn new() -> Self {
        Self
    }
}

impl Highlighter for JasminHighlighter {
    fn highlight<'a>(&self, source: &'a str, _language: &str, theme: Theme) -> ParseResult<HighlightResult<'a>> {
        let highlighter = OakHighlighter::new();
        let config = JasminLanguage::default();
        let parser = crate::parser::JasminParser::new(&config);
        let lexer = crate::lexer::JasminLexer::new(&config);
        highlighter.highlight_with_language::<JasminLanguage, crate::parser::JasminParser<'_>, crate::lexer::JasminLexer<'_>>(source, theme, &parser, &lexer)
    }
}
