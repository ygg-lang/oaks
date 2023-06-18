use crate::{
    OakError, SourceLocation,
    source::{Source, SourceView, TextEdit},
};
use lsp_types::Position;
use serde::{Deserialize, Serialize};
use std::range::Range;
use url::Url;

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

impl<'input> Source for &'input SourceText {
    fn length(&self) -> usize {
        self.raw.len()
    }

    fn get_text_in(&self, range: Range<usize>) -> &str {
        self.raw.get(range.start..range.end).unwrap_or("")
    }

    fn offset_to_position(&self, offset: usize) -> Position {
        let total_len = self.raw.len();
        // 超出文件范围，返回最后一行的末尾位置（0-based 行号）
        if offset >= total_len {
            let last_line_idx = self.line_map.len().saturating_sub(1);
            let line_start = self.line_map.get(last_line_idx).copied().unwrap_or(0);
            let line = last_line_idx as u32; // 0-based
            let column = (total_len.saturating_sub(line_start)) as u32; // 以字符计数
            return Position { line, character: column };
        }

        // 二分查找所在行的起始偏移
        let line_idx = self.line_map.binary_search(&offset).unwrap_or_else(|idx| idx.saturating_sub(1));

        let line_start = self.line_map[line_idx];
        let line = line_idx as u32; // 0-based 行号
        // 计算列：从行起始到目标偏移的字符数量
        let column = self.raw[line_start..offset].chars().count() as u32;
        Position { line, character: column }
    }

    fn position_to_offset(&self, position: Position) -> usize {
        let line = position.line as usize;
        let column = position.character as usize;

        // 超出了文件范围
        if line >= self.line_map.len() {
            // 超出了最后一行，返回文件末尾
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
    #[allow(mismatched_lifetime_syntaxes)]
    pub fn view(&self, range: Range<usize>) -> SourceView {
        SourceView { source: self, range }
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
    pub fn lsp_to_text_edit(&self, edit: lsp_types::TextEdit) -> TextEdit {
        TextEdit { span: self.lsp_range_to_span(edit.range), text: edit.new_text }
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
        let position = self.offset_to_position(offset);
        // SourceLocation 的行号语义为 1-based
        SourceLocation { line: position.line + 1, column: position.character, url: self.url.clone() }
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
