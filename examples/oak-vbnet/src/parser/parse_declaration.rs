use crate::{language::VbNetLanguage, lexer::token_type::VbNetTokenType, parser::element_type::VbNetElementType};
use oak_core::{OakError, parser::ParserState, source::Source};

use super::{State, VbNetParser};

impl<'config> VbNetParser<'config> {
    pub(crate) fn parse_namespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        self.skip_trivia(state);
        let checkpoint = state.checkpoint();
        state.expect(VbNetTokenType::Namespace)?;

        while state.not_at_end() && state.at(VbNetTokenType::Identifier) {
            state.bump();
            if state.at(VbNetTokenType::Dot) { state.bump() } else { break }
        }

        if state.at(VbNetTokenType::LeftBrace) {
            state.bump();
            while state.not_at_end() && !state.at(VbNetTokenType::RightBrace) {
                self.parse_statement(state)?;
            }
            state.expect(VbNetTokenType::RightBrace)?;
        }

        state.finish_at(checkpoint, VbNetElementType::Namespace);
        Ok(())
    }

    pub(crate) fn parse_imports<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        self.skip_trivia(state);
        let checkpoint = state.checkpoint();
        state.expect(VbNetTokenType::Imports)?;

        while state.not_at_end() && state.at(VbNetTokenType::Identifier) {
            state.bump();
            if state.at(VbNetTokenType::Dot) { state.bump() } else { break }
        }

        if state.at(VbNetTokenType::Equal) {
            state.bump();
            state.expect(VbNetTokenType::Identifier)?;
        }

        state.finish_at(checkpoint, VbNetElementType::Imports);
        Ok(())
    }

    pub(crate) fn parse_class<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        self.skip_trivia(state);
        let checkpoint = state.checkpoint();
        state.expect(VbNetTokenType::Class)?;

        state.expect(VbNetTokenType::Identifier)?;

        if state.at(VbNetTokenType::LeftParen) {
            state.bump();
            if state.at(VbNetTokenType::Of) {
                state.bump();
                while state.not_at_end() {
                    if state.at(VbNetTokenType::RightParen) {
                        state.bump();
                        break;
                    }
                    if state.at(VbNetTokenType::Identifier) {
                        state.bump();
                        if state.at(VbNetTokenType::As) {
                            state.bump();
                            while state.not_at_end() && state.at(VbNetTokenType::Identifier) {
                                state.bump();
                                if state.at(VbNetTokenType::Dot) {
                                    state.bump()
                                }
                            }
                        }
                    }
                    if state.at(VbNetTokenType::Comma) {
                        state.bump();
                    }
                    else if !state.at(VbNetTokenType::RightParen) {
                        break;
                    }
                }
            }
            else {
                if state.at(VbNetTokenType::RightParen) {
                    state.bump();
                }
            }
        }

        if state.at(VbNetTokenType::Inherits) {
            state.bump();
            while state.not_at_end() && state.at(VbNetTokenType::Identifier) {
                state.bump();
                if state.at(VbNetTokenType::Dot) { state.bump() } else { break }
            }
        }

        if state.at(VbNetTokenType::Implements) {
            state.bump();
            while state.not_at_end() && state.at(VbNetTokenType::Identifier) {
                state.bump();
                if state.at(VbNetTokenType::Dot) {
                    state.bump()
                }
                else if state.at(VbNetTokenType::Comma) {
                    state.bump()
                }
                else {
                    break;
                }
            }
        }

        while state.not_at_end() && !state.at(VbNetTokenType::End) {
            self.skip_trivia(state);
            if state.not_at_end() && !state.at(VbNetTokenType::End) {
                self.parse_statement(state)?;
            }
        }

        if state.at(VbNetTokenType::End) {
            state.bump();
            state.eat(VbNetTokenType::Class);
        }

        state.finish_at(checkpoint, VbNetElementType::Class);
        Ok(())
    }

    pub(crate) fn parse_interface<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        self.skip_trivia(state);
        let checkpoint = state.checkpoint();
        state.expect(VbNetTokenType::Interface)?;

        state.expect(VbNetTokenType::Identifier)?;

        if state.at(VbNetTokenType::Of) {
            state.bump();
            while state.not_at_end() {
                if state.at(VbNetTokenType::RightParen) {
                    break;
                }
                if state.at(VbNetTokenType::Identifier) {
                    state.bump();
                    if state.at(VbNetTokenType::As) {
                        state.bump();
                        while state.not_at_end() && state.at(VbNetTokenType::Identifier) {
                            state.bump();
                            if state.at(VbNetTokenType::Dot) {
                                state.bump()
                            }
                        }
                    }
                }
                if state.at(VbNetTokenType::Comma) {
                    state.bump();
                }
                else if !state.at(VbNetTokenType::RightParen) {
                    break;
                }
            }
        }

        if state.at(VbNetTokenType::Inherits) {
            state.bump();
            while state.not_at_end() && state.at(VbNetTokenType::Identifier) {
                state.bump();
                if state.at(VbNetTokenType::Dot) {
                    state.bump()
                }
                else if state.at(VbNetTokenType::Comma) {
                    state.bump()
                }
                else {
                    break;
                }
            }
        }

        while state.not_at_end() && !state.at(VbNetTokenType::End) {
            self.parse_statement(state)?;
        }

        if state.at(VbNetTokenType::End) {
            state.bump();
            state.eat(VbNetTokenType::Interface);
        }

        state.finish_at(checkpoint, VbNetElementType::Interface);
        Ok(())
    }

    pub(crate) fn parse_structure<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        self.skip_trivia(state);
        let checkpoint = state.checkpoint();
        state.expect(VbNetTokenType::Structure)?;

        state.expect(VbNetTokenType::Identifier)?;

        while state.not_at_end() && !state.at(VbNetTokenType::End) {
            self.parse_statement(state)?;
        }

        if state.at(VbNetTokenType::End) {
            state.bump();
            state.eat(VbNetTokenType::Structure);
        }

        state.finish_at(checkpoint, VbNetElementType::Structure);
        Ok(())
    }

    pub(crate) fn parse_enum<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        self.skip_trivia(state);
        let checkpoint = state.checkpoint();
        state.expect(VbNetTokenType::Enum)?;

        state.expect(VbNetTokenType::Identifier)?;

        while state.not_at_end() && !state.at(VbNetTokenType::End) {
            if state.at(VbNetTokenType::Identifier) {
                state.bump();
                if state.at(VbNetTokenType::Equal) {
                    state.bump();
                    self.parse_expression(state)?;
                }
            }
            self.skip_trivia(state);
        }

        if state.at(VbNetTokenType::End) {
            state.bump();
            state.eat(VbNetTokenType::Enum);
        }

        state.finish_at(checkpoint, VbNetElementType::Enum);
        Ok(())
    }

    pub(crate) fn parse_module<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        self.skip_trivia(state);
        let checkpoint = state.checkpoint();
        state.expect(VbNetTokenType::Module)?;

        state.expect(VbNetTokenType::Identifier)?;

        while state.not_at_end() && !state.at(VbNetTokenType::End) {
            self.parse_statement(state)?;
        }

        if state.at(VbNetTokenType::End) {
            state.bump();
            state.eat(VbNetTokenType::Module);
        }

        state.finish_at(checkpoint, VbNetElementType::Module);
        Ok(())
    }
}
