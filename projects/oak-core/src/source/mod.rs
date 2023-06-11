//! Source text management and location tracking for incremental parsing.
//!
//! This module provides structures for managing source code text and tracking
//! locations within it, including support for LSP (Language Server Protocol) integration.

extern crate url;

use crate::errors::OakError;
use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::{fmt::{Display, Formatter}, range::Range};
use serde::{Deserialize, Serialize};
pub use url::Url;

/// Represents source code text with line mapping and optional URL reference.
///
/// This struct manages the raw source text and provides utilities for:
/// - Text extraction at specific offsets or ranges
/// - Character and line/column position tracking
/// - LSP position and range conversions (when `lsp-types` feature is enabled)
/// - Error reporting with precise location information
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SourceText {
    pub(crate) url: Option<Url>,
    pub(crate) raw: String,
    pub(crate) line_map: Vec<usize>,
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

impl SourceText {
    /// Applies multiple text edits to the source text and returns the minimum affected offset.
    ///
    /// This method is used for incremental updates to source code, such as those
    /// received from LSP clients or other text editing operations.
    ///
    /// # Arguments
    ///
    /// * `edits` - A slice of [`TextEdit`] operations to apply
    ///
    /// # Returns
    ///
    /// The minimum byte offset that was affected by any of the edits. This is
    /// useful for determining where to restart parsing after incremental changes.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut source = SourceText::new("let x = 5;");
    /// let edits = vec![TextEdit { span: 4..5, text: "y".to_string() }];
    /// let min_offset = source.apply_edits(&edits);
    /// assert_eq!(min_offset, 4);
    /// ```
    pub fn apply_edits(&mut self, edits: &[TextEdit]) -> usize {
        let mut min = self.raw.len();
        for TextEdit { span, text } in edits {
            min = min.min(span.start);
            self.raw.replace_range(span.start..span.end, text);
        }
        min
    }

    /// Creates a new [`SourceText`] containing a slice of the original text.
    ///
    /// This method extracts a portion of the source text and creates a new
    /// [`SourceText`] instance with the extracted content. The line map is
    /// rebuilt for the new content.
    ///
    /// # Arguments
    ///
    /// * `range` - The byte range to extract from the original text
    ///
    /// # Returns
    ///
    /// A new [`SourceText`] instance containing the extracted text slice
    ///
    /// # Examples
    ///
    /// ```
    /// let source = SourceText::new("fn main() {\n    println!(\"Hello\");\n}");
    /// let slice = source.slice(0..12); // "fn main() {"
    /// ```
    pub fn slice(&self, range: Range<usize>) -> Self {
        Self { url: self.url.clone(), raw: self.raw[range].to_string(), line_map: build_line_map(&self.raw[range]) }
    }

    /// Gets the URL associated with this source text, if any.
    ///
    /// # Returns
    ///
    /// An [`Option<&Url>`] containing the URL reference if one was set,
    /// or `None` if no URL is associated with this source text.
    ///
    /// # Examples
    ///
    /// ```
    /// let source = SourceText::new_with_url("code", Url::parse("file:///main.rs").unwrap());
    /// assert!(source.get_url().is_some());
    /// ```
    pub fn get_url(&self) -> Option<&Url> {
        self.url.as_ref()
    }

    /// Gets the length of the source text in bytes.
    ///
    /// # Returns
    ///
    /// The length of the source text in bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// let source = SourceText::new("Hello, world!");
    /// assert_eq!(source.len(), 13);
    /// ```
    pub fn len(&self) -> usize {
        self.raw.len()
    }

    /// Checks if the source text is empty.
    ///
    /// # Returns
    ///
    /// `true` if the source text is empty, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let source = SourceText::new("");
    /// assert!(source.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.raw.is_empty()
    }
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

impl Display for SourceLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        if let Some(url) = &self.url {
            write!(f, "{}:{}:{}", url, self.line, self.column)
        } else {
            write!(f, "{}:{}", self.line, self.column)
        }
    }
}

impl SourceText {
    /// Creates a new SourceText from a string.
    ///
    /// # Arguments
    ///
    /// * `input` - The source code text
    ///
    /// # Examples
    ///
    /// ```
    /// let source = SourceText::new("fn main() {}");
    /// ```
    pub fn new(input: impl ToString) -> Self {
        let text = input.to_string();
        let line_map = build_line_map(&text);
        Self { url: None, raw: text, line_map }
    }
    /// Creates a new SourceText from a string with an optional URL.
    ///
    /// # Arguments
    ///
    /// * `input` - The source code text
    /// * `url` - URL reference for the source file
    ///
    /// # Examples
    ///
    /// ```
    /// let source = SourceText::new_with_url("fn main() {}", Url::parse("file:///main.rs").unwrap());
    /// ```
    pub fn new_with_url(input: impl ToString, url: Url) -> Self {
        let text = input.to_string();
        let line_map = build_line_map(&text);
        Self { url: Some(url), raw: text, line_map }
    }

