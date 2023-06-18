use crate::{SourceText, source::Source};
use lsp_types::Position;
use std::range::Range;

/// A view into a portion of source text.
///
/// This struct represents a slice or window into a larger source text,
/// allowing parsers to work with specific sections of code while maintaining
/// proper position tracking and coordinate conversion.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SourceView<'view> {
    pub(crate) source: &'view SourceText,
    pub(crate) range: Range<usize>,
}

impl<'view> From<&'view SourceText> for SourceView<'view> {
    fn from(value: &'view SourceText) -> Self {
        Self { source: value, range: Range { start: 0, end: value.len() } }
    }
}

impl<'view> Source for SourceView<'view> {
    fn length(&self) -> usize {
        self.range.end.saturating_sub(self.range.start)
    }

    fn get_text_in(&self, range: Range<usize>) -> &str {
        // Calculate the absolute range within the underlying source
        let start = self.range.start + range.start;
        let end = self.range.start + range.end;

        // Ensure we don't exceed the view's bounds
        let end = end.min(self.range.end);

        self.source.get_text_in((start..end).into())
    }

    fn offset_to_position(&self, offset: usize) -> Position {
        // Convert view-relative offset to source-relative offset
        let source_offset = self.range.start + offset;
        self.source.offset_to_position(source_offset)
    }

    fn position_to_offset(&self, position: Position) -> usize {
        // Convert position to source-relative offset, then adjust to view-relative
        let source_offset = self.source.position_to_offset(position);
        source_offset.saturating_sub(self.range.start)
    }
}

impl<'view> SourceView<'view> {
    /// Creates a new view into a sub-range of this view.
    ///
    /// This method allows creating nested views within existing views,
    /// enabling parsers to work with increasingly specific sections of code.
    ///
    /// # Arguments
    ///
    /// * `range` - The relative range within this view to create a sub-view for
    ///
    /// # Returns
    ///
    /// A new `SourceView` representing the specified sub-range
    pub fn view(&self, range: Range<usize>) -> Self {
        let start = self.range.start + range.start;
        let end = self.range.start + range.end;
        Self { source: self.source, range: Range { start, end } }
    }
}
