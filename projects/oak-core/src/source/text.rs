use crate::{
    OakError,
    source::{Source, SourceId, TextEdit},
};
use core::range::Range;
use std::borrow::Cow;
use triomphe::Arc;

/// Represents source code text with optional source ID reference.
///
/// This struct manages the raw source text and provides utilities for:
/// - Text extraction at specific offsets or ranges
/// - Error reporting with precise location information
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SourceText {
    pub(crate) source_id: Option<SourceId>,
    pub(crate) raw: Arc<str>,
}

impl Default for SourceText {
    fn default() -> Self {
        Self { source_id: None, raw: Arc::from("") }
    }
}

impl Source for SourceText {
    fn length(&self) -> usize {
        self.raw.len()
    }

    fn chunk_at(&self, offset: usize) -> crate::source::TextChunk<'_> {
        let len = self.raw.len();
        if offset >= len {
            return crate::source::TextChunk { start: len, text: "" };
        }
        crate::source::TextChunk { start: offset, text: self.raw.get(offset..).unwrap_or("") }
    }

    fn get_text_in(&self, range: Range<usize>) -> Cow<'_, str> {
        self.raw.get(range.start..range.end).map(Cow::Borrowed).unwrap_or(Cow::Borrowed(""))
    }

    fn source_id(&self) -> Option<SourceId> {
        self.source_id
    }
}

impl SourceText {
    /// Creates a new source text from the given input.
    pub fn new(input: impl Into<Arc<str>>) -> Self {
        Self { source_id: None, raw: input.into() }
    }

    /// Creates a new source text from the given input and source ID.
    pub fn new_with_id(input: impl Into<Arc<str>>, source_id: SourceId) -> Self {
        Self { source_id: Some(source_id), raw: input.into() }
    }

    /// Returns the raw source text as a string slice.
    pub fn text(&self) -> &str {
        &self.raw
    }

    /// Applies multiple text edits to the source text and returns the affected range.
    pub fn apply_edits_range(&mut self, edits: &[TextEdit]) -> Range<usize> {
        let old_len = self.raw.len();
        if edits.is_empty() {
            return Range { start: old_len, end: old_len };
        }

        let mut order: Vec<usize> = (0..edits.len()).collect();
        order.sort_by_key(|&i| edits[i].span.start);

        let mut reparse_from = old_len;
        let mut reparse_to = 0;
        let mut delta: isize = 0;

        for &i in &order {
            let TextEdit { span, text } = &edits[i];
            reparse_from = reparse_from.min(span.start);
            let start_new = (span.start as isize + delta) as usize;
            let end_new = start_new + text.len();
            reparse_to = reparse_to.max(end_new);
            delta += text.len() as isize - (span.end - span.start) as isize;
        }

        let mut raw = self.raw.to_string();
        for &i in order.iter().rev() {
            let TextEdit { span, text } = &edits[i];
            raw.replace_range(span.start..span.end, text);
        }
        self.raw = Arc::from(raw);

        Range { start: reparse_from, end: reparse_to }
    }

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
    /// # #![feature(new_range_api)]
    /// # use oak_core::source::SourceText;
    /// # use oak_core::source::TextEdit;
    /// # use core::range::Range;
    /// let mut source = SourceText::new("let x = 5;");
    /// let edits = vec![TextEdit { span: Range { start: 4, end: 5 }, text: "y".to_string() }];
    /// let min_offset = source.apply_edits(&edits);
    /// assert_eq!(min_offset, 4);
    /// ```
    pub fn apply_edits(&mut self, edits: &[TextEdit]) -> usize {
        self.apply_edits_range(edits).start
    }

    /// Gets the source ID associated with this source text, if any.
    ///
    /// # Returns
    ///
    /// An [`Option<SourceId>`] containing the source ID if one was set,
    /// or `None` if no ID is associated with this source text.
    ///
    /// # Examples
    ///
    /// ```
    /// # use oak_core::SourceText;
    /// let source = SourceText::new_with_id("code", 1);
    /// assert!(source.source_id().is_some());
    /// ```
    pub fn source_id(&self) -> Option<SourceId> {
        self.source_id
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
    /// # use oak_core::SourceText;
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
    /// # use oak_core::SourceText;
    /// let source = SourceText::new("");
    /// assert!(source.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.raw.is_empty()
    }

    /// Creates a syntax error with location information.
    ///
    /// # Arguments
    ///
    /// * `message` - The error message
    /// * `offset` - The byte offset where the error occurred
    ///
    /// # Returns
    ///
    /// An [`OakError`] with precise location information including line and column.
    ///
    /// # Examples
    ///
    /// ```
    /// # use oak_core::SourceText;
    /// let source = SourceText::new("let x =");
    /// let error = source.syntax_error("Unexpected end of input", 7);
    /// ```
    pub fn syntax_error(&self, message: impl Into<String>, offset: usize) -> OakError {
        OakError::syntax_error(message, offset, self.source_id)
    }

    /// Creates an error for an unexpected character at the specified offset.
    pub fn unexpected_character(&self, character: char, offset: usize) -> OakError {
        OakError::unexpected_character(character, offset, self.source_id)
    }

    /// Creates an error for an expected token that was missing at the specified offset.
    pub fn expected_token(&self, expected: impl Into<String>, offset: usize) -> OakError {
        OakError::expected_token(expected, offset, self.source_id)
    }

    /// Creates an error for an expected name that was missing at the specified offset.
    pub fn expected_name(&self, name_kind: impl Into<String>, offset: usize) -> OakError {
        OakError::expected_name(name_kind, offset, self.source_id)
    }

    /// Creates an error for a trailing comma that is not allowed at the specified offset.
    pub fn trailing_comma_not_allowed(&self, offset: usize) -> OakError {
        OakError::trailing_comma_not_allowed(offset, self.source_id)
    }
}