    /// Gets text starting from the specified byte offset.
    ///
    /// # Arguments
    ///
    /// * `offset` - The byte offset from which to start extracting text
    ///
    /// # Returns
    ///
    /// An `Option<&str>` containing the text from the offset to the end of the file,
    /// or `None` if the offset is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// let source = SourceText::new("Hello, world!");
    /// assert_eq!(source.get_text_at(7), Some("world!"));
    /// ```
    pub fn get_text_at(&self, offset: usize) -> Option<&str> {
        self.raw.get(offset..)
    }

    /// Gets text within the specified byte range.
    ///
    /// # Arguments
    ///
    /// * `span` - The byte range (start..end) to extract text from
    ///
    /// # Returns
    ///
    /// An `Option<&str>` containing the text within the specified range,
    /// or `None` if the range is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// let source = SourceText::new("Hello, world!");
    /// assert_eq!(source.get_text_in(0..5), Some("Hello"));
    /// ```
    pub fn get_text_in(&self, span: Range<usize>) -> Option<&str> {
        self.raw.get(span.start..span.end)
    }
    /// Gets the character at the specified byte offset.
    ///
    /// # Arguments
    ///
    /// * `offset` - The byte offset of the character to retrieve
    ///
    /// # Returns
    ///
    /// An `Option<char>` containing the first character at the offset,
    /// or `None` if the offset is out of bounds or not at a character boundary.
    ///
    /// # Examples
    ///
    /// ```
    /// let source = SourceText::new("Hello, world!");
    /// assert_eq!(source.get_char_at(0), Some('H'));
    /// ```
    pub fn get_char_at(&self, offset: usize) -> Option<char> {
        self.get_text_at(offset)?.chars().next()
    }

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
    #[cfg(feature = "lsp-types")]
    pub fn get_lsp_range(&self, span: Range<usize>) -> lsp_types::Range {
        let start = self.get_lsp_position(span.start);
        let end = self.get_lsp_position(span.end);
        lsp_types::Range { start, end }
    }

    /// Converts a byte offset to an LSP Position.
    ///
    /// # Arguments
    ///
    /// * `offset` - The byte offset to convert
    ///
    /// # Returns
    ///
    /// An `lsp_types::Position` with line and character information.
    ///
    /// # Availability
    ///
    /// This method is only available when the `lsp-types` feature is enabled.
    #[cfg(feature = "lsp-types")]
    pub fn get_lsp_position(&self, offset: usize) -> lsp_types::Position {
        let (line, column) = self.get_line_column(offset);
        lsp_types::Position {
            // LSP uses 0-based line numbers
            line: line.saturating_sub(1),
            // LSP uses 0-based character positions
            character: column,
        }
    }

