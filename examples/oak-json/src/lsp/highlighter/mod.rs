#![doc = include_str!("readme.md")]
/// Local definition of highlight kinds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    /// Keywords
    Keyword,
    /// Strings
    String,
    /// Numbers
    Number,
    /// Comments
    Comment,
    /// Identifiers
    Identifier,
    /// Literals (null, true, false)
    Literal,
}

/// Highlighter trait
pub trait Highlighter {
    /// Highlights the given text
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

/// JSON syntax highlighter
pub struct JsonHighlighter;

impl JsonHighlighter {
    pub fn new() -> Self {
        Self
    }
}

impl Highlighter for JsonHighlighter {
    fn highlight(&self, _text: &str) -> Vec<(usize, usize, HighlightKind)> {
        // TODO: Implement highlighter based on lexer
        Vec::new()
    }
}
