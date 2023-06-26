use crate::{language::VbNetLanguage, lexer::token_type::VbNetTokenType, parser::element_type::VbNetElementType};
use oak_core::{OakError, parser::ParserState, source::Source};

use super::{State, VbNetParser};

impl<'config> VbNetParser<'config> {
    pub(crate) fn parse_function<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(VbNetTokenType::Function)?;

        self.skip_trivia(state);

        state.expect(VbNetTokenType::Identifier)?;

        if state.at(VbNetTokenType::LeftParen) {
            state.bump();
            if state.at(VbNetTokenType::Of) {
                state.bump();
                while state.not_at_end() && !state.at(VbNetTokenType::RightParen) {
                    self.skip_trivia(state);

                    if state.at(VbNetTokenType::Identifier) {
                        state.bump();
                        if state.at(VbNetTokenType::As) {
                            state.bump();
                            while state.not_at_end() && (state.at(VbNetTokenType::Identifier) || state.at(VbNetTokenType::Dot)) {
                                if state.at(VbNetTokenType::Identifier) {
                                    state.bump();
                                }
                                else if state.at(VbNetTokenType::Dot) {
                                    state.bump();
                                }
                            }
                        }
                    }

                    self.skip_trivia(state);

                    if state.at(VbNetTokenType::Comma) {
                        state.bump();
                    }
                    else if !state.at(VbNetTokenType::RightParen) {
                        break;
                    }
                }
                state.expect(VbNetTokenType::RightParen)?;

                self.skip_trivia(state);

                if state.at(VbNetTokenType::LeftParen) {
                    state.bump();
                    while state.not_at_end() && !state.at(VbNetTokenType::RightParen) {
                        self.skip_trivia(state);

                        if state.at(VbNetTokenType::RightParen) {
                            break;
                        }

                        self.parse_parameter(state)?;

                        self.skip_trivia(state);

                        if state.at(VbNetTokenType::Comma) {
                            state.bump();
                        }
                        else if !state.at(VbNetTokenType::RightParen) {
                            break;
                        }
                    }
                    state.expect(VbNetTokenType::RightParen)?;
                }
            }
            else {
                while state.not_at_end() && !state.at(VbNetTokenType::RightParen) {
                    self.skip_trivia(state);

                    if state.at(VbNetTokenType::RightParen) {
                        break;
                    }

                    self.parse_parameter(state)?;

                    self.skip_trivia(state);

                    if state.at(VbNetTokenType::Comma) {
                        state.bump();
                    }
                    else if !state.at(VbNetTokenType::RightParen) {
                        break;
                    }
                }
                state.expect(VbNetTokenType::RightParen)?;
            }
        }

