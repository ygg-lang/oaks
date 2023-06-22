//! Source text management and location tracking for incremental parsing.
//!
//! This module provides structures for managing source code text and tracking
//! locations within it.

use core::range::Range;
use std::borrow::Cow;
mod buffer;
mod cursor;
mod rope;
mod simd;
mod streaming;
mod text;

pub use self::{
    buffer::{SourceBuffer, ToSource},
    cursor::SourceCursor,
    rope::{RopeBuffer, RopeSource},
    simd::SimdScanner,
    streaming::{ChunkedBuffer, ChunkedSource},
    text::SourceText,
};
use crate::OakError;

/// A unique identifier for a source file.
pub type SourceId = u32;

/// A chunk of text from a source, including its start offset.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextChunk<'a> {
    /// The start byte offset of this chunk in the source.
    pub start: usize,
    /// The text content of this chunk.
    pub text: &'a str,
}

impl<'a> TextChunk<'a> {
    /// Returns the end byte offset of this chunk.
    #[inline]
    pub fn end(&self) -> usize {
        self.start + self.text.len()
    }

    /// Returns a slice of the chunk text starting from the specified absolute offset.
    #[inline]
    pub fn slice_from(&self, offset: usize) -> &'a str {
        if offset <= self.start {
            return self.text;
        }
        let rel = offset.saturating_sub(self.start);
        self.text.get(rel..).unwrap_or("")
    }
}

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
/// # #![feature(new_range_api)]
/// # use oak_core::source::TextEdit;
/// use core::range::Range;
/// let edit = TextEdit {
///     span: Range { start: 4, end: 9 }, // Replace characters at positions 4-8
///     text: "world".into(),             // With the text "world"
/// };
/// ```
pub struct TextEdit {
    /// The byte range in the original text to be replaced (start..end)
    pub span: Range<usize>,
    /// The new text to insert in place of the specified range
    pub text: Cow<'static, str>,
}

/// Trait for abstract text sources.
///
/// This trait provides a unified interface for different text sources that may have:
/// - Different character representations (Unicode escapes, HTML entities)
/// - Different internal storage formats
/// - Different error handling requirements
///
/// All offsets exposed by this trait are simple text ranges from the start of this source.
pub trait Source: Send + Sync {
    /// Get the length of this source.
    ///
    /// This represents the total size of this source in bytes.
    fn length(&self) -> usize;

    /// Returns the ID of this source, if available.
    fn source_id(&self) -> Option<SourceId> {
        None
    }

    /// Returns a text chunk containing the specified offset.
    fn chunk_at(&self, offset: usize) -> TextChunk<'_>;

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
        self.chunk_at(offset).slice_from(offset).chars().next()
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
    /// The text content in the specified range.
    fn get_text_in(&self, range: Range<usize>) -> Cow<'_, str>;

    /// Get the text from the current position to the end of the source.
    ///
    /// # Arguments
    ///
    /// * `offset` - The byte offset to start from (relative to this source)
    ///
    /// # Returns
    ///
    /// The remaining text from the offset to the end.
    fn get_text_from(&self, offset: usize) -> Cow<'_, str> {
        if offset >= self.length() {
            return Cow::Borrowed("");
        }
        self.get_text_in(core::range::Range { start: offset, end: self.length() })
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
        let mut cursor = SourceCursor::new_at(self, offset);
        let mut base = offset;
        loop {
            let rest = cursor.rest();
            if let Some(pos) = rest.find(ch) {
                return Some(base + pos);
            }
            let next = cursor.chunk_end();
            if next >= self.length() {
                return None;
            }
            base = next;
            cursor.set_position(next);
        }
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
        let mut cursor = SourceCursor::new_at(self, offset);
        cursor.find_str(pattern)
    }

    /// Create a syntax error with location information.
    ///
    /// # Arguments
    ///
    /// * `message` - The error message
    /// * `offset` - The byte offset where the error occurred
    ///
    /// # Returns
    ///
    /// An [`OakError`] with precise location information.
    fn syntax_error(&self, message: String, offset: usize) -> OakError {
        OakError::syntax_error(message, offset, self.source_id())
    }
}

impl Source for str {
    fn length(&self) -> usize {
        self.len()
    }

    fn chunk_at(&self, offset: usize) -> TextChunk<'_> {
        let len = self.len();
        if offset >= len {
            return TextChunk { start: len, text: "" };
        }
        TextChunk { start: offset, text: &self[offset..] }
    }

    fn get_text_in(&self, range: Range<usize>) -> Cow<'_, str> {
        self.get(range.start..range.end).map(Cow::Borrowed).unwrap_or(Cow::Borrowed(""))
    }
}

impl<S: Source + ?Sized> Source for &S {
    fn length(&self) -> usize {
        (**self).length()
    }

    fn chunk_at(&self, offset: usize) -> TextChunk<'_> {
        (**self).chunk_at(offset)
    }

    fn get_text_in(&self, range: Range<usize>) -> Cow<'_, str> {
        (**self).get_text_in(range)
    }

    fn source_id(&self) -> Option<SourceId> {
        (**self).source_id()
    }

    fn get_char_at(&self, offset: usize) -> Option<char> {
        (**self).get_char_at(offset)
    }

    fn get_text_from(&self, offset: usize) -> Cow<'_, str> {
        (**self).get_text_from(offset)
    }

    fn find_char_from(&self, offset: usize, ch: char) -> Option<usize> {
        (**self).find_char_from(offset, ch)
    }

    fn find_str_from(&self, offset: usize, pattern: &str) -> Option<usize> {
        (**self).find_str_from(offset, pattern)
    }

    fn syntax_error(&self, message: String, offset: usize) -> OakError {
        (**self).syntax_error(message, offset)
    }
}
