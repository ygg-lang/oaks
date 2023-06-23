#![doc = include_str!("readme.md")]
//! Ada 语法高亮器

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

/// Ada 语法高亮器
pub struct AdaHighlighter {
    /// 是否使用基于解析器的高亮
    pub use_parser: bool,
}

impl Default for AdaHighlighter {
    fn default() -> Self {
        Self { use_parser: false }
    }
}

impl AdaHighlighter {
    /// 创建一个新的 Ada 高亮器实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 高亮 Ada 关键字
    fn highlight_keywords(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let keywords = [
            "abort",
            "else",
            "new",
            "return",
            "abs",
            "elsif",
            "not",
            "reverse",
            "abstract",
            "end",
            "null",
            "accept",
            "entry",
            "select",
            "access",
            "exception",
            "of",
            "separate",
            "aliased",
            "exit",
            "or",
            "some",
            "all",
            "others",
            "subtype",
            "and",
            "for",
            "out",
            "synchronized",
            "array",
            "function",
            "at",
            "overriding",
            "tagged",
            "generic",
            "package",
            "task",
            "begin",
            "goto",
            "pragma",
            "terminate",
            "body",
            "private",
            "then",
            "if",
            "procedure",
            "type",
            "case",
            "in",
            "protected",
            "constant",
            "interface",
            "until",
            "is",
            "raise",
            "use",
            "declare",
            "range",
            "delay",
            "limited",
            "record",
            "when",
            "delta",
            "loop",
            "rem",
            "while",
            "digits",
            "renames",
            "with",
            "do",
            "mod",
            "requeue",
            "xor",
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

    /// 高亮字符串字面量
    fn highlight_strings(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let mut chars = text.char_indices().peekable();

        while let Some((i, ch)) = chars.next() {
            if ch == '"' {
                let start = i;
                let mut end = i + 1;

                while let Some((j, next_ch)) = chars.next() {
                    end = j + next_ch.len_utf8();
                    if next_ch == '"' {
                        if let Some(&(_, peek_ch)) = chars.peek() {
                            if peek_ch == '"' {
                                chars.next(); // skip escaped quote
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

    /// 高亮数字字面量
    fn highlight_numbers(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let mut chars = text.char_indices().peekable();

        while let Some((i, ch)) = chars.next() {
            if ch.is_ascii_digit() {
                let start = i;
                let mut end = i + 1;

                while let Some(&(j, next_ch)) = chars.peek() {
                    if next_ch.is_ascii_digit() || next_ch == '.' || next_ch == '_' || next_ch == '#' || (next_ch >= 'a' && next_ch <= 'f') || (next_ch >= 'A' && next_ch <= 'F') {
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
        let mut pos = 0;

        for line in text.lines() {
            if let Some(comment_start) = line.find("--") {
                let start = pos + comment_start;
                let end = pos + line.len();
                highlights.push((start, end, HighlightKind::Comment))
            }
            pos += line.len() + 1
        }
        highlights
    }

    pub fn highlight_into(&self, text: &str, output: &mut Vec<(usize, usize, HighlightKind)>) {
        let mut highlights = Vec::new();
        highlights.extend(self.highlight_keywords(text));
        highlights.extend(self.highlight_strings(text));
        highlights.extend(self.highlight_numbers(text));
        highlights.extend(self.highlight_comments(text));
        highlights.sort_by_key(|&(start, _, _)| start);
        output.extend(highlights);
    }
}

impl Highlighter for AdaHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        self.highlight_into(text, &mut highlights);
        highlights
    }
}
