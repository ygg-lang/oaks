#![doc = include_str!("readme.md")]

/// Rhombus syntax highlighter
pub struct RhombusHighlighter;

/// Alias for backwards compatibility
pub type SchemeHighlighter = RhombusHighlighter;

impl RhombusHighlighter {
    pub fn new() -> Self {
        Self
    }
}
