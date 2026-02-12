#![doc = include_str!("readme.md")]
/// Represents the type of a highlighted segment in the source code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    /// A language keyword.
    Keyword,
    /// A string literal.
    String,
    /// A numeric literal.
    Number,
    /// A source code comment.
    Comment,
    /// A name identifier.
    Identifier,
}

/// A trait for highlighting source code.
pub trait Highlighter {
    /// Performs highlighting on the given text and returns a list of highlighted segments.
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

/// Highlighter implementation for the Protobuf language.
pub struct ProtobufHighlighter {
    /// Whether to use the parser for more accurate highlighting.
    pub use_parser: bool,
}

impl Default for ProtobufHighlighter {
    fn default() -> Self {
        Self { use_parser: false }
    }
}

impl ProtobufHighlighter {
    /// Creates a new `ProtobufHighlighter` with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new `ProtobufHighlighter` that uses the parser for highlighting.
    pub fn with_parser() -> Self {
        Self { use_parser: true }
    }

    fn highlight_keywords(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let keywords = [
            "syntax",
            "package",
            "import",
            "option",
            "message",
            "enum",
            "service",
            "rpc",
            "returns",
            "repeated",
            "optional",
            "required",
            "oneof",
            "map",
            "reserved",
            "extend",
            "extensions",
            "group",
            "double",
            "float",
            "int32",
            "int64",
            "uint32",
            "uint64",
            "sint32",
            "sint64",
            "fixed32",
            "fixed64",
            "sfixed32",
            "sfixed64",
            "bool",
            "string",
            "bytes",
            "default",
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

impl Highlighter for ProtobufHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = self.highlight_keywords(text);
        highlights.sort_by_key(|h| h.0);
        highlights
    }
}
