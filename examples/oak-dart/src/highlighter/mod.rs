//! Dart 语法高亮器

/// 高亮类型的本地定义
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    /// 关键字
    Keyword,
    /// 字符串
    String,
    /// 数字
    Number,
    /// 注释
    Comment,
    /// 宏/注解
    Annotation,
    /// 标识符
    Identifier,
}

/// 高亮器 trait
pub trait Highlighter {
    /// 对给定的文本进行高亮处理
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

/// Dart 语法高亮器
pub struct DartHighlighter {
    /// 是否使用基于解析器的高亮
    pub use_parser: bool,
}

impl Default for DartHighlighter {
    fn default() -> Self {
        Self { use_parser: false }
    }
}

impl DartHighlighter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        // 简单的基于关键字的高亮实现
        let keywords = [
            "abstract",
            "as",
            "assert",
            "async",
            "await",
            "break",
            "case",
            "catch",
            "class",
            "const",
            "continue",
            "covariant",
            "default",
            "deferred",
            "do",
            "dynamic",
            "else",
            "enum",
            "export",
            "extends",
            "extension",
            "external",
            "factory",
            "false",
            "final",
            "finally",
            "for",
            "Function",
            "get",
            "hide",
            "if",
            "implements",
            "import",
            "in",
            "interface",
            "is",
            "late",
            "library",
            "mixin",
            "new",
            "null",
            "on",
            "operator",
            "part",
            "required",
            "rethrow",
            "return",
            "set",
            "show",
            "static",
            "super",
            "switch",
            "sync",
            "this",
            "throw",
            "true",
            "try",
            "typedef",
            "var",
            "void",
            "while",
            "with",
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
