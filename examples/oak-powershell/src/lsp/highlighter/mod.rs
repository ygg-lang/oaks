#![doc = include_str!("readme.md")]
//! PowerShell highlighter
/// Represents the kind of highlighting for a syntax element.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    /// A keyword.
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

/// A highlighter for the PowerShell language.
pub struct PowerShellHighlighter {
    /// Whether to use the parser for highlighting.
    pub use_parser: bool,
}

impl Default for PowerShellHighlighter {
    fn default() -> Self {
        Self { use_parser: false }
    }
}

impl PowerShellHighlighter {
    /// Creates a new `PowerShellHighlighter`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new `PowerShellHighlighter` that uses the parser.
    pub fn with_parser() -> Self {
        Self { use_parser: true }
    }

    fn highlight_keywords(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let keywords = [
            "begin",
            "break",
            "catch",
            "class",
            "continue",
            "data",
            "define",
            "do",
            "dynamicparam",
            "else",
            "elseif",
            "end",
            "exit",
            "filter",
            "finally",
            "for",
            "foreach",
            "from",
            "function",
            "if",
            "in",
            "inline",
            "parallel",
            "param",
            "process",
            "return",
            "switch",
            "throw",
            "trap",
            "try",
            "until",
            "using",
            "var",
            "while",
            "workflow",
        ];

        for keyword in &keywords {
            let mut start = 0;
            while let Some(pos) = text[start..].to_lowercase().find(&keyword.to_lowercase()) {
                let absolute_pos = start + pos;
                let end_pos = absolute_pos + keyword.len();

                let is_word_boundary_before = absolute_pos == 0 || !text.chars().nth(absolute_pos - 1).unwrap_or(' ').is_alphanumeric();
                let is_word_boundary_after = end_pos >= text.len() || !text.chars().nth(end_pos).unwrap_or(' ').is_alphanumeric();

                if is_word_boundary_before && is_word_boundary_after {
                    highlights.push((absolute_pos, end_pos, HighlightKind::Keyword));
                }

                start = absolute_pos + 1;
            }
        }

        highlights
    }
}

impl Highlighter for PowerShellHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = self.highlight_keywords(text);
        highlights.sort_by_key(|h| h.0);
        highlights
    }
}
