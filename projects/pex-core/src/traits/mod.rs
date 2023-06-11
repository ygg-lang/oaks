use crate::{ParseResult, ParseState};
use core::ops::Range;

/// A trait for parsing a node from a [`ParseState`].
pub trait Parse
where
    Self: Sized,
{
    /// Parse the node from the input.
    fn parse(input: ParseState) -> ParseResult<Self>;
    /// Parse the node from the input as text.
    fn parse_text(input: &str) -> ParseResult<Self> {
        Self::parse(ParseState::new(input))
    }
    /// Get the range of the node as a range of usize.
    fn get_range(&self) -> Range<usize>;
    /// Get the range of the node as a range of u32s.
    fn get_range32(&self) -> Range<u32> {
        let range = self.get_range();
        Range { start: range.start as u32, end: range.end as u32 }
    }
}
