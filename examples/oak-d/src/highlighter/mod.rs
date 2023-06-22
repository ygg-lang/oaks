/// Highlight kinds for D language
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    /// Keywords
    Keyword,
    /// Strings
    String,
    /// Numbers
    Number,
    /// Comments
    Comment,
    /// Identifiers
    Identifier,
}

/// Highlighter trait
pub trait Highlighter {
    /// Highlight the given text
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

/// D language syntax highlighter
pub struct DHighlighter;

impl DHighlighter {
    /// Create a new D highlighter
    pub fn new() -> Self {
        Self
    }
}

impl Highlighter for DHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        // Basic keyword highlighting for D
        let mut highlights = Vec::new();
        let keywords = [
            "module",
            "import",
            "public",
            "private",
            "protected",
            "package",
            "export",
            "static",
            "final",
            "abstract",
            "override",
            "synchronized",
            "const",
            "immutable",
            "inout",
            "shared",
            "class",
            "struct",
            "interface",
            "union",
            "enum",
            "function",
            "delegate",
            "if",
            "else",
            "while",
            "for",
            "foreach",
            "do",
            "switch",
            "case",
            "default",
            "break",
            "continue",
            "return",
            "goto",
            "try",
            "catch",
            "finally",
            "throw",
            "assert",
            "new",
            "delete",
            "this",
            "super",
            "void",
            "bool",
            "byte",
            "ubyte",
            "short",
            "ushort",
            "int",
            "uint",
            "long",
            "ulong",
            "float",
            "double",
            "real",
            "char",
            "wchar",
            "dchar",
        ];

        for keyword in keywords {
            let mut start = 0;
            while let Some(pos) = text[start..].find(keyword) {
                let abs_pos = start + pos;
                let end_pos = abs_pos + keyword.len();

                // Check word boundaries
                let before = abs_pos == 0 || !text.chars().nth(abs_pos - 1).unwrap_or(' ').is_alphanumeric();
                let after = end_pos >= text.len() || !text.chars().nth(end_pos).unwrap_or(' ').is_alphanumeric();

                if before && after {
                    highlights.push((abs_pos, end_pos, HighlightKind::Keyword));
                }
                start = abs_pos + 1;
            }
        }

        highlights
    }
}
