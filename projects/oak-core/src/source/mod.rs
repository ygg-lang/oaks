//! Source text management and location tracking for incremental parsing.
//!
//! This module provides structures for managing source code text and tracking
//! locations within it, including support for LSP (Language Server Protocol) integration.

mod text;
mod view;

pub use self::{text::SourceText, view::SourceView};
use crate::OakError;
use lsp_types::Position;
use serde::{Deserialize, Serialize};
use std::range::Range;
pub use url::Url;

/// Represents a text edit operation for incremental updates.
///
/// Text edits are used to apply changes to source text in an incremental
/// manner, such as those received from LSP clients or other text editing
/// operations. Each edit specifies a byte range to replace and the new text
/// to insert in that range.
///
/// # Examples
///
/// ```
/// let edit = TextEdit {
///     span: 4..9,           // Replace characters at positions 4-8
///     text: "world".into(), // With the text "world"
/// };
/// ```
pub struct TextEdit {
    /// The byte range in the original text to be replaced (start..end)
    pub span: Range<usize>,
    /// The new text to insert in place of the specified range
    pub text: String,
}

/// Represents a specific location within source code.
///
/// This struct provides line and column information for error reporting
/// and debugging, optionally including a URL reference to the source file.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SourceLocation {
    /// The 1-based line number in the source text
    pub line: u32,
    /// The 0-based column number within the line
    pub column: u32,
    /// Optional URL reference to the source file
    pub url: Option<Url>,
}

/// Trait for abstract text sources with error position management.
///
/// This trait provides a unified interface for different text sources that may have:
/// - Different character representations (Unicode escapes, HTML entities)
/// - Different internal storage formats
/// - Different error handling requirements
///
/// All offsets exposed by this trait are simple text ranges from the start of this source.
/// Internal complexity like global offset mapping, character encoding transformations,
/// and position tracking are handled internally.
pub trait Source {
    /// Get the length of this source.
    ///
    /// This represents the total size of this source in bytes.
    fn length(&self) -> usize;

    /// Check if the source is empty.
    fn is_empty(&self) -> bool {
        self.length() == 0
    }

    /// Get a single character at the specified offset.
    ///
    /// This method should handle any character encoding transformations
    /// and return the actual character that would be seen by the parser.
    ///
    /// # Arguments
    ///
    /// * `offset` - The byte offset from the start of this source
    ///
    /// # Returns
    ///
    /// The character at the specified offset, or `None` if the offset is invalid
    fn get_char_at(&self, offset: usize) -> Option<char> {
        self.get_text_from(offset).chars().next()
    }

    /// Get the text content at the specified range.
    ///
    /// The range is specified as simple offsets from the start of this source.
    /// The returned text should have any character encoding transformations
    /// already applied (e.g., Unicode escapes decoded, HTML entities resolved).
    ///
    /// # Arguments
    ///
    /// * `range` - The byte range to extract text from (relative to this source)
    ///
    /// # Returns
    ///
    /// The text content in the specified range, or `None` if the range is invalid
    fn get_text_in(&self, range: Range<usize>) -> &str;

    /// Get the text from the current position to the end of the source.
    ///
    /// # Arguments
    ///
    /// * `offset` - The byte offset to start from (relative to this source)
    ///
    /// # Returns
    ///
    /// The remaining text from the offset to the end, or `None` if the offset is invalid
    fn get_text_from(&self, offset: usize) -> &str {
        if offset >= self.length() {
            return "";
        }
        self.get_text_in((offset..self.length()).into())
    }

    /// Get the URL of this source, if available.
    ///
    /// This method returns a reference to the URL associated with this source,
    /// typically used for file-based sources or remote resources.
    ///
    /// # Returns
    ///
    /// An optional reference to the source URL, or `None` if no URL is available
    fn get_url(&self) -> Option<&Url> {
        None
    }

    /// Convert an offset to position information for error reporting.
    ///
    /// This method handles the mapping from offsets to human-readable
    /// line/column positions for error reporting.
    ///
    /// # Arguments
    ///
    /// * `offset` - The byte offset from the start of this source
    ///
    /// # Returns
    ///
    /// A [`SourcePosition`] with line and column information,
    /// or `None` if the offset is invalid
    fn offset_to_position(&self, offset: usize) -> Position;

    /// Convert a position to an offset.
    ///
    /// # Arguments
    ///
    /// * `position` - The position to convert
    ///
    /// # Returns
    ///
    /// The offset corresponding to the position
    fn position_to_offset(&self, position: Position) -> usize;

    /// Converts a byte range to an LSP Range.
    ///
    /// # Arguments
    ///
    /// * `span` - The byte range to convert
    ///
    /// # Returns
    ///
    /// An `lsp_types::Range` with line/column positions.
    ///
    /// # Availability
    ///
    /// This method is only available when the `lsp-types` feature is enabled.
    fn span_to_lsp_range(&self, span: Range<usize>) -> lsp_types::Range {
        let start = self.offset_to_position(span.start);
        let end = self.offset_to_position(span.end);
        lsp_types::Range { start, end }
    }

    /// Converts an LSP Range to a byte-based source span.
    ///
    /// # Arguments
    ///
    /// * `range` - The LSP Range to convert
    ///
    /// # Returns
    ///
    /// A `Range<usize>` representing the byte offset range.
    ///
    /// # Availability
    ///
    /// This method is only available when the `lsp-types` feature is enabled.
    fn lsp_range_to_span(&self, range: lsp_types::Range) -> Range<usize> {
        Range { start: self.position_to_offset(range.start), end: self.position_to_offset(range.end) }
    }

    /// Find the next occurrence of a character starting from an offset.
    ///
    /// # Arguments
    ///
    /// * `offset` - The byte offset to start searching from (relative to this source)
    /// * `ch` - The character to search for
    ///
    /// # Returns
    ///
    /// The offset of the next occurrence, or `None` if not found
    fn find_char_from(&self, offset: usize, ch: char) -> Option<usize> {
        let text = self.get_text_from(offset);
        text.find(ch).map(|pos| offset + pos)
    }

    /// Find the next occurrence of a substring starting from an offset.
    ///
    /// # Arguments
    ///
    /// * `offset` - The byte offset to start searching from (relative to this source)
    /// * `pattern` - The substring to search for
    ///
    /// # Returns
    ///
    /// The offset of the next occurrence, or `None` if not found
    fn find_str_from(&self, offset: usize, pattern: &str) -> Option<usize> {
        let text = self.get_text_from(offset);
        text.find(pattern).map(|pos| offset + pos)
    }

    /// Create an error for an invalid range.
    ///
    /// # Arguments
    ///
    /// * `range` - The invalid range
    /// * `message` - The error message
    ///
    /// # Returns
    ///
    /// An [`OakError`] with position information at the start of the range
    fn syntax_error(&self, message: impl Into<String>, position: usize) -> OakError {
        let position = self.offset_to_position(position);
        OakError::syntax_error(
            message.into(),
            SourceLocation { line: position.line, column: position.character, url: self.get_url().cloned() },
        )
    }
}
