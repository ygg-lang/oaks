use crate::{ParseResult, ParseResult::Stop, Parsed};

use super::*;

/// Helper for choice pattern
#[derive(Debug, Clone)]
pub struct ChoiceHelper<'a, T> {
    state: ParseState<'a>,
    result: Option<Parsed<'a, T>>,
}

impl<'i> ParseState<'i> {
    /// Begin a choice progress
    #[inline]
    pub fn begin_choice<T>(self) -> ChoiceHelper<'i, T> {
        ChoiceHelper { state: self, result: None }
    }
}

impl<'a, T> ChoiceHelper<'a, T> {
    /// Create a new choice helper
    #[inline]
    pub fn new(state: ParseState<'a>) -> Self {
        Self { state, result: None }
    }
    /// Try to parse a value
    #[inline]
    pub fn choose<F>(mut self, mut parse: F) -> Self
    where
        F: FnMut(ParseState<'a>) -> ParseResult<'a, T>,
    {
        if self.result.is_none() {
            match parse(self.state.clone()) {
                Pending(s, v) => self.result = Some((s, v)),
                Stop(err) => self.state.set_error(err),
            }
        }
        self
    }
    /// Try to parse a value
    pub fn choose_from<F, U>(self, mut parse: F) -> Self
    where
        F: FnMut(ParseState<'a>) -> ParseResult<'a, U>,
        T: From<U>,
    {
        self.choose(|s| parse(s).map_inner(|s| T::from(s)))
    }

    /// End choice
    #[inline]
    pub fn end_choice(self) -> ParseResult<'a, T> {
        match self.result {
            Some(ok) => Pending(ok.0, ok.1),
            None => Stop(self.state.get_error()),
        }
    }
}