        if state.at(VbNetTokenType::As) {
            state.bump();
            let mut in_generic = false;
            while state.not_at_end() {
                if state.at(VbNetTokenType::Identifier) {
                    state.bump();
                }
                else if state.at(VbNetTokenType::LeftParen) {
                    state.bump();
                    in_generic = true;
                    if state.at(VbNetTokenType::Of) {
                        state.bump();
                        while state.not_at_end() && !state.at(VbNetTokenType::RightParen) {
                            if state.at(VbNetTokenType::Identifier) {
                                state.bump();
                            }
                            else if state.at(VbNetTokenType::Dot) {
                                state.bump();
                            }
                            else if state.at(VbNetTokenType::Comma) {
                                state.bump();
                            }
                            else if state.at(VbNetTokenType::Whitespace) {
                                state.bump();
                            }
                        }
                    }
                    state.expect(VbNetTokenType::RightParen)?;
                    in_generic = false;
                }
                else if state.at(VbNetTokenType::Dot) {
                    state.bump();
                }
                else if state.at(VbNetTokenType::Whitespace) {
                    state.bump();
                }
                else if !in_generic {
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
            state.eat(VbNetTokenType::Function);
        }

        state.finish_at(checkpoint, VbNetElementType::Function);
        Ok(())
    }

    pub(crate) fn parse_sub<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(VbNetTokenType::Sub)?;

        self.skip_trivia(state);

        state.expect(VbNetTokenType::Identifier)?;

        if state.at(VbNetTokenType::LeftParen) {
            state.bump();
            if state.at(VbNetTokenType::Of) {
                state.bump();
                while state.not_at_end() && !state.at(VbNetTokenType::RightParen) {
                    self.skip_trivia(state);

                    if state.at(VbNetTokenType::Identifier) {
                        state.bump();
                        if state.at(VbNetTokenType::As) {
                            state.bump();
                            while state.not_at_end() && (state.at(VbNetTokenType::Identifier) || state.at(VbNetTokenType::Dot)) {
                                if state.at(VbNetTokenType::Identifier) {
                                    state.bump();
                                }
                                else if state.at(VbNetTokenType::Dot) {
                                    state.bump();
                                }
                            }
                        }
                    }

                    self.skip_trivia(state);

                    if state.at(VbNetTokenType::Comma) {
                        state.bump();
                    }
                    else if !state.at(VbNetTokenType::RightParen) {
                        break;
                    }
                }
                state.expect(VbNetTokenType::RightParen)?;

                self.skip_trivia(state);

                if state.at(VbNetTokenType::LeftParen) {
                    state.bump();
                    while state.not_at_end() && !state.at(VbNetTokenType::RightParen) {
                        self.skip_trivia(state);

                        if state.at(VbNetTokenType::RightParen) {
                            break;
                        }

                        self.parse_parameter(state)?;

                        self.skip_trivia(state);

                        if state.at(VbNetTokenType::Comma) {
                            state.bump();
                        }
                        else if !state.at(VbNetTokenType::RightParen) {
                            break;
                        }
                    }
                    state.expect(VbNetTokenType::RightParen)?;
                }
            }
            else {
                while state.not_at_end() && !state.at(VbNetTokenType::RightParen) {
                    self.skip_trivia(state);

                    if state.at(VbNetTokenType::RightParen) {
                        break;
                    }

                    self.parse_parameter(state)?;

                    self.skip_trivia(state);

                    if state.at(VbNetTokenType::Comma) {
                        state.bump();
                    }
                    else if !state.at(VbNetTokenType::RightParen) {
                        break;
                    }
                }
                state.expect(VbNetTokenType::RightParen)?;
            }
        }

        if state.at(VbNetTokenType::LeftParen) {
            state.bump();
            while state.not_at_end() && !state.at(VbNetTokenType::RightParen) {
                self.parse_parameter(state)?;
                if state.at(VbNetTokenType::Comma) { state.bump() } else { break }
            }
            state.expect(VbNetTokenType::RightParen)?;
        }

        while state.not_at_end() && !state.at(VbNetTokenType::End) {
            self.skip_trivia(state);
            if state.not_at_end() && !state.at(VbNetTokenType::End) {
                self.parse_statement(state)?;
            }
        }

        if state.at(VbNetTokenType::End) {
            state.bump();
            state.eat(VbNetTokenType::Sub);
        }

        state.finish_at(checkpoint, VbNetElementType::Sub);
        Ok(())
    }

    pub(crate) fn parse_property<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(VbNetTokenType::Property)?;

        state.expect(VbNetTokenType::Identifier)?;

        if state.at(VbNetTokenType::LeftParen) {
            state.bump();
            while state.not_at_end() && !state.at(VbNetTokenType::RightParen) {
                self.parse_parameter(state)?;
                if state.at(VbNetTokenType::Comma) { state.bump() } else { break }
            }
            state.expect(VbNetTokenType::RightParen)?;
        }

        if state.at(VbNetTokenType::As) {
            state.bump();
            while state.not_at_end() && state.at(VbNetTokenType::Identifier) {
                state.bump();
                if state.at(VbNetTokenType::Dot) { state.bump() } else { break }
            }
        }

        while state.not_at_end() && !state.at(VbNetTokenType::End) {
            if state.at(VbNetTokenType::Get) {
                state.bump();
                while state.not_at_end() && !state.at(VbNetTokenType::Set) && !state.at(VbNetTokenType::End) {
                    self.parse_statement(state)?;
                }
            }
            else if state.at(VbNetTokenType::Set) {
                state.bump();
                if state.at(VbNetTokenType::Let) {
                    state.bump();
                }
                if state.at(VbNetTokenType::Identifier) {
                    state.bump();
                }
                while state.not_at_end() && !state.at(VbNetTokenType::Get) && !state.at(VbNetTokenType::End) {
                    self.parse_statement(state)?;
                }
            }
            else {
                self.parse_statement(state)?;
            }
        }

        if state.at(VbNetTokenType::End) {
            state.bump();
            state.eat(VbNetTokenType::Property);
        }

