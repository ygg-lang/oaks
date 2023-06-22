//! TOML 语法高亮器

/// 高亮类型的本地定义
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    /// 键
    Key,
    /// 字符串
    String,
    /// 数字
    Number,
    /// 布尔值
    Boolean,
    /// 注释
    Comment,
    /// 日期时间
    DateTime,
    /// 符号 (如 [ ] { } = .)
    Punctuation,
}

/// 高亮器 trait
pub trait Highlighter {
    /// 对给定的文本进行高亮处理
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

/// TOML 语法高亮器
pub struct TomlHighlighter {
    /// 是否使用基于解析器的高亮
    pub use_parser: bool,
}

impl Default for TomlHighlighter {
    fn default() -> Self {
        Self { use_parser: false }
    }
}

impl TomlHighlighter {
    pub fn new() -> Self {
        Self::default()
    }

    fn highlight_comments(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let mut start = 0;
        while let Some(pos) = text[start..].find('#') {
            let absolute_pos = start + pos;
            let end_of_line = text[absolute_pos..].find('\n').map(|i| absolute_pos + i).unwrap_or(text.len());
            highlights.push((absolute_pos, end_of_line, HighlightKind::Comment));
            start = end_of_line;
        }
        highlights
    }

    fn highlight_strings(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let mut chars = text.char_indices().peekable();

        while let Some((i, ch)) = chars.next() {
            if ch == '"' || ch == '\'' {
                let quote = ch;
                let start = i;
                let mut end = i + 1;
                let mut escaped = false;

                while let Some((j, next_ch)) = chars.next() {
                    end = j + next_ch.len_utf8();
                    if escaped {
                        escaped = false;
                    }
                    else if next_ch == '\\' && quote == '"' {
                        escaped = true;
                    }
                    else if next_ch == quote {
                        break;
                    }
                }
                highlights.push((start, end, HighlightKind::String));
            }
        }
        highlights
    }
}

impl Highlighter for TomlHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        highlights.extend(self.highlight_comments(text));
        highlights.extend(self.highlight_strings(text));
        // TODO: Add more highlighters for keys, numbers, booleans, etc.
        highlights.sort_by_key(|h| h.0);
        highlights
    }
}
