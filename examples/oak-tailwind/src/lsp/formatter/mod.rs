//! Formatter for the Tailwind language.

use oak_core::SourceText;

/// Formatter for Tailwind language.
pub struct TailwindFormatter;

impl TailwindFormatter {
    /// Formats the given source text.
    pub fn format(&self, source: &SourceText, _indent: usize) -> String {
        // TODO: Implement Tailwind formatting logic
        source.text().to_string()
    }
}
