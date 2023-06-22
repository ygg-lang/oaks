use oak_core::{Language, errors::ParseResult};
use oak_highlight::{
    highlighter::{HighlightResult, Highlighter, OakHighlighter},
    themes::Theme,
};

pub struct IniHighlighter {
    use_parser: bool,
}

impl IniHighlighter {
    pub fn new(use_parser: bool) -> Self {
        Self { use_parser }
    }
}

impl Highlighter for IniHighlighter {
    fn highlight<'a>(&self, source: &'a str, _language: &str, theme: Theme) -> ParseResult<HighlightResult<'a>> {
        let highlighter = OakHighlighter::new();
        if self.use_parser {
            let config = crate::language::IniLanguage::default();
            let parser = crate::parser::IniParser::new(&config);
            let lexer = crate::lexer::IniLexer::new(&config);
            highlighter.highlight_with_language::<crate::language::IniLanguage, crate::parser::IniParser<'_>, crate::lexer::IniLexer>(source, theme, &parser, &lexer)
        }
        else {
            highlighter.highlight(source, crate::language::IniLanguage::NAME, theme)
        }
    }
}
