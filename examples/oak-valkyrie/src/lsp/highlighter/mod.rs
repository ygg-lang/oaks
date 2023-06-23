#![doc = include_str!("readme.md")]
use oak_core::errors::ParseResult;
use oak_highlight::{
    HighlightResult,
    highlighter::{Highlighter, OakHighlighter},
    themes::Theme,
};

/// A syntax highlighter for Valkyrie source code.
pub struct ValkyrieHighlighter {
    /// Whether to use parser-based highlighting for enhanced accuracy
    pub use_parser: bool,
}

impl Default for ValkyrieHighlighter {
    fn default() -> Self {
        Self { use_parser: true }
    }
}

impl ValkyrieHighlighter {
    /// Creates a new Valkyrie highlighter.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Highlighter for ValkyrieHighlighter {
    fn highlight<'a>(&self, source: &'a str, _language: &str, theme: Theme) -> ParseResult<HighlightResult<'a>> {
        let highlighter = OakHighlighter::new().theme(theme);

        if self.use_parser {
            let config = crate::ValkyrieLanguage::default();
            let parser = crate::ValkyrieParser::new(&config);
            let lexer = crate::ValkyrieLexer::new(&config);
            highlighter.highlight_with_language(source, theme, &parser, &lexer)
        }
        else {
            highlighter.highlight(source, "valkyrie", theme)
        }
    }
}
