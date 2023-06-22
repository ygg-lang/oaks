/// Delphi 高亮类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    Keyword,
    String,
    Number,
    Comment,
    Identifier,
}

/// Delphi 语法高亮器
pub struct DelphiHighlighter;

impl Default for DelphiHighlighter {
    fn default() -> Self {
        Self
    }
}

impl DelphiHighlighter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        // 简单实现：仅匹配关键字
        let keywords = ["program", "unit", "interface", "implementation", "begin", "end", "var", "type", "procedure", "function", "if", "then", "else", "for", "to", "do", "while", "repeat", "until"];

        for keyword in keywords {
            let mut start = 0;
            while let Some(pos) = text[start..].find(keyword) {
                let abs_pos = start + pos;
                let end_pos = abs_pos + keyword.len();
                highlights.push((abs_pos, end_pos, HighlightKind::Keyword));
                start = end_pos;
            }
        }

        highlights
    }
}
