#![doc = include_str!("readme.md")]
//! PureScript syntax highlighter

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

pub struct PurescriptHighlighter {
    pub use_parser: bool,
}

impl Default for PurescriptHighlighter {
    fn default() -> Self {
        Self { use_parser: false }
    }
}

impl PurescriptHighlighter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_parser() -> Self {
        Self { use_parser: true }
    }

    fn highlight_keywords(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let keywords = [
            "ado", "as", "case", "class", "data", "derive", "do", "else", "false", "forall", "foreign", "hiding", "if", "import", "in", "infix", "infixl", "infixr", "instance", "let", "module", "newtype", "nominal", "of", "role", "then", "true", "type",
            "where",
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
    fn highlight_strings(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let mut chars = text.char_indices().peekable();

        while let Some((i, ch)) = chars.next() {
            match ch {
                '"' => {
                    let start = i;
                    let mut escaped = false;
                    let mut found_end = false;

                    while let Some((j, next_ch)) = chars.next() {
                        if escaped {
                            escaped = false
                        }
                        else if next_ch == '\\' {
                            escaped = true
                        }
                        else if next_ch == '"' {
                            highlights.push((start, j + 1, HighlightKind::String));
                            found_end = true;
                            break;
                        }
                    }
                    if !found_end {
                        highlights.push((start, text.len(), HighlightKind::String))
                    }
                }
                _ => {}
            }
        }
        highlights
    }

    fn highlight_numbers(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let mut start = None;

        for (i, ch) in text.char_indices() {
            if ch.is_ascii_digit() {
                if start.is_none() {
                    start = Some(i)
                }
            }
            else {
                if let Some(s) = start {
                    highlights.push((s, i, HighlightKind::Number));
                    start = None
                }
            }
        }
        if let Some(s) = start {
            highlights.push((s, text.len(), HighlightKind::Number))
        }
        highlights
    }

    fn highlight_comments(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let mut start = 0;
        while let Some(pos) = text[start..].find("--") {
            let absolute_pos = start + pos;
            let end_pos = text[absolute_pos..].find('\n').map(|n| absolute_pos + n).unwrap_or(text.len());
            highlights.push((absolute_pos, end_pos, HighlightKind::Comment));
            start = end_pos
        }
        // 块注释
        let mut start = 0;
        while let Some(pos) = text[start..].find("{-") {
            let absolute_pos = start + pos;
            let end_pos = text[absolute_pos..].find("-}").map(|n| absolute_pos + n + 2).unwrap_or(text.len());
            highlights.push((absolute_pos, end_pos, HighlightKind::Comment));
            start = end_pos
        }
        highlights
    }
}

impl Highlighter for PurescriptHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = self.highlight_keywords(text);
        highlights.extend(self.highlight_strings(text));
        highlights.extend(self.highlight_numbers(text));
        highlights.extend(self.highlight_comments(text));
        highlights.sort_by_key(|h| h.0);
        highlights
    }
}
