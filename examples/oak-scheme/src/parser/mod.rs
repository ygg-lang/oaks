#![doc = include_str!("readme.md")]

pub mod element_type;

use crate::{
    language::SchemeLanguage,
    lexer::{SchemeLexer, token_type::SchemeTokenType},
    parser::element_type::SchemeElementType,
};
use oak_core::{
    OakError,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, SchemeLanguage, S>;

/// Parser for the Scheme language.
pub struct SchemeParser<'config> {
    pub(crate) config: &'config SchemeLanguage,
}

impl<'config> SchemeParser<'config> {
    /// Creates a new `SchemeParser` with the given configuration.
    pub fn new(config: &'config SchemeLanguage) -> Self {
        Self { config }
    }

    fn parse_form<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        match state.peek_kind() {
            Some(SchemeTokenType::LeftParen) => self.parse_list(state),
            Some(SchemeTokenType::Quote_) | Some(SchemeTokenType::Quasiquote_) | Some(SchemeTokenType::Unquote_) | Some(SchemeTokenType::UnquoteSplicing_) => {
                let cp = state.checkpoint();
                state.bump();
                self.parse_form(state)?;
                state.finish_at(cp, SchemeElementType::Quotation);
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
        state.expect(SchemeTokenType::LeftParen).ok();
        while state.not_at_end() && !state.at(SchemeTokenType::RightParen) {
            self.parse_form(state)?;
        }
        state.expect(SchemeTokenType::RightParen).ok();
        state.finish_at(cp, SchemeElementType::List);
        Ok(())
    }
}

impl<'config> Parser<SchemeLanguage> for SchemeParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<SchemeLanguage>) -> ParseOutput<'a, SchemeLanguage> {
        let lexer = SchemeLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                self.parse_form(state)?
            }

            Ok(state.finish_at(checkpoint, SchemeElementType::SourceFile))
        })
    }
}
