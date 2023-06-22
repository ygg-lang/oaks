#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    Keyword,
    String,
    Number,
    Comment,
    Identifier,
}

/// 高亮器 trait
pub trait Highlighter {
    /// 对给定的文本进行高亮处理
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

/// Pascal 语法高亮器
pub struct PascalHighlighter {
    pub use_parser: bool,
}

impl Default for PascalHighlighter {
    fn default() -> Self {
        Self { use_parser: false }
    }
}

impl PascalHighlighter {
    pub fn new() -> Self {
        Self::default()
    }

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
                    highlights.push((absolute_pos, end_pos, HighlightKind::Keyword));
                }

                start = absolute_pos + 1;
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
