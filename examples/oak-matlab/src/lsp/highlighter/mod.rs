#![doc = include_str!("readme.md")]
//! MATLAB 语法高亮器
//!
//! 这个模块提供了 MATLAB 源代码的语法高亮功能，支持关键字、字符串、数字、注释等的高亮显示。

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
    /// 标识符
    Identifier,
}

/// 高亮器 trait
pub trait Highlighter {
    /// 对给定的文本进行高亮处理
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

/// MATLAB 语法高亮器
///
/// `MatlabHighlighter` 实现了 `Highlighter` trait，为 MATLAB 代码提供语法高亮功能。
pub struct MatlabHighlighter;

impl Default for MatlabHighlighter {
    fn default() -> Self {
        Self
    }
}

impl MatlabHighlighter {
    /// 创建一个新的 MATLAB 高亮器实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 高亮 MATLAB 关键字
    fn highlight_keywords(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let keywords = ["break", "case", "catch", "classdef", "continue", "else", "elseif", "end", "for", "function", "global", "if", "otherwise", "parfor", "persistent", "return", "spmd", "switch", "try", "while"];

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

    /// 高亮字符串字面量
    fn highlight_strings(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let mut chars = text.char_indices().peekable();

        while let Some((i, ch)) = chars.next() {
            if ch == '\'' {
                let start = i;
                let mut end = i + 1;
                while let Some((j, next_ch)) = chars.next() {
                    end = j + next_ch.len_utf8();
                    if next_ch == '\'' {
                        // 检查是否是双引号转义
                        if let Some(&(_, peek_ch)) = chars.peek() {
                            if peek_ch == '\'' {
                                chars.next();
                                continue;
                            }
                        }
                        break;
                    }
                }
                highlights.push((start, end, HighlightKind::String))
            }
        }
        highlights
    }
}

impl Highlighter for MatlabHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();

        highlights.extend(self.highlight_keywords(text));
        highlights.extend(self.highlight_strings(text));

        // 按位置排序
        highlights.sort_by_key(|&(start, _, _)| start);
        highlights
    }
}
