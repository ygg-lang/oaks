#![doc = include_str!("readme.md")]
/// Delphi highlight kinds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    /// Keyword
    Keyword,
    /// String
    String,
    /// Number
    Number,
    /// Comment
    Comment,
    /// Identifier
    Identifier,
}

/// Delphi syntax highlighter
pub struct DelphiHighlighter;

impl Default for DelphiHighlighter {
    fn default() -> Self {
        Self
    }
}

impl DelphiHighlighter {
    /// Creates a new `DelphiHighlighter`
    pub fn new() -> Self {
        Self::default()
    }

    /// Highlights the text
    pub fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        // Simple implementation: match keywords only
        let keywords = ["program", "unit", "interface", "implementation", "begin", "end", "var", "type", "procedure", "function", "if", "then", "else", "for", "to", "do", "while", "repeat", "until"];

        for keyword in keywords {
            let mut start = 0;
            while let Some(pos) = text[start..].find(keyword) {
                let abs_pos = start + pos;
                let end_pos = abs_pos + keyword.len();
                highlights.push((abs_pos, end_pos, HighlightKind::Keyword));
                start = end_pos
            }
        }

        highlights
    }
}
