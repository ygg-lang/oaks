#![doc = include_str!("readme.md")]
/// Kind of highlight.
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

/// Highlighter trait.
pub trait Highlighter {
    /// Highlights the given text.
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

/// Objective-C syntax highlighter.
pub struct ObjectiveCHighlighter {
    /// Whether to use the parser for highlighting.
    pub use_parser: bool,
}

impl Default for ObjectiveCHighlighter {
    fn default() -> Self {
        Self { use_parser: false }
    }
}

impl ObjectiveCHighlighter {
    /// Creates a new Objective-C highlighter.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new Objective-C highlighter that uses the parser.
    pub fn with_parser() -> Self {
        Self { use_parser: true }
    }

    fn highlight_keywords(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let keywords = [
            "auto",
            "break",
            "case",
            "char",
            "const",
            "continue",
            "default",
            "do",
            "double",
            "else",
            "enum",
            "extern",
            "float",
            "for",
            "goto",
            "if",
            "inline",
            "int",
            "long",
            "register",
            "restrict",
            "return",
            "short",
            "signed",
            "sizeof",
            "static",
            "struct",
            "switch",
            "typedef",
            "union",
            "unsigned",
            "void",
            "volatile",
            "while",
            "@interface",
            "@implementation",
            "@protocol",
            "@end",
            "@property",
            "@synthesize",
            "@dynamic",
            "@public",
            "@private",
            "@protected",
            "@package",
            "@class",
            "@selector",
            "@encode",
            "@defs",
            "@synchronized",
            "@try",
            "@throw",
            "@catch",
            "@finally",
            "@optional",
            "@required",
            "@autoreleasepool",
            "@import",
            "id",
            "Class",
            "SEL",
            "IMP",
            "BOOL",
            "YES",
            "NO",
            "nil",
            "Nil",
            "self",
            "super",
        ];

        for keyword in &keywords {
            let mut start = 0;
            while let Some(pos) = text[start..].find(keyword) {
                let absolute_pos = start + pos;
                let end_pos = absolute_pos + keyword.len();

                let is_word_boundary_before = absolute_pos == 0 || !text.chars().nth(absolute_pos - 1).unwrap_or(' ').is_alphanumeric() && text.chars().nth(absolute_pos - 1).unwrap_or(' ') != '@';
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

impl Highlighter for ObjectiveCHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = self.highlight_keywords(text);
        highlights.sort_by_key(|h| h.0);
        highlights
    }
}
