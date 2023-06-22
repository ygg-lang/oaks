//! Org-mode 语法高亮器

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    Keyword,
    String,
    Number,
    Comment,
    Identifier,
    Heading,
    Property,
}

/// 高亮器 trait
pub trait Highlighter {
    /// 对给定的文本进行高亮处理
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

/// Org-mode 语法高亮器
pub struct OrgModeHighlighter {
    pub use_parser: bool,
}

impl Default for OrgModeHighlighter {
    fn default() -> Self {
        Self { use_parser: false }
    }
}

impl OrgModeHighlighter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_parser() -> Self {
        Self { use_parser: true }
    }

    fn highlight_patterns(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();

        for (_i, line) in text.lines().enumerate() {
            let line_start = text.find(line).unwrap_or(0);
            if line.starts_with('*') {
                highlights.push((line_start, line_start + line.len(), HighlightKind::Heading));
            }
            else if line.starts_with('#') {
                highlights.push((line_start, line_start + line.len(), HighlightKind::Comment));
            }
        }

        highlights
    }
}

impl Highlighter for OrgModeHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = self.highlight_patterns(text);
        highlights.sort_by_key(|h| h.0);
        highlights
    }
}
