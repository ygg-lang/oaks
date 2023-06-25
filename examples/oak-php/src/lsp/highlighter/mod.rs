#![doc = include_str!("readme.md")]
/// The kind of highlighting for a range of text.
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

/// Highlighter trait
pub trait Highlighter {
    /// Highlights the given text
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

/// A highlighter for PHP code.
pub struct PhpHighlighter {
    /// Whether to use the parser for highlighting.
    pub use_parser: bool,
}

impl Default for PhpHighlighter {
    fn default() -> Self {
        Self { use_parser: false }
    }
}

impl PhpHighlighter {
    /// Creates a new `PhpHighlighter`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new `PhpHighlighter` that uses the parser.
    pub fn with_parser() -> Self {
        Self { use_parser: true }
    }

    fn highlight_keywords(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let keywords = [
            "abstract",
            "and",
            "array",
            "as",
            "break",
            "callable",
            "case",
            "catch",
            "class",
            "clone",
            "const",
            "continue",
            "declare",
            "default",
            "die",
            "do",
            "echo",
            "else",
            "elseif",
            "empty",
            "enddeclare",
            "endfor",
            "endforeach",
            "endif",
            "endswitch",
            "endwhile",
            "eval",
            "exit",
            "extends",
            "final",
            "finally",
            "for",
            "foreach",
            "function",
            "global",
            "goto",
            "if",
            "implements",
            "include",
            "include_once",
            "instanceof",
            "insteadof",
            "interface",
            "isset",
            "list",
            "namespace",
            "new",
            "or",
            "print",
            "private",
            "protected",
            "public",
            "require",
            "require_once",
            "return",
            "static",
            "switch",
            "throw",
            "trait",
            "try",
            "unset",
            "use",
            "var",
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
                    highlights.push((absolute_pos, end_pos, HighlightKind::Keyword));
                }

                start = absolute_pos + 1;
            }
        }

        highlights
    }
}

impl Highlighter for PhpHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = self.highlight_keywords(text);
        highlights.sort_by_key(|h| h.0);
        highlights
    }
}
