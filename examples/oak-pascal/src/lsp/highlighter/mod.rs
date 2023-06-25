#![doc = include_str!("readme.md")]
/// Types of elements that can be highlighted.
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
    /// Highlights the given text.
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

/// Syntax highlighter for Pascal.
pub struct PascalHighlighter {
    /// Whether to use the parser for highlighting.
    pub use_parser: bool,
}

impl Default for PascalHighlighter {
    fn default() -> Self {
        Self { use_parser: false }
    }
}

impl PascalHighlighter {
    /// Creates a new Pascal highlighter.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new Pascal highlighter that uses the parser.
    pub fn with_parser() -> Self {
        Self { use_parser: true }
    }

    fn highlight_keywords(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let keywords = [
            "and",
            "array",
            "as",
            "asm",
            "begin",
            "case",
            "class",
            "const",
            "constructor",
            "destructor",
            "dispinterface",
            "div",
            "do",
            "downto",
            "else",
            "end",
            "except",
            "exports",
            "file",
            "finalization",
            "finally",
            "for",
            "function",
            "goto",
            "if",
            "implementation",
            "in",
            "inherited",
            "initialization",
            "inline",
            "interface",
            "is",
            "label",
            "library",
            "mod",
            "nil",
            "not",
            "object",
            "of",
            "or",
            "out",
            "packed",
            "procedure",
            "program",
            "property",
            "raise",
            "record",
            "repeat",
            "resourcestring",
            "set",
            "shl",
            "shr",
            "string",
            "then",
            "threadvar",
            "to",
            "try",
            "type",
            "unit",
            "until",
            "uses",
            "var",
            "while",
            "with",
            "xor",
        ];

        for keyword in &keywords {
            let mut start = 0;
            while let Some(pos) = text[start..].to_lowercase().find(&keyword.to_lowercase()) {
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

impl Highlighter for PascalHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = self.highlight_keywords(text);
        highlights.sort_by_key(|h| h.0);
        highlights
    }
}
