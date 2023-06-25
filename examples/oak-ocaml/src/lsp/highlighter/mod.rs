#![doc = include_str!("readme.md")]
/// Kind of highlighting to apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    /// A keyword.
    Keyword,
    /// A string literal.
    String,
    /// A numeric literal.
    Number,
    /// A comment.
    Comment,
    /// An identifier.
    Identifier,
}

/// A highlighter trait.
pub trait Highlighter {
    /// Highlights the given text.
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

/// A highlighter for the OCaml language.
pub struct OCamlHighlighter {
    /// Whether to use the parser for highlighting.
    pub use_parser: bool,
}

impl Default for OCamlHighlighter {
    fn default() -> Self {
        Self { use_parser: false }
    }
}

impl OCamlHighlighter {
    /// Creates a new OCaml highlighter.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new OCaml highlighter that uses the parser.
    pub fn with_parser() -> Self {
        Self { use_parser: true }
    }

    fn highlight_keywords(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let keywords = [
            "and",
            "as",
            "assert",
            "asr",
            "begin",
            "class",
            "constraint",
            "do",
            "done",
            "downto",
            "else",
            "end",
            "exception",
            "external",
            "false",
            "for",
            "fun",
            "function",
            "functor",
            "if",
            "in",
            "include",
            "inherit",
            "initializer",
            "land",
            "lazy",
            "let",
            "lor",
            "lsl",
            "lsr",
            "lxor",
            "match",
            "method",
            "mod",
            "module",
            "mutable",
            "new",
            "nonrec",
            "object",
            "of",
            "open",
            "or",
            "private",
            "rec",
            "sig",
            "struct",
            "then",
            "to",
            "true",
            "try",
            "type",
            "val",
            "virtual",
            "when",
            "while",
            "with",
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

impl Highlighter for OCamlHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = self.highlight_keywords(text);
        highlights.sort_by_key(|h| h.0);
        highlights
    }
}
