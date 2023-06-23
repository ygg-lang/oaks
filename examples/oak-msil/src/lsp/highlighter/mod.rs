#![doc = include_str!("readme.md")]
//! MSIL 语法高亮器
//!
//! 这个模块提供了 MSIL 源代码的语法高亮功能，支持关键字、指令、指令码、注释等的高亮显示。

/// 高亮类型的本地定义
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    /// 关键字 (如 public, private, static)
    Keyword,
    /// 指令 (以 . 开头的，如 .assembly, .class, .method)
    Directive,
    /// 指令码 (如 ldstr, call, ret)
    Instruction,
    /// 字符串
    String,
    /// 数字
    Number,
    /// 注释
    Comment,
    /// 标识符
    Identifier,
}

/// 高亮器 trait
pub trait Highlighter {
    /// 对给定的文本进行高亮处理
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

/// MSIL 语法高亮器
pub struct MsilHighlighter;

impl MsilHighlighter {
    /// 创建一个新的 MSIL 高亮器实例
    pub fn new() -> Self {
        Self
    }

    /// 高亮 MSIL 关键字
    fn highlight_keywords(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let keywords = ["public", "private", "static", "hidebysig", "cil", "managed", "instance", "void", "extends", "implements"];

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

    /// 高亮指令 (以 . 开头)
    fn highlight_directives(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let mut chars = text.char_indices().peekable();

        while let Some((i, ch)) = chars.next() {
            if ch == '.' {
                let start = i;
                let mut end = i + 1;
                while let Some(&(j, next_ch)) = chars.peek() {
                    if next_ch.is_alphanumeric() || next_ch == '_' {
                        end = j + next_ch.len_utf8();
                        chars.next();
                    }
                    else {
                        break;
                    }
                }
                highlights.push((start, end, HighlightKind::Directive))
            }
        }
        highlights
    }
}

impl Default for MsilHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

impl Highlighter for MsilHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();

        highlights.extend(self.highlight_keywords(text));
        highlights.extend(self.highlight_directives(text));

        // 按位置排序
        highlights.sort_by_key(|&(start, _, _)| start);
        highlights
    }
}
