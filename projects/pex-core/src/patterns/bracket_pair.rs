use crate::{ParseResult, ParseState, StringView};
use alloc::vec::Vec;

/// A bracket pattern like `[]` or `(1, )`
#[derive(Copy, Clone, Debug)]
pub struct BracketPattern {
    /// The open bracket pattern
    pub open: &'static str,
    /// The close bracket pattern
    pub close: &'static str,
    /// The delimiter of the bracket elements
    pub delimiter: &'static str,
    /// Whether the last element can be dangling
    pub dangling: Option<bool>,
    /// Need add delimiter if there is only one element
    pub one_tailing: bool,
}

/// A bracket pair like `[1, 2, 3]`
#[derive(Debug)]
pub struct BracketPair<'i, T> {
    /// The left bracket
    pub lhs: StringView<'i>,
    /// The right bracket
    pub rhs: StringView<'i>,
    /// The elements in the bracket
    pub body: Vec<T>,
}

impl BracketPattern {
    /// Create a new bracket pattern
    pub fn new(open: &'static str, close: &'static str) -> Self {
        Self { open, close, delimiter: ",", dangling: None, one_tailing: false }
    }
    /// Create a new bracket pattern
    pub fn with_delimiter(mut self, delimiter: &'static str) -> Self {
        self.delimiter = delimiter;
        self
    }
    /// Create a new bracket pattern
    pub fn with_dangling(mut self, dangling: bool) -> Self {
        self.dangling = Some(dangling);
        self
    }
    /// Create a new bracket pattern
    pub fn with_one_tailing(mut self, one_tailing: bool) -> Self {
        self.one_tailing = one_tailing;
        self
    }
}

impl BracketPattern {
    /// ```js
    /// [ ~ ]
    /// [ ~ term (~ , ~ term)* (~ ,)? ~ ]
    ///
    /// <| a, b, c |>
    /// ```
    pub fn consume<'i, F, I, T, U>(&self, input: ParseState<'i>, ignore: I, parser: F) -> ParseResult<'i, BracketPair<'i, T>>
    where
        F: FnMut(ParseState<'i>) -> ParseResult<'i, T> + Copy,
        I: FnMut(ParseState<'i>) -> ParseResult<'i, U> + Copy,
    {
        input
            .begin_choice()
            .choose(|s| self.consume_empty(s, ignore))
            .choose(|s| self.consume_many(s, ignore, parser))
            .end_choice()
    }

    /// `[ ~ ]`
    fn consume_empty<'i, I, T, U>(&self, input: ParseState<'i>, ignore: I) -> ParseResult<'i, BracketPair<'i, T>>
    where
        I: FnMut(ParseState<'i>) -> ParseResult<'i, U>,
    {
        let (s_rhs, lhs) = input.match_str(self.open)?;
        let (finally, rhs) = s_rhs.skip(ignore).match_str(self.close)?;
        finally.finish(BracketPair {
            lhs: StringView::new(lhs, input.start_offset),
            rhs: StringView::new(rhs, s_rhs.start_offset),
            body: Vec::new(),
        })
    }
    /// `[ ~ term (~ , ~ term)* ~ ,? ~ ]`
    fn consume_many<'i, F, I, T, U>(&self, input: ParseState<'i>, ignore: I, parser: F) -> ParseResult<'i, BracketPair<'i, T>>
    where
        F: FnMut(ParseState<'i>) -> ParseResult<'i, T> + Copy,
        I: FnMut(ParseState<'i>) -> ParseResult<'i, U> + Copy,
    {
        let mut terms = Vec::with_capacity(1);
        let (state, lhs) = input.match_str(self.open)?;
        let (state, first) = state.skip(ignore).match_fn(parser)?;
        terms.push(first);
        let (state, _) = state.match_repeats(|s| self.delimiter_term(s, ignore, parser, &mut terms))?;
        let s_rhs = if self.one_tailing && terms.len() == 1 {
            state.skip(ignore).match_str(self.delimiter)?.0
        }
        else {
            match self.dangling {
                Some(true) => state.skip(ignore).match_str(self.delimiter)?.0,
                Some(false) => state,
                None => match state.skip(ignore).match_str(self.delimiter) {
                    ParseResult::Pending(s, _) => s,
                    ParseResult::Stop(_) => state,
                },
            }
        };
        let (finally, rhs) = s_rhs.skip(ignore).match_str(self.close)?;
        finally.finish(BracketPair {
            lhs: StringView::new(lhs, input.start_offset),
            rhs: StringView::new(rhs, s_rhs.start_offset),
            body: terms,
        })
    }
    /// `~ , ~ term`
    fn delimiter_term<'i, 't, F, I, T, U>(
        &self,
        input: ParseState<'i>,
        ignore: I,
        parser: F,
        terms: &'t mut Vec<T>,
    ) -> ParseResult<'i, ()>
    where
        F: FnMut(ParseState<'i>) -> ParseResult<'i, T> + Copy,
        I: FnMut(ParseState<'i>) -> ParseResult<'i, U> + Copy,
    {
        let (state, _) = input.skip(ignore).match_str(self.delimiter)?;
        let (state, term) = state.skip(ignore).match_fn(parser)?;
        terms.push(term);
        state.finish(())
    }
}
