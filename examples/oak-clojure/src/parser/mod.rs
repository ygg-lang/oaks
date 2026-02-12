/// Element type module for Clojure.
pub mod element_type;
pub use element_type::ClojureElementType;

use crate::{language::ClojureLanguage, lexer::token_type::ClojureTokenType};
use oak_core::{
    OakError,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, ClojureLanguage, S>;

/// Clojure parser implementation.
pub struct ClojureParser<'config> {
    pub(crate) config: &'config ClojureLanguage,
}

impl<'config> ClojureParser<'config> {
    /// Creates a new `ClojureParser`.
    pub fn new(config: &'config ClojureLanguage) -> Self {
        Self { config }
    }

    fn parse_form<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        match state.peek_kind() {
            Some(ClojureTokenType::ListStart) => self.parse_list(state),
            Some(ClojureTokenType::VectorStart) => self.parse_vector(state),
            Some(ClojureTokenType::MapStart) => self.parse_map(state),
            Some(ClojureTokenType::SetStart) => self.parse_set(state),
            Some(ClojureTokenType::AnonFnStart) => self.parse_anon_fn(state),
            Some(ClojureTokenType::Quote) => {
                let cp = state.checkpoint();
                state.bump();
                self.parse_form(state)?;
                state.finish_at(cp, ClojureElementType::Quotation);
                Ok(())
            }
            Some(ClojureTokenType::Meta) => {
                let cp = state.checkpoint();
                state.bump();
                self.parse_form(state)?;
                state.finish_at(cp, ClojureElementType::Meta);
                Ok(())
            }
            Some(_) => {
                state.bump();
                Ok(())
            }
            None => Ok(()),
        }
    }

    fn parse_list<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(ClojureTokenType::ListStart).ok();
        while state.not_at_end() && !state.at(ClojureTokenType::ListEnd) {
            self.parse_form(state)?;
        }
        state.expect(ClojureTokenType::ListEnd).ok();
        state.finish_at(cp, ClojureElementType::List);
        Ok(())
    }

    fn parse_vector<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(ClojureTokenType::VectorStart).ok();
        while state.not_at_end() && !state.at(ClojureTokenType::VectorEnd) {
            self.parse_form(state)?;
        }
        state.expect(ClojureTokenType::VectorEnd).ok();
        state.finish_at(cp, ClojureElementType::Vector);
        Ok(())
    }

    fn parse_map<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(ClojureTokenType::MapStart).ok();
        while state.not_at_end() && !state.at(ClojureTokenType::MapEnd) {
            self.parse_form(state)?;
        }
        state.expect(ClojureTokenType::MapEnd).ok();
        state.finish_at(cp, ClojureElementType::Map);
        Ok(())
    }

    fn parse_set<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(ClojureTokenType::SetStart).ok();
        while state.not_at_end() && !state.at(ClojureTokenType::MapEnd) {
            self.parse_form(state)?;
        }
        state.expect(ClojureTokenType::MapEnd).ok();
        state.finish_at(cp, ClojureElementType::Set);
        Ok(())
    }

    fn parse_anon_fn<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(ClojureTokenType::AnonFnStart).ok();
        while state.not_at_end() && !state.at(ClojureTokenType::ListEnd) {
            self.parse_form(state)?;
        }
        state.expect(ClojureTokenType::ListEnd).ok();
        state.finish_at(cp, ClojureElementType::AnonFn);
        Ok(())
    }
}

impl<'config> Parser<ClojureLanguage> for ClojureParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<ClojureLanguage>) -> ParseOutput<'a, ClojureLanguage> {
        let lexer = crate::lexer::ClojureLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                self.parse_form(state)?
            }

            Ok(state.finish_at(checkpoint, ClojureElementType::SourceFile))
        })
    }
}
