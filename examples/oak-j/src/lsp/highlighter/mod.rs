#![doc = include_str!("readme.md")]

/// Local definition of highlight kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    /// Keywords (corresponding to primitives in J).
    Keyword,
    /// Strings.
    String,
    /// Numbers.
    Number,
    /// Comments.
    Comment,
    /// Identifiers.
    Identifier,
}

/// Highlighter trait.
pub trait Highlighter {
    /// Highlights the given text.
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

/// J syntax highlighter.
pub struct JHighlighter {
    /// Whether to use parser-based highlighting.
    pub use_parser: bool,
}

impl Default for JHighlighter {
    fn default() -> Self {
        Self { use_parser: false }
    }
}

impl JHighlighter {
    /// Creates a new J highlighter instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Highlights J primitives (corresponding to keywords in other languages).
    fn highlight_keywords(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        // J assignment symbols.
        for op in ["=:", "=."] {
            let mut start = 0;
            while let Some(pos) = text[start..].find(op) {
                highlights.push((start + pos, start + pos + op.len(), HighlightKind::Keyword));
                start += pos + op.len();
            }
        }
        highlights
    }
}

impl Highlighter for JHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = self.highlight_keywords(text);

        // Simple regex or manual scanning logic can be added here.
        // For simplicity, we currently rely mainly on the lexer for highlighting.

        highlights.sort_by_key(|h| h.0);
        highlights
    }
}

#[cfg(feature = "oak-highlight")]
impl oak_highlight::Highlighter for JHighlighter {
    fn highlight<'a>(&self, _source: &'a str, _language: &str, _theme: oak_highlight::Theme) -> oak_core::errors::ParseResult<oak_highlight::HighlightResult<'a>> {
        // TODO: Implement proper highlighting using the new oak-highlight API
        Ok(oak_highlight::HighlightResult { segments: Vec::new(), source: std::borrow::Cow::Borrowed(_source) })
    }
}
