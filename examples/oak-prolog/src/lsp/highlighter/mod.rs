#![doc = include_str!("readme.md")]

/// Highlight kind enumeration
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

/// Highlighter trait
pub trait Highlighter {
    /// Highlights the given text
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

/// Prolog syntax highlighter
pub struct PrologHighlighter {
    pub use_parser: bool,
}

impl Default for PrologHighlighter {
    fn default() -> Self {
        Self { use_parser: false }
    }
}

impl PrologHighlighter {
    /// Creates a new Prolog highlighter instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a highlighter that uses the parser
    pub fn with_parser() -> Self {
        Self { use_parser: true }
    }

    fn highlight_keywords(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let keywords = ["is", "mod", "div", "abs", "sin", "cos", "tan", "exp", "log", "sqrt", "dynamic", "multifile", "discontiguous", "module", "use_module", "export", "import", "if", "then", "else", "spy", "nospy", "trace", "notrace"];

        for keyword in &keywords {
            let mut start = 0;
            while let Some(pos) = text[start..].find(keyword) {
                let absolute_pos = start + pos;
                let end_pos = absolute_pos + keyword.len();

                let is_word_boundary_before = absolute_pos == 0 || !text.chars().nth(absolute_pos - 1).unwrap_or(' ').is_alphanumeric();
                let is_word_boundary_after = end_pos >= text.len() || !text.chars().nth(end_pos).unwrap_or(' ').is_alphanumeric();

                if is_word_boundary_before && is_word_boundary_after {
                    highlights.push((absolute_pos, end_pos, HighlightKind::Keyword));
                }

                start = absolute_pos + 1;
            }
        }

        highlights
    }
}

impl Highlighter for PrologHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = self.highlight_keywords(text);
        highlights.sort_by_key(|h| h.0);
        highlights
    }
}
