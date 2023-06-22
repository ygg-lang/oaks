//! Notedown syntax highlighter

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    Keyword,
    String,
    Number,
    Comment,
    Identifier,
    Heading,
    CodeBlock,
}

/// 高亮器 trait
pub trait Highlighter {
    /// 对给定的文本进行高亮处理
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

pub struct NoteHighlighter {
    pub use_parser: bool,
}

impl Default for NoteHighlighter {
    fn default() -> Self {
        Self { use_parser: false }
    }
}

impl NoteHighlighter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_parser() -> Self {
        Self { use_parser: true }
    }

    fn highlight_patterns(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();

        // Simple heading detection
        for line in text.lines() {
            let line_start = text.find(line).unwrap_or(0);
            if line.starts_with('#') {
                highlights.push((line_start, line_start + line.len(), HighlightKind::Heading));
            }
        }

        highlights
    }
}

impl Highlighter for NoteHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = self.highlight_patterns(text);
        highlights.sort_by_key(|h| h.0);
        highlights
    }
}
