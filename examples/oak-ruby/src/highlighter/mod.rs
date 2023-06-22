//! Ruby 语法高亮器
//!
//! 这个模块提供了 Ruby 源代码的语法高亮功能。

/// Ruby 语法高亮器
pub struct RubyHighlighter {
    /// 是否使用基于解析器的高亮以提高准确性
    pub use_parser: bool,
}

impl Default for RubyHighlighter {
    fn default() -> Self {
        Self { use_parser: false }
    }
}

impl RubyHighlighter {
    /// 创建一个新的 Ruby 高亮器实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 高亮 Ruby 关键字
    pub fn highlight(&self, text: &str) -> Vec<(usize, usize, String)> {
        let mut highlights = Vec::new();
        let keywords = [
            "if", "unless", "elsif", "else", "case", "when", "then", "for", "while", "until", "break", "next", "redo", "retry", "return", "yield", "def", "class", "module", "end", "lambda", "proc", "begin", "rescue", "ensure", "raise", "require", "load",
            "include", "extend", "prepend", "and", "or", "not", "in", "true", "false", "nil", "super", "self", "alias", "undef", "defined", "do",
        ];

        for keyword in &keywords {
            let mut start = 0;
            while let Some(pos) = text[start..].find(keyword) {
                let absolute_pos = start + pos;
                let end_pos = absolute_pos + keyword.len();

                let is_word_boundary_before = absolute_pos == 0 || !text.chars().nth(absolute_pos - 1).unwrap_or(' ').is_alphanumeric();
                let is_word_boundary_after = end_pos >= text.len() || !text.chars().nth(end_pos).unwrap_or(' ').is_alphanumeric();

                if is_word_boundary_before && is_word_boundary_after {
                    highlights.push((absolute_pos, end_pos, "keyword".to_string()));
                }
                start = absolute_pos + 1;
            }
        }
        highlights
    }
}
