#![doc = include_str!("readme.md")]
//! LLVM IR syntax highlighter

/// Local definition of highlight kinds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    /// Keyword
    Keyword,
    /// String
    String,
    /// Number
    Number,
    /// Comment
    Comment,
    /// Local variable
    LocalVar,
    /// Global variable
    GlobalVar,
    /// Metadata
    Metadata,
    /// Identifier
    Identifier,
}

/// Highlighter trait
pub trait Highlighter {
    /// Highlights the given text
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

/// Highlighter implementation for LLVM IR.
pub struct LLirHighlighter;

impl LLirHighlighter {
    /// Creates a new `LLirHighlighter`.
    pub fn new() -> Self {
        Self
    }
}

impl Highlighter for LLirHighlighter {
    fn highlight(&self, _text: &str) -> Vec<(usize, usize, HighlightKind)> {
        // TODO: Implement highlighting
        Vec::new()
    }
}
