#![doc = include_str!("readme.md")]
//! Nginx syntax highlighter
//!
//! This module provides syntax highlighting for Nginx configuration files.

/// Highlight kinds for Nginx
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    /// A keyword (e.g., `server`, `location`).
    Keyword,
    /// A configuration directive.
    Directive,
    /// A variable.
    Variable,
    /// A comment.
    Comment,
    /// A string literal.
    String,
    /// A numeric literal.
    Number,
}

/// Highlighter trait for Nginx
pub trait Highlighter {
    /// Highlights the given text and returns a vector of spans and highlight kinds.
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

/// Nginx syntax highlighter
pub struct NginxHighlighter;

impl Default for NginxHighlighter {
    fn default() -> Self {
        Self
    }
}

impl NginxHighlighter {
    /// Creates a new Nginx highlighter instance
    pub fn new() -> Self {
        Self::default()
    }
}

impl Highlighter for NginxHighlighter {
    fn highlight(&self, _text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let highlights = Vec::new();
        highlights
    }
}
