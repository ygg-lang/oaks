pub enum HighlightKind {
    Keyword,
    String,
    Number,
    Comment,
    Identifier,
}

pub struct RHighlighter;

impl RHighlighter {
    pub fn new() -> Self {
        Self
    }
    pub fn highlight(&self, _text: &str) -> Vec<(usize, usize, HighlightKind)> {
        Vec::new()
    }
}
