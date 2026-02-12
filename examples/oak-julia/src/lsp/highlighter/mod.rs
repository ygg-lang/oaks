#![doc = include_str!("readme.md")]
/// Type of highlighting to apply for Julia code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    /// Language keyword.
    Keyword,
    /// String literal.
    String,
    /// Numeric literal.
    Number,
    /// Comment block or line.
    Comment,
    /// Identifier or name.
    Identifier,
}

/// Julia-specific syntax highlighter.
pub struct JuliaHighlighter;

impl JuliaHighlighter {
    /// Creates a new `JuliaHighlighter`.
    pub fn new() -> Self {
        Self
    }

    /// Highlights the given text and returns a list of ranges with their corresponding highlight kind.
    pub fn highlight(&self, _text: &str) -> Vec<(usize, usize, HighlightKind)> {
        // TODO: Implement actual highlighting
        Vec::new()
    }
}
