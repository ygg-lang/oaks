#![doc = include_str!("readme.md")]
//! Tcl syntax highlighter

/// Local definition of highlight kinds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    /// Keyword
    Keyword,
    /// Variable
    Variable,
    /// String
    String,
    /// Number
    Number,
    /// Comment
    Comment,
    /// Punctuation
    Punctuation,
}

/// Highlighter trait
pub trait Highlighter {
    /// Highlights the given text
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

/// Tcl syntax highlighter
pub struct TclHighlighter {
    /// Whether to use parser-based highlighting
    pub use_parser: bool,
}

impl Default for TclHighlighter {
    fn default() -> Self {
        Self { use_parser: false }
    }
}

impl TclHighlighter {
    pub fn new() -> Self {
        Self::default()
    }

    fn highlight_keywords(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let keywords = ["set", "proc", "if", "else", "elseif", "for", "foreach", "while", "return", "break", "continue", "global", "upvar", "variable", "unset", "switch", "case", "default", "try", "catch", "finally", "throw"];

        for keyword in &keywords {
            let mut start = 0;
            while let Some(pos) = text[start..].find(keyword) {
                let absolute_pos = start + pos;
                let end_pos = absolute_pos + keyword.len();

                let is_word_boundary_before = absolute_pos == 0 || !text.chars().nth(absolute_pos - 1).unwrap_or(' ').is_alphanumeric();
                let is_word_boundary_after = end_pos >= text.len() || !text.chars().nth(end_pos).unwrap_or(' ').is_alphanumeric();

                if is_word_boundary_before && is_word_boundary_after {
                    highlights.push((absolute_pos, end_pos, HighlightKind::Keyword))
                }
                start = absolute_pos + 1
            }
        }
        highlights
    }

    fn highlight_comments(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let mut start = 0;
        while let Some(pos) = text[start..].find('#') {
            let absolute_pos = start + pos;
            let end_of_line = text[absolute_pos..].find('\n').map(|i| absolute_pos + i).unwrap_or(text.len());
            highlights.push((absolute_pos, end_of_line, HighlightKind::Comment));
            start = end_of_line
        }
        highlights
    }
}

impl Highlighter for TclHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        highlights.extend(self.highlight_keywords(text));
        highlights.extend(self.highlight_comments(text));
        highlights.sort_by_key(|h| h.0);
        highlights
    }
}
