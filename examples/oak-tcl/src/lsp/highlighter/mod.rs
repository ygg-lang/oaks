#![doc = include_str!("readme.md")]
//! Tcl 语法高亮器

/// 高亮类型的本地定义
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    /// 关键字
    Keyword,
    /// 变量
    Variable,
    /// 字符串
    String,
    /// 数字
    Number,
    /// 注释
    Comment,
    /// 符号
    Punctuation,
}

/// 高亮器 trait
pub trait Highlighter {
    /// 对给定的文本进行高亮处理
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

/// Tcl 语法高亮器
pub struct TclHighlighter {
    /// 是否使用基于解析器的高亮
    pub use_parser: bool,
}

impl Default for TclHighlighter {
    fn default() -> Self {
        Self { use_parser: false }
    }
}

impl TclHighlighter {
    pub fn new() -> Self {
        Self::default()
    }

    fn highlight_keywords(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let keywords = ["set", "proc", "if", "else", "elseif", "for", "foreach", "while", "return", "break", "continue", "global", "upvar", "variable", "unset", "switch", "case", "default", "try", "catch", "finally", "throw"];

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

    fn highlight_comments(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let mut start = 0;
        while let Some(pos) = text[start..].find('#') {
            let absolute_pos = start + pos;
            let end_of_line = text[absolute_pos..].find('\n').map(|i| absolute_pos + i).unwrap_or(text.len());
            highlights.push((absolute_pos, end_of_line, HighlightKind::Comment));
            start = end_of_line
        }
        highlights
    }
}

impl Highlighter for TclHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        highlights.extend(self.highlight_keywords(text));
        highlights.extend(self.highlight_comments(text));
        highlights.sort_by_key(|h| h.0);
        highlights
    }
}
