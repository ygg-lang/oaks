#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    Keyword,
    String,
    Number,
    Comment,
    Identifier,
}

pub struct JuliaHighlighter;

impl JuliaHighlighter {
    pub fn new() -> Self {
        Self
    }

    pub fn highlight(&self, _text: &str) -> Vec<(usize, usize, HighlightKind)> {
        // TODO: Implement actual highlighting
        Vec::new()
    }
}
