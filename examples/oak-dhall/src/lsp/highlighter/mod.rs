#![doc = include_str!("readme.md")]

/// Highlight kinds for DHall.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    /// Keywords.
    Keyword,
    /// String literals.
    String,
    /// Number literals.
    Number,
    /// Comments.
    Comment,
    /// Identifiers.
    Identifier,
}

/// Highlighter implementation for DHall.
pub struct DHallHighlighter {}

impl Default for DHallHighlighter {
    fn default() -> Self {
        Self {}
    }
}

impl DHallHighlighter {
    /// Creates a new `DHallHighlighter`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Highlights the given text.
    pub fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        // TODO: Implement real DHall highlighting logic
        let mut highlights = Vec::new();
        let keywords = ["let", "in", "forall", "if", "then", "else", "merge", "using", "as"];

        for keyword in &keywords {
            let mut start = 0;
            while let Some(pos) = text[start..].find(keyword) {
                let absolute_pos = start + pos;
                let end_pos = absolute_pos + keyword.len();
                highlights.push((absolute_pos, end_pos, HighlightKind::Keyword));
                start = absolute_pos + 1
            }
        }

        highlights
    }
}
