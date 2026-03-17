#![doc = include_str!("readme.md")]

/// Element type definitions for Rhombus parser.
pub mod element_type;

use crate::{
    language::RhombusLanguage,
    lexer::{RhombusLexer, token_type::RhombusTokenType},
    parser::element_type::RhombusElementType,
};
use oak_core::{
    OakError,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, RhombusLanguage, S>;

/// Parser for the Rhombus language.
pub struct RhombusParser<'config> {
    pub(crate) config: &'config RhombusLanguage,
}

impl<'config> RhombusParser<'config> {
    /// Creates a new `RhombusParser` with the given configuration.
    pub fn new(config: &'config RhombusLanguage) -> Self {
        Self { config }
    }

    fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();

        // Rhombus statements often end with a newline or are grouped in blocks
        self.parse_expression(state)?;

        state.finish_at(cp, RhombusElementType::Statement);
        Ok(())
    }

    fn parse_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        match state.peek_kind() {
            Some(RhombusTokenType::LeftParen) | Some(RhombusTokenType::LeftBracket) | Some(RhombusTokenType::LeftBrace) => self.parse_block(state),
            Some(_) => {
                state.bump();
                Ok(())
            }
            None => Ok(()),
        }
    }

    fn parse_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        let open_kind = state.peek_kind();
        let close_kind = match open_kind {
            Some(RhombusTokenType::LeftParen) => Some(RhombusTokenType::RightParen),
            Some(RhombusTokenType::LeftBracket) => Some(RhombusTokenType::RightBracket),
            Some(RhombusTokenType::LeftBrace) => Some(RhombusTokenType::RightBrace),
            _ => None,
        };

        state.bump();

        // Check for special forms
        if let Some(token_type) = state.peek_kind() {
            match token_type {
                RhombusTokenType::Require => {
                    state.bump();
                    while state.not_at_end() {
                        if let Some(ck) = close_kind {
                            if state.at(ck) {
                                break;
                            }
                        }
                        self.parse_statement(state)?;
                    }
                    if let Some(ck) = close_kind {
                        state.expect(ck).ok();
                    }
                    state.finish_at(cp, RhombusElementType::Identifier); // Using Identifier for now
                    return Ok(());
                }
                RhombusTokenType::Provide => {
                    state.bump();
                    while state.not_at_end() {
                        if let Some(ck) = close_kind {
                            if state.at(ck) {
                                break;
                            }
                        }
                        self.parse_statement(state)?;
                    }
                    if let Some(ck) = close_kind {
                        state.expect(ck).ok();
                    }
                    state.finish_at(cp, RhombusElementType::Identifier); // Using Identifier for now
                    return Ok(());
                }
                _ => {}
            }
        }

        while state.not_at_end() {
            if let Some(ck) = close_kind {
                if state.at(ck) {
                    break;
                }
            }
            self.parse_statement(state)?;
        }
        if let Some(ck) = close_kind {
            state.expect(ck).ok();
        }
        state.finish_at(cp, RhombusElementType::Block);
        Ok(())
    }
}

impl<'config> Parser<RhombusLanguage> for RhombusParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<RhombusLanguage>) -> ParseOutput<'a, RhombusLanguage> {
        let lexer = RhombusLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                self.parse_statement(state)?
            }

            Ok(state.finish_at(checkpoint, RhombusElementType::SourceFile))
        })
    }
}
