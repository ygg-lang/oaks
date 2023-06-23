#![doc = include_str!("readme.md")]
pub mod element_type;

pub use element_type::AdaElementType;

use crate::{language::AdaLanguage, lexer::AdaTokenType};
use oak_core::{
    OakError, TextEdit,
    parser::{Parser, ParserState},
    source::Source,
};

pub(crate) type State<'a, S> = ParserState<'a, AdaLanguage, S>;

pub struct AdaParser<'config> {
    pub(crate) config: &'config AdaLanguage,
}

impl<'config> AdaParser<'config> {
    pub fn new(config: &'config AdaLanguage) -> Self {
        Self { config }
    }

    pub(crate) fn parse_context_clause<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(AdaTokenType::With).ok();

        while state.at(AdaTokenType::Identifier) {
            state.advance();
            if state.at(AdaTokenType::Dot) {
                state.advance();
            }
            else {
                break;
            }
        }

        state.expect(AdaTokenType::Semicolon).ok();
        state.finish_at(cp, AdaElementType::ContextClause);
        Ok(())
    }

    pub(crate) fn parse_package_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(AdaTokenType::Package).ok();

        if state.at(AdaTokenType::Body) {
            state.advance();
        }

        state.expect(AdaTokenType::Identifier).ok();
        state.expect(AdaTokenType::Is).ok();

        while state.not_at_end() && !state.at(AdaTokenType::End) {
            if state.at(AdaTokenType::Procedure) || state.at(AdaTokenType::Function) {
                self.parse_subprogram_declaration(state)?;
            }
            else {
                state.advance();
            }
        }

        state.expect(AdaTokenType::End).ok();
        if state.at(AdaTokenType::Identifier) {
            state.advance();
        }
        state.expect(AdaTokenType::Semicolon).ok();

        state.finish_at(cp, AdaElementType::PackageDeclaration);
        Ok(())
    }

    pub(crate) fn parse_subprogram_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();

        if state.at(AdaTokenType::Procedure) {
            state.advance();
        }
        else {
            state.expect(AdaTokenType::Function).ok();
        }

        state.expect(AdaTokenType::Identifier).ok();

        // Parameters
        if state.at(AdaTokenType::LeftParen) {
            state.advance();
            while state.not_at_end() && !state.at(AdaTokenType::RightParen) {
                state.advance();
            }
            state.expect(AdaTokenType::RightParen).ok();
        }

        if state.at(AdaTokenType::Return) {
            state.advance();
            state.expect(AdaTokenType::Identifier).ok();
        }

        if state.at(AdaTokenType::Is) {
            state.advance();
            while state.not_at_end() && !state.at(AdaTokenType::End) {
                state.advance();
            }
            state.expect(AdaTokenType::End).ok();
            if state.at(AdaTokenType::Identifier) {
                state.advance();
            }
        }

        state.expect(AdaTokenType::Semicolon).ok();
        state.finish_at(cp, AdaElementType::SubprogramDeclaration);
        Ok(())
    }

    pub(crate) fn parse_pragma<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(AdaTokenType::Pragma).ok();
        state.expect(AdaTokenType::Identifier).ok();

        if state.at(AdaTokenType::LeftParen) {
            state.advance();
            while state.not_at_end() && !state.at(AdaTokenType::RightParen) {
                state.advance();
            }
            state.expect(AdaTokenType::RightParen).ok();
        }

        state.expect(AdaTokenType::Semicolon).ok();
        state.finish_at(cp, AdaElementType::Pragma);
        Ok(())
    }
}

impl<'config> Parser<AdaLanguage> for AdaParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl oak_core::ParseCache<AdaLanguage>) -> oak_core::ParseOutput<'a, AdaLanguage> {
        let lexer = crate::lexer::AdaLexer::new(self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() && !state.at(AdaTokenType::Eof) {
                if state.at(AdaTokenType::With) {
                    self.parse_context_clause(state)?
                }
                else if state.at(AdaTokenType::Package) {
                    self.parse_package_declaration(state)?
                }
                else if state.at(AdaTokenType::Procedure) || state.at(AdaTokenType::Function) {
                    self.parse_subprogram_declaration(state)?
                }
                else if state.at(AdaTokenType::Pragma) {
                    self.parse_pragma(state)?
                }
                else {
                    state.bump()
                }
            }

            Ok(state.finish_at(checkpoint, AdaElementType::Root))
        })
    }
}
