/// Type of highlighting to apply for Wolfram code.
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

/// Wolfram-specific syntax highlighter.
pub struct WolframHighlighter;

impl WolframHighlighter {
    /// Creates a new `WolframHighlighter`.
    pub fn new() -> Self {
        Self
    }

    /// Highlights the given text and returns a list of ranges with their corresponding highlight kind.
    pub fn highlight(&self, _text: &str) -> Vec<(usize, usize, HighlightKind)> {
        // TODO: Implement actual highlighting
        Vec::new()
    }
}
