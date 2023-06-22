//! DHall 语法高亮器

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    Keyword,
    String,
    Number,
    Comment,
    Identifier,
}

pub struct DHallHighlighter {}

impl Default for DHallHighlighter {
    fn default() -> Self {
        Self {}
    }
}

impl DHallHighlighter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        // TODO: 实现真正的 DHall 高亮逻辑
        let mut highlights = Vec::new();
        let keywords = ["let", "in", "forall", "if", "then", "else", "merge", "using", "as"];

        for keyword in &keywords {
            let mut start = 0;
            while let Some(pos) = text[start..].find(keyword) {
                let absolute_pos = start + pos;
                let end_pos = absolute_pos + keyword.len();
                highlights.push((absolute_pos, end_pos, HighlightKind::Keyword));
                start = absolute_pos + 1;
            }
        }

        highlights
    }
}
