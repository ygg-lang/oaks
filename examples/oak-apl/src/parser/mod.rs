#![doc = include_str!("readme.md")]
/// Element type definitions.
pub mod element_type;

pub use element_type::AplElementType;

use crate::{language::AplLanguage, lexer::token_type::AplTokenType};
use oak_core::{
    OakError, TextEdit,
    parser::{Parser, ParserState},
    source::Source,
};

pub(crate) type State<'a, S> = ParserState<'a, AplLanguage, S>;

/// Parser for the APL language.
pub struct AplParser<'config> {
    /// The language configuration.
    pub(crate) config: &'config AplLanguage,
}

impl<'config> AplParser<'config> {
    /// Creates a new `AplParser`.
    pub fn new(config: &'config AplLanguage) -> Self {
        Self { config }
    }

    /// Parses an APL statement.
    pub(crate) fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();

        // Try parsing assignment: ID ← Expression
        if state.at(AplTokenType::Identifier) && state.peek_kind_at(1) == Some(AplTokenType::LeftArrow) {
            state.advance(); // ID
            state.advance(); // ←
            self.parse_expression(state)?;
            state.finish_at(cp, AplElementType::Assignment);
        }
        else {
            self.parse_expression(state)?;
            state.finish_at(cp, AplElementType::Statement);
        }

        Ok(())
    }

    /// Parses an APL expression (simplified).
    pub(crate) fn parse_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();

        while state.not_at_end() && !state.at(AplTokenType::Newline) && !state.at(AplTokenType::Diamond) {
            if state.at(AplTokenType::Identifier) {
                state.advance();
            }
            else if state.at(AplTokenType::NumberLiteral) {
                state.advance();
            }
            else if state.at(AplTokenType::StringLiteral) {
                state.advance();
            }
            else if state.at(AplTokenType::LeftParen) {
                state.advance();
                self.parse_expression(state)?;
                state.expect(AplTokenType::RightParen).ok();
            }
            else if state.at(AplTokenType::LeftBrace) {
                // Dfn { ... }
                state.advance();
                while state.not_at_end() && !state.at(AplTokenType::RightBrace) {
                    state.advance();
                }
                state.expect(AplTokenType::RightBrace).ok();
            }
            else {
                // Possibly a primitive function or operator
                state.advance();
            }
        }

        state.finish_at(cp, AplElementType::Expression);
        Ok(())
    }
}

impl<'config> Parser<AplLanguage> for AplParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl oak_core::ParseCache<AplLanguage>) -> oak_core::ParseOutput<'a, AplLanguage> {
        let lexer = crate::lexer::AplLexer::new(self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() && !state.at(AplTokenType::Eof) {
                if state.at(AplTokenType::Newline) || state.at(AplTokenType::Whitespace) {
                    state.advance();
                    continue;
                }

                if state.at(AplTokenType::Comment) {
                    state.advance();
                    continue;
                }

                self.parse_statement(state)?;

                if state.at(AplTokenType::Diamond) {
                    state.advance();
                }
            }

            Ok(state.finish_at(checkpoint, AplElementType::Root))
        })
    }
}
