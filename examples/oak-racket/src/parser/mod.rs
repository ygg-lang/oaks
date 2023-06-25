//! Parser implementation for the Racket language.

pub mod element_type;

use crate::{
    language::RacketLanguage,
    lexer::{RacketLexer, token_type::RacketTokenType},
    parser::element_type::RacketElementType,
};
use oak_core::{
    OakError,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, RacketLanguage, S>;

/// Parser for the Racket language.
pub struct RacketParser<'config> {
    pub(crate) config: &'config RacketLanguage,
}

impl<'config> RacketParser<'config> {
    /// Creates a new `RacketParser` with the given configuration.
    pub fn new(config: &'config RacketLanguage) -> Self {
        Self { config }
    }

    fn parse_form<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        match state.peek_kind() {
            Some(RacketTokenType::LeftParen) | Some(RacketTokenType::LeftBracket) | Some(RacketTokenType::LeftBrace) => self.parse_list(state),
            Some(RacketTokenType::Quote_) | Some(RacketTokenType::Quasiquote_) | Some(RacketTokenType::Unquote_) | Some(RacketTokenType::UnquoteSplicing_) => {
                let cp = state.checkpoint();
                state.bump();
                self.parse_form(state)?;
                state.finish_at(cp, RacketElementType::Quotation);
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
        let open_kind = state.peek_kind();
        let close_kind = match open_kind {
            Some(RacketTokenType::LeftParen) => Some(RacketTokenType::RightParen),
            Some(RacketTokenType::LeftBracket) => Some(RacketTokenType::RightBracket),
            Some(RacketTokenType::LeftBrace) => Some(RacketTokenType::RightBrace),
            _ => None,
        };

        state.bump();
        while state.not_at_end() {
            if let Some(ck) = close_kind {
                if state.at(ck) {
                    break;
                }
            }
            self.parse_form(state)?;
        }
        if let Some(ck) = close_kind {
            state.expect(ck).ok();
        }
        state.finish_at(cp, RacketElementType::List);
        Ok(())
    }
}

impl<'config> Parser<RacketLanguage> for RacketParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<RacketLanguage>) -> ParseOutput<'a, RacketLanguage> {
        let lexer = RacketLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                self.parse_form(state)?
            }

            Ok(state.finish_at(checkpoint, RacketElementType::SourceFile))
        })
    }
}
