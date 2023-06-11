#![doc = include_str!("readme.md")]
use crate::{ParseResult, ParseState, StopBecause};
mod bracket;
mod color;
mod comment;
mod number;
mod string;
mod surround_pair;
mod trie_set;
mod zero_base_byte;

pub use self::{
    color::HexColor,
    comment::{CommentBlock, CommentLine},
    number::*,
    string::{
        paragraph_break, quotation_pair, quotation_pair_escaped, quotation_pair_nested, surround_pair_with_escaper,
        unescape_us, UnicodeUnescape,
    },
    surround_pair::{SurroundPair, SurroundPattern},
    trie_set::CharactersTrie,
    zero_base_byte::ZeroBytePattern,
};
use crate::{
    utils::hex4_to_char,
    ParseResult::{Pending, Stop},
    StringView,
};

/// Match ascii whitespace and newlines, fail if empty
///
/// # Examples
///
/// ```
/// # use pex::{ParseResult, ParseState, helpers::ascii_whitespace};
/// let state = ParseState::new("  \na");
/// state.skip(ascii_whitespace);
/// ```
#[inline]
pub fn ascii_whitespace<'i>(state: ParseState<'i>) -> ParseResult<&'i str> {
    match state.residual.find(|c: char| !c.is_ascii_whitespace()) {
        Some(len) => state.advance_view(len),
        None => StopBecause::missing_character(' ', state.start_offset)?,
    }
}

/// Match whitespace and newlines, fail if empty
///
/// # Examples
///
/// ```
/// # use pex::{ParseResult, ParseState, helpers::whitespace};
/// let state = ParseState::new("  \na");
/// state.skip(whitespace);
/// ```
#[inline]
pub fn whitespace<'i>(state: ParseState<'i>) -> ParseResult<&'i str> {
    match state.residual.find(|c: char| !c.is_whitespace()) {
        Some(len) => state.advance_view(len),
        None => StopBecause::missing_character(' ', state.start_offset)?,
    }
}

/// Function form of the str combinator.
///
/// # Examples
///
/// ```
/// # use pex::{ParseResult, ParseState, helpers::str};
/// let state = ParseState::new("  \na");
/// state.skip(str(" "));
/// ```
#[inline]
pub fn str<'i>(s: &'static str) -> impl Fn(ParseState<'i>) -> ParseResult<'i, &'i str> {
    move |input: ParseState| input.match_str(s)
}

/// Function form of the char combinator.
///
/// # Examples
///
/// ```
/// # use pex::{ParseResult, ParseState, helpers::char};
/// let state = ParseState::new("  \na");
/// state.skip(char(' '));
/// ```
#[inline]
pub fn char(c: char) -> impl Fn(ParseState) -> ParseResult<char> {
    move |input: ParseState| input.match_char(c)
}

/// Function form of the char combinator.
///
/// # Examples
///
/// ```
/// # use pex::{ParseResult, ParseState, helpers::{char, omit}};
/// let state = ParseState::new("  \na");
/// state.skip(omit(char(' ')));
/// ```
#[inline]
pub fn omit<T, F>(mut parse: F) -> impl FnMut(ParseState) -> ParseResult<()>
where
    F: FnMut(ParseState) -> ParseResult<T>,
{
    move |input: ParseState| parse(input).map_inner(|_| ())
}

/// Function form of the optional combinator.
///
/// # Examples
///
/// ```
/// # use pex::{ParseResult, ParseState, helpers::{char, optional}};
/// let state = ParseState::new("  \na");
/// state.skip(optional(char('a')));
/// ```
#[inline]
pub fn optional<T, F>(mut parse: F) -> impl FnMut(ParseState) -> ParseResult<Option<T>>
where
    F: FnMut(ParseState) -> ParseResult<T>,
{
    move |input: ParseState| match parse(input) {
        Pending(state, value) => state.finish(Some(value)),
        Stop(_) => input.finish(None),
    }
}

/// Make the [`from_str`](core::str::FromStr) function from the pex parser
///
/// # Examples
///
/// ```
/// # use std::str::FromStr;
/// # use pex::{helpers::{make_from_str, whitespace}, ParseResult, ParseState, StopBecause};
/// # struct Compound;
/// # impl Compound {
/// #     fn parse(state: ParseState) -> ParseResult<Self> {
/// #         state.finish(Compound)
/// #     }
/// # }
/// impl FromStr for Compound {
///     type Err = StopBecause;
///
///     fn from_str(s: &str) -> Result<Self, StopBecause> {
///         // ignore whitespace at the start and end
///         let state = ParseState::new(s.trim_end()).skip(whitespace);
///         make_from_str(state, Self::parse)
///     }
/// }
/// ```
#[inline]
pub fn make_from_str<T, F>(state: ParseState, mut parser: F) -> Result<T, StopBecause>
where
    F: FnMut(ParseState) -> ParseResult<T>,
{
    match parser(state) {
        Pending(state, compound) if state.is_empty() => Ok(compound),
        Pending(state, ..) => Err(StopBecause::ExpectEOF { position: state.start_offset }),
        Stop(e) => Err(e),
    }
}
