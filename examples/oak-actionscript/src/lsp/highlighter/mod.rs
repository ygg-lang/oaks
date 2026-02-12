#![doc = include_str!("readme.md")]

/// Local definition of highlight types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    /// Keyword.
    Keyword,
    /// String.
    String,
    /// Number.
    Number,
    /// Comment.
    Comment,
    /// Identifier.
    Identifier,
}

/// ActionScript syntax highlighter.
pub struct ActionScriptHighlighter {
    /// Whether to use parser-based highlighting.
    pub use_parser: bool,
}

impl Default for ActionScriptHighlighter {
    fn default() -> Self {
        Self { use_parser: false }
    }
}

impl ActionScriptHighlighter {
    /// Creates a new ActionScript highlighter instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Highlights the given text.
    pub fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        highlights.extend(self.highlight_keywords(text));
        highlights
    }

    fn highlight_keywords(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let keywords = [
            "as",
            "break",
            "case",
            "catch",
            "class",
            "const",
            "continue",
            "default",
            "delete",
            "do",
            "else",
            "extends",
            "false",
            "finally",
            "for",
            "function",
            "if",
            "implements",
            "import",
            "in",
            "instanceof",
            "interface",
            "internal",
            "is",
            "native",
            "new",
            "null",
            "package",
            "private",
            "protected",
            "public",
            "return",
            "set",
            "static",
            "super",
            "switch",
            "this",
            "throw",
            "true",
            "try",
            "typeof",
            "use",
            "var",
            "void",
            "while",
            "with",
            "dynamic",
            "final",
            "native",
            "override",
            "static",
            "abstract",
            "virtual",
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
