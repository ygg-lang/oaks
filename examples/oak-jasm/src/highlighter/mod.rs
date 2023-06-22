use oak_core::{Language, errors::ParseResult};
use oak_highlight::{
    highlighter::{HighlightResult, Highlighter, OakHighlighter},
    themes::Theme,
};

/// JASM 高亮器
pub struct JasmHighlighter {
    use_parser: bool,
}

impl JasmHighlighter {
    pub fn new(use_parser: bool) -> Self {
        Self { use_parser }
    }
}

impl Highlighter for JasmHighlighter {
    fn highlight<'a>(&self, source: &'a str, _language: &str, theme: Theme) -> ParseResult<HighlightResult<'a>> {
        let highlighter = OakHighlighter::new();

        if self.use_parser {
            let config = crate::language::JasmLanguage::default();
            let parser = crate::parser::JasmParser::new(&config);
            let lexer = crate::lexer::JasmLexer::new(&config);
            highlighter.highlight_with_language::<crate::language::JasmLanguage, crate::parser::JasmParser<'_>, crate::lexer::JasmLexer>(source, theme, &parser, &lexer)
        }
        else {
            highlighter.highlight(source, crate::language::JasmLanguage::NAME, theme)
        }
    }
}
