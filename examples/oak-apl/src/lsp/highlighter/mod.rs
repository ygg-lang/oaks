#![doc = include_str!("readme.md")]

/// Represents the kind of highlighting for a syntax element.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    /// A keyword (typically special symbols in APL).
    Keyword,
    /// A string literal.
    String,
    /// A number literal.
    Number,
    /// A comment.
    Comment,
    /// An identifier.
    Identifier,
}

/// A trait for highlighting text.
pub trait Highlighter {
    /// Highlights the given text and returns a list of ranges with their highlight kind.
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

/// A highlighter for the APL language.
pub struct AplHighlighter {
    /// Whether to use the parser for highlighting.
    pub use_parser: bool,
}

impl Default for AplHighlighter {
    fn default() -> Self {
        Self { use_parser: false }
    }
}

impl AplHighlighter {
    /// Creates a new `AplHighlighter`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Highlights APL symbols (as keywords).
    fn highlight_symbols(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let symbols = ['←', '→', '⋄', '⎕', '⍞', '⍴', '⍳', '⍬', '⍣', '⍤', '⍛', '⍢', '⌸', '⌺', '⌼', '⍠', '⌻', '⍃', '⍄', '⍈', '⍐', '⍗', '⍇', '⍈', '⍌', '⍍', '⍏', '⍖'];

        for (i, ch) in text.char_indices() {
            if symbols.contains(&ch) {
                highlights.push((i, i + ch.len_utf8(), HighlightKind::Keyword));
            }
        }
        highlights
    }
}

impl Highlighter for AplHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        // Simple implementation: only highlight symbols
        self.highlight_symbols(text)
    }
}
