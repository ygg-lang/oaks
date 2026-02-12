#![doc = include_str!("readme.md")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    Keyword,
    String,
    Number,
    Comment,
    Identifier,
}

/// Highlighter trait.
pub trait Highlighter {
    /// Highlights the given text.
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

pub struct KotlinHighlighter;

impl Highlighter for KotlinHighlighter {
    fn highlight(&self, _text: &str) -> Vec<(usize, usize, HighlightKind)> {
        // TODO: Implement actual highlighting
        Vec::new()
    }
}
