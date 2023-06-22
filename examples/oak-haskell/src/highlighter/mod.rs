use crate::{language::HaskellLanguage, lexer::HaskellLexer, parser::HaskellParser};
use oak_core::{Language, errors::ParseResult};
use oak_highlight::{
    highlighter::{HighlightResult, Highlighter, OakHighlighter},
    themes::Theme,
};

/// Haskell 语法高亮器
pub struct HaskellHighlighter {
    use_parser: bool,
}

impl HaskellHighlighter {
    pub fn new(use_parser: bool) -> Self {
        Self { use_parser }
    }
}

impl Default for HaskellHighlighter {
    fn default() -> Self {
        Self::new(true)
    }
}

impl Highlighter for HaskellHighlighter {
    fn highlight<'a>(&self, source: &'a str, _language: &str, theme: Theme) -> ParseResult<HighlightResult<'a>> {
        let highlighter = OakHighlighter::new();

        if self.use_parser {
            let config = HaskellLanguage::default();
            let parser = HaskellParser::new(&config);
            let lexer = HaskellLexer::new(&config);
            highlighter.highlight_with_language::<HaskellLanguage, HaskellParser<'_>, HaskellLexer<'_>>(source, theme, &parser, &lexer)
        }
        else {
            highlighter.highlight(source, HaskellLanguage::NAME, theme)
        }
    }
}
