#![doc = include_str!("readme.md")]

/// Kind of highlight
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    /// Keywords
    Keyword,
    /// String literals
    String,
    /// Number literals
    Number,
    /// Comments
    Comment,
    /// Identifiers
    Identifier,
    /// Headings
    Heading,
    /// Code blocks
    CodeBlock,
}

/// Highlighter trait for processing text highlights
pub trait Highlighter {
    /// Highlight the given text and return a vector of spans with highlight kinds
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
    /// Highlight the given text into an existing vector
    fn highlight_into(&self, text: &str, output: &mut Vec<(usize, usize, HighlightKind)>);
}

/// Notedown specific highlighter
pub struct NoteHighlighter {
    /// Whether to use the parser for highlighting
    pub use_parser: bool,
}

impl Default for NoteHighlighter {
    fn default() -> Self {
        Self { use_parser: false }
    }
}

impl NoteHighlighter {
    /// Create a new highlighter with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new highlighter that uses the parser
    pub fn with_parser() -> Self {
        Self { use_parser: true }
    }

    fn highlight_patterns(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();

        // Simple heading detection
        for line in text.lines() {
            let line_start = text.find(line).unwrap_or(0);
            if line.starts_with('#') {
                highlights.push((line_start, line_start + line.len(), HighlightKind::Heading))
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

    fn highlight_into(&self, text: &str, output: &mut Vec<(usize, usize, HighlightKind)>) {
        output.extend(self.highlight(text));
    }
}