        state.finish_at(checkpoint, VbNetElementType::Property);
        Ok(())
    }

    pub(crate) fn parse_parameter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        self.skip_trivia(state);

        if state.at(VbNetTokenType::ByVal) || state.at(VbNetTokenType::ByRef) || state.at(VbNetTokenType::Optional) {
            state.bump();
            self.skip_trivia(state);
        }

        state.expect(VbNetTokenType::Identifier)?;

        if state.at(VbNetTokenType::As) {
            state.bump();
            while state.not_at_end() {
                if state.at(VbNetTokenType::Identifier) {
                    state.bump();
                }
                else if state.at(VbNetTokenType::LeftParen) {
                    state.bump();
                    if state.at(VbNetTokenType::Of) {
                        state.bump();
                        while state.not_at_end() && !state.at(VbNetTokenType::RightParen) {
                            while state.not_at_end() && !state.at(VbNetTokenType::RightParen) && !state.at(VbNetTokenType::Comma) {
                                if state.at(VbNetTokenType::Identifier) {
                                    state.bump();
                                }
                                else if state.at(VbNetTokenType::Dot) {
                                    state.bump();
                                }
                                else if state.at(VbNetTokenType::LeftParen) {
                                    state.bump();
                                    if state.at(VbNetTokenType::Of) {
                                        state.bump();
                                        while state.not_at_end() && !state.at(VbNetTokenType::RightParen) {
                                            if state.at(VbNetTokenType::Identifier) {
                                                state.bump();
                                            }
                                            else if state.at(VbNetTokenType::Dot) {
                                                state.bump();
                                            }
                                            else if state.at(VbNetTokenType::Comma) {
                                                state.bump();
                                            }
                                            else if state.at(VbNetTokenType::Whitespace) {
                                                state.bump();
                                            }
                                        }
                                    }
                                    state.expect(VbNetTokenType::RightParen)?;
                                }
                                else if state.at(VbNetTokenType::Whitespace) {
                                    state.bump();
                                }
                                else {
                                    break;
                                }
                            }
                            if state.at(VbNetTokenType::Comma) {
                                state.bump();
                            }
                            else {
                                break;
                            }
                        }
                    }
                    state.expect(VbNetTokenType::RightParen)?;
                }
                else if state.at(VbNetTokenType::Dot) {
                    state.bump();
                }
                else if state.at(VbNetTokenType::Whitespace) {
                    state.bump();
                }
                else {
                    break;
                }
            }
        }

        if state.at(VbNetTokenType::Equal) {
            state.bump();
            self.parse_expression(state)?;
        }

        Ok(())
    }

    pub(crate) fn parse_dim<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(VbNetTokenType::Dim)?;

        state.expect(VbNetTokenType::Identifier)?;

        if state.at(VbNetTokenType::As) {
            state.bump();
            while state.not_at_end() && (state.at(VbNetTokenType::Identifier) || state.at(VbNetTokenType::LeftParen)) {
                if state.at(VbNetTokenType::Identifier) {
                    state.bump();
                }
                else if state.at(VbNetTokenType::LeftParen) {
                    state.bump();
                    if state.at(VbNetTokenType::Of) {
                        state.bump();
                        while state.not_at_end() && (state.at(VbNetTokenType::Identifier) || state.at(VbNetTokenType::Dot) || state.at(VbNetTokenType::Comma)) {
                            if state.at(VbNetTokenType::Identifier) {
                                state.bump();
                            }
                            else if state.at(VbNetTokenType::Dot) {
                                state.bump();
                            }
                            else if state.at(VbNetTokenType::Comma) {
                                state.bump();
                            }
                        }
                    }
                    state.expect(VbNetTokenType::RightParen)?;
                }
                if state.at(VbNetTokenType::Dot) { state.bump() } else { break }
            }
        }

        if state.at(VbNetTokenType::Equal) {
            state.bump();
            self.parse_expression(state)?;
        }

        state.finish_at(checkpoint, VbNetElementType::Dim);
        Ok(())
    }

    pub(crate) fn parse_const<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(VbNetTokenType::Const)?;

        state.expect(VbNetTokenType::Identifier)?;

        if state.at(VbNetTokenType::As) {
            state.bump();
            while state.not_at_end() && state.at(VbNetTokenType::Identifier) {
                state.bump();
                if state.at(VbNetTokenType::Dot) { state.bump() } else { break }
            }
        }

        state.expect(VbNetTokenType::Equal)?;
        self.parse_expression(state)?;

        state.finish_at(checkpoint, VbNetElementType::Const);
        Ok(())
    }
}
