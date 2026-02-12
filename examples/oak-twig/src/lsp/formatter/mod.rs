#![doc = include_str!("readme.md")]
use oak_core::SourceText;

/// Twig language formatter
pub struct TwigFormatter;

impl TwigFormatter {
    pub fn format(&self, source: &SourceText, _indent: usize) -> String {
        // TODO: Implement Twig formatting logic
        source.text().to_string()
    }
}
