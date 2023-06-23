#![doc = include_str!("readme.md")]
//! Nginx syntax highlighter
//!
//! This module provides syntax highlighting for Nginx configuration files.

/// Highlight kinds for Nginx
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    Keyword,
    Directive,
    Variable,
    Comment,
    String,
    Number,
}

/// Highlighter trait for Nginx
pub trait Highlighter {
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
