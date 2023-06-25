#![doc = include_str!("readme.md")]
//! Nim syntax highlighter

/// Local definition of highlight types
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
    /// Type
    Type,
    /// Function
    Function,
    /// Identifier
    Identifier,
}

/// Nim syntax highlighter
pub struct NimHighlighter {
    /// Whether to use the parser for more precise highlighting
    pub use_parser: bool,
}

/// Highlighter trait
pub trait Highlighter {
    /// Highlights the given text
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

impl Default for NimHighlighter {
    fn default() -> Self {
        Self { use_parser: false }
    }
}

impl NimHighlighter {
    /// Creates a new Nim highlighter
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a Nim highlighter that uses the parser
    pub fn with_parser() -> Self {
        Self { use_parser: true }
    }

    /// Highlight Nim keywords
    fn highlight_keywords(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let keywords = [
            "addr",
            "and",
            "as",
            "asm",
            "bind",
            "block",
            "break",
            "case",
            "cast",
            "concept",
            "const",
            "continue",
            "converter",
            "defer",
            "discard",
            "distinct",
            "div",
            "do",
            "elif",
            "else",
            "end",
            "enum",
            "except",
            "export",
            "finally",
            "for",
            "from",
            "func",
            "if",
            "import",
            "in",
            "is",
            "isnot",
            "iterator",
            "let",
            "macro",
            "method",
            "mixin",
            "mod",
            "nil",
            "not",
            "notin",
            "object",
            "of",
            "or",
            "out",
            "proc",
            "ptr",
            "raise",
            "ref",
            "return",
            "shl",
            "shr",
            "static",
            "template",
            "try",
            "type",
            "using",
            "var",
            "when",
            "while",
            "xor",
            "yield",
        ];

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
}

impl Highlighter for NimHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = self.highlight_keywords(text);
        highlights.sort_by_key(|h| h.0);
        highlights
    }
}