    /// Converts an LSP TextEdit to a TextEdit.
    ///
    /// # Arguments
    ///
    /// * `edit` - The LSP TextEdit to convert
    ///
    /// # Returns
    ///
    /// A `TextEdit` with byte-based span suitable for internal use.
    ///
    /// # Availability
    ///
    /// This method is only available when the `lsp-types` feature is enabled.
    #[cfg(feature = "lsp-types")]
    pub fn lsp_to_text_edit(&self, edit: lsp_types::TextEdit) -> TextEdit {
        TextEdit { span: self.lsp_to_source_span(edit.range), text: edit.new_text }
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
    #[cfg(feature = "lsp-types")]
    pub fn lsp_to_source_span(&self, range: lsp_types::Range) -> Range<usize> {
        Range { start: self.lsp_to_offset(range.start), end: self.lsp_to_offset(range.end) }
    }

    /// Converts an LSP Position to a byte offset.
    ///
    /// # Arguments
    ///
    /// * `position` - The LSP Position to convert
    ///
    /// # Returns
    ///
    /// A `usize` byte offset.
    ///
    /// # Availability
    ///
    /// This method is only available when the `lsp-types` feature is enabled.
    #[cfg(feature = "lsp-types")]
    pub fn lsp_to_offset(&self, position: lsp_types::Position) -> usize {
        let line = position.line as usize;
        let column = position.character as usize;

        // Handle out-of-bounds line numbers
        if line >= self.line_map.len() {
            return self.raw.len();
        }

        let line_start = self.line_map[line];

        // Find the end of this line
        let line_end = if line + 1 < self.line_map.len() { self.line_map[line + 1] } else { self.raw.len() };

        // Calculate the byte offset within the line, handling UTF-8 character boundaries
        let mut current_column = 0;
        let mut offset = line_start;

        for ch in self.raw[line_start..line_end].chars() {
            if current_column >= column {
                break;
            }
            current_column += 1;
            offset += ch.len_utf8();
        }

        offset
    }

    /// Converts a byte offset to line and column numbers.
    ///
    /// # Arguments
    ///
    /// * `offset` - The byte offset to convert
    ///
    /// # Returns
    ///
    /// A tuple `(line, column)` where line is 1-based and column is 0-based.
    ///
    /// # Examples
    ///
    /// ```
    /// let source = SourceText::new("Hello\nworld!");
    /// assert_eq!(source.get_line_column(7), (2, 0)); // 'w' in "world!"
    /// ```
    pub fn get_line_column(&self, offset: usize) -> (u32, u32) {
        if offset >= self.raw.len() {
            return (self.line_map.len() as u32, 0);
        }

        // Find the line containing this offset using binary search
        let line_idx = self.line_map.binary_search(&offset).unwrap_or_else(|idx| idx.saturating_sub(1));

        let line_start = self.line_map[line_idx];
        let line = (line_idx + 1) as u32; // 1-based line numbers
        let column = (offset - line_start) as u32; // 0-based column numbers

        (line, column)
    }

    /// Creates a kind error with location information.
    ///
    /// # Arguments
    ///
    /// * `message` - The error message
    /// * `offset` - The byte offset where the error occurred
    ///
    /// # Returns
    ///
    /// A `PexError` with precise location information including line and column.
    ///
    /// # Examples
    ///
    /// ```
    /// let source = SourceText::new("let x =");
    /// let error = source.syntax_error("Unexpected end of input", 7);
    /// ```
    /// Creates a kind error with location information.
    ///
    /// # Arguments
    ///
    /// * `message` - The error message
    /// * `offset` - The byte offset where the error occurred
    ///
    /// # Returns
    ///
    /// A `PexError` with precise location information including line and column.
    ///
    /// # Examples
    ///
    /// ```
    /// let source = SourceText::new("let x =");
    /// let error = source.syntax_error("Unexpected end of input", 7);
    /// ```
    pub fn syntax_error(&self, message: impl Into<String>, offset: usize) -> OakError {
        OakError::syntax_error(message, self.get_location(offset))
    }

    /// Creates an error for an unexpected character with location information.
    ///
    /// # Arguments
    ///
    /// * `character` - The unexpected character
    /// * `offset` - The byte offset where the unexpected character was found
    ///
    /// # Returns
    ///
    /// A `PexError` with precise location information including line and column.
    ///
    /// # Examples
    ///
    /// ```
    /// let source = SourceText::new("let x@y = 5");
    /// let error = source.unexpected_character('@', 6);
    /// ```
    pub fn unexpected_character(&self, character: char, offset: usize) -> OakError {
        OakError::unexpected_character(character, self.get_location(offset))
    }

    /// Gets the source location for a given byte offset.
    ///
    /// # Arguments
    ///
    /// * `offset` - The byte offset to get location for
    ///
    /// # Returns
    ///
    /// A `SourceLocation` with line, column, and optional URL information.
    ///
    /// # Examples
    ///
    /// ```
    /// let source = SourceText::new("line 1\nline 2\n");
    /// let location = source.get_location(8); // Start of second line
    /// assert_eq!(location.line, 2);
    /// assert_eq!(location.column, 0);
    /// ```
    pub fn get_location(&self, offset: usize) -> SourceLocation {
        let (line, column) = self.get_line_column(offset);
        SourceLocation { line, column, url: self.url.clone() }
    }
}

/// Builds a line map for efficient line/column calculations.
///
/// This creates a vector of byte offsets where each line starts.
/// Handles both LF (`\n`) and CRLF (`\r\n`) line endings properly.
fn build_line_map(text: &str) -> Vec<usize> {
    let mut line_map = vec![0]; // First line starts at offset 0
    let mut chars = text.char_indices().peekable();

    while let Some((i, ch)) = chars.next() {
        if ch == '\r' {
            // Check for CRLF sequence
            if let Some((_, '\n')) = chars.peek() {
                // Skip the '\n' as it's part of CRLF
                chars.next();
                // Next line starts after CRLF
                line_map.push(i + 2);
            }
            else {
                // Standalone CR - treat as line ending
                line_map.push(i + 1);
            }
        }
        else if ch == '\n' {
            line_map.push(i + 1);
        }
    }
    line_map
}
