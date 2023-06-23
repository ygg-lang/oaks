#![doc = include_str!("readme.md")]
//! Rust 语法高亮器
//!
//! 这个模块提供了 Rust 源代码的语法高亮功能，支持关键字、字符串、数字、注释等的高亮显示。

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
    /// 宏
    Macro,
    /// 标识符
    Identifier,
}

/// 高亮器 trait
pub trait Highlighter {
    /// 对给定的文本进行高亮处理
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

/// Rust 语法高亮器
///
/// `RustHighlighter` 实现了 `Highlighter` trait，为 Rust 代码提供语法高亮功能。
pub struct RustHighlighter {
    /// 是否使用基于解析器的高亮以提高准确性
    pub use_parser: bool,
}

impl Default for RustHighlighter {
    fn default() -> Self {
        Self { use_parser: false }
    }
}

impl RustHighlighter {
    /// 创建一个新的 Rust 高亮器实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 创建一个使用解析器的高亮器实例
    pub fn with_parser() -> Self {
        Self { use_parser: true }
    }

    /// 高亮 Rust 关键字
    fn highlight_keywords(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let keywords = [
            "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self", "Self", "static", "struct", "super", "trait",
            "true", "type", "unsafe", "use", "where", "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro", "override", "priv", "typeof", "unsized", "virtual", "yield", "try", "union", "raw",
        ];

        for keyword in &keywords {
            let mut start = 0;
            while let Some(pos) = text[start..].find(keyword) {
                let absolute_pos = start + pos;
                let end_pos = absolute_pos + keyword.len();

                // 确保这是一个完整的单词
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

    /// 高亮字符串字面量
    fn highlight_strings(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let mut chars = text.char_indices().peekable();

        while let Some((i, ch)) = chars.next() {
            match ch {
                '"' => {
                    let start = i;
                    let mut end = i + 1;
                    let mut escaped = false;

                    while let Some((j, next_ch)) = chars.next() {
                        end = j + next_ch.len_utf8();
                        if escaped {
                            escaped = false;
                        }
                        else if next_ch == '\\' {
                            escaped = true;
                        }
                        else if next_ch == '"' {
                            break;
                        }
                    }

                    highlights.push((start, end, HighlightKind::String))
                }
                '\'' => {
                    let start = i;
                    let mut end = i + 1;
                    let mut escaped = false;

                    while let Some((j, next_ch)) = chars.next() {
                        end = j + next_ch.len_utf8();
                        if escaped {
                            escaped = false;
                        }
                        else if next_ch == '\\' {
                            escaped = true;
                        }
                        else if next_ch == '\'' {
                            break;
                        }
                    }

                    highlights.push((start, end, HighlightKind::String))
                }
                _ => {}
            }
        }

        highlights
    }

    /// 高亮数字字面量
    fn highlight_numbers(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let mut chars = text.char_indices().peekable();

        while let Some((i, ch)) = chars.next() {
            if ch.is_ascii_digit() {
                let start = i;
                let mut end = i + 1;

                // 继续读取数字字符
                while let Some(&(j, next_ch)) = chars.peek() {
                    if next_ch.is_ascii_digit() || next_ch == '.' || next_ch == '_' {
                        end = j + next_ch.len_utf8();
                        chars.next();
                    }
                    else {
                        break;
                    }
                }

                highlights.push((start, end, HighlightKind::Number))
            }
        }

        highlights
    }

    /// 高亮注释
    fn highlight_comments(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let lines: Vec<&str> = text.lines().collect();
        let mut pos = 0;

        for line in lines {
            if let Some(comment_start) = line.find("//") {
                let start = pos + comment_start;
                let end = pos + line.len();
                highlights.push((start, end, HighlightKind::Comment))
            }
            pos += line.len() + 1; // +1 for newline
        }

        highlights
    }
}

impl Highlighter for RustHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();

        highlights.extend(self.highlight_keywords(text));
        highlights.extend(self.highlight_strings(text));
        highlights.extend(self.highlight_numbers(text));
        highlights.extend(self.highlight_comments(text));

        // 按位置排序
        highlights.sort_by_key(|&(start, _, _)| start);
        highlights
    }
}
