use crate::{language::VbNetLanguage, lexer::token_type::VbNetTokenType, parser::element_type::VbNetElementType};
use oak_core::{OakError, parser::ParserState, source::Source};

use super::{State, VbNetParser};

impl<'config> VbNetParser<'config> {
    pub(crate) fn parse_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        self.parse_assignment_expression(state)?;
        Ok(())
    }

    pub(crate) fn parse_assignment_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();

        self.parse_binary_expression(state)?;

        if state.at(VbNetTokenType::Equal) {
            state.bump();
            self.parse_assignment_expression(state)?;
            state.finish_at(checkpoint, VbNetElementType::AssignmentExpression);
        }

        Ok(())
    }

    pub(crate) fn parse_binary_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();

        self.parse_unary_expression(state)?;

        while let Some(kind) = state.peek_kind() {
            match kind {
                VbNetTokenType::Plus
                | VbNetTokenType::Minus
                | VbNetTokenType::Star
                | VbNetTokenType::Slash
                | VbNetTokenType::Backslash
                | VbNetTokenType::Percent
                | VbNetTokenType::Caret
                | VbNetTokenType::Equal
                | VbNetTokenType::NotEqual
                | VbNetTokenType::LessThan
                | VbNetTokenType::LessEqual
                | VbNetTokenType::GreaterThan
                | VbNetTokenType::GreaterEqual
                | VbNetTokenType::And
                | VbNetTokenType::Or
                | VbNetTokenType::Xor
                | VbNetTokenType::AndAlso
                | VbNetTokenType::OrElse
                | VbNetTokenType::Is
                | VbNetTokenType::IsNot
                | VbNetTokenType::Like
                | VbNetTokenType::Ampersand => {
                    state.bump();
                    self.parse_unary_expression(state)?;
                    state.finish_at(checkpoint, VbNetElementType::BinaryExpression);
                }
                _ => break,
            }
        }

        Ok(())
    }

    pub(crate) fn parse_unary_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();

        if let Some(kind) = state.peek_kind() {
            match kind {
                VbNetTokenType::Plus | VbNetTokenType::Minus | VbNetTokenType::Not => {
                    state.bump();
                    self.parse_primary_expression(state)?;
                    state.finish_at(checkpoint, VbNetElementType::UnaryExpression);
                    return Ok(());
                }
                VbNetTokenType::TypeOf => {
                    state.bump();
                    self.parse_primary_expression(state)?;
                    if state.at(VbNetTokenType::Is) {
                        state.bump();
                        self.parse_primary_expression(state)?;
                        state.finish_at(checkpoint, VbNetElementType::TypeOfExpression);
                    }
                    return Ok(());
                }
                _ => {}
            }
        }

        self.parse_primary_expression(state)?;
        Ok(())
    }

    pub(crate) fn parse_primary_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        let kind = state.peek_kind();

        match kind {
            Some(VbNetTokenType::Await) => {
                state.bump();
                self.parse_expression(state)?;
                state.finish_at(checkpoint, VbNetElementType::Expression);
            }
            Some(VbNetTokenType::Function) | Some(VbNetTokenType::Sub) => {
                state.bump();
                if state.at(VbNetTokenType::LeftParen) {
                    state.bump();
                    while state.not_at_end() && !state.at(VbNetTokenType::RightParen) {
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
                        else {
                            break;
                        }
                    }
                    if state.at(VbNetTokenType::RightParen) {
                        state.bump();
                    }
                }
                self.parse_expression(state)?;
                state.finish_at(checkpoint, VbNetElementType::LambdaExpression);
            }
            Some(VbNetTokenType::Identifier) => {
                state.bump();
                if state.at(VbNetTokenType::LeftParen) {
                    state.bump();
                    while state.at(VbNetTokenType::Whitespace) {
                        state.bump();
                    }
                    if state.at(VbNetTokenType::Of) {
                        state.bump();
                        while state.not_at_end() {
                            if state.at(VbNetTokenType::RightParen) {
                                state.bump();
                                break;
                            }
                            if state.at(VbNetTokenType::Identifier) {
                                state.bump();
                            }
                            else if state.at(VbNetTokenType::Dot) {
                                state.bump();
                            }
                            else if state.at(VbNetTokenType::Comma) {
                                state.bump();
                            }
                            else {
                                break;
                            }
                        }
                    }
                    else {
                        while state.not_at_end() && !state.at(VbNetTokenType::RightParen) {
                            self.parse_expression(state)?;
                            if state.at(VbNetTokenType::Comma) { state.bump() } else { break }
                        }
                        if state.at(VbNetTokenType::RightParen) {
                            state.bump();
                        }
                    }
                    state.finish_at(checkpoint, VbNetElementType::MethodCall);
                }
                else if state.at(VbNetTokenType::Dot) {
                    state.bump();
                    if state.at(VbNetTokenType::Identifier) {
                        state.bump();
                        if state.at(VbNetTokenType::LeftParen) {
                            state.bump();
                            while state.not_at_end() && !state.at(VbNetTokenType::RightParen) {
                                self.parse_expression(state)?;
                                if state.at(VbNetTokenType::Comma) { state.bump() } else { break }
                            }
                            state.expect(VbNetTokenType::RightParen)?;
                            state.finish_at(checkpoint, VbNetElementType::MethodCall);
                        }
                        else {
                            state.finish_at(checkpoint, VbNetElementType::MemberAccess);
                        }
                    }
                }
                else if state.at(VbNetTokenType::LeftBracket) {
                    state.bump();
                    self.parse_expression(state)?;
                    state.expect(VbNetTokenType::RightBracket)?;
                    state.finish_at(checkpoint, VbNetElementType::ElementAccess);
                }
                else {
                    state.finish_at(checkpoint, VbNetElementType::Identifier);
                }
            }
            Some(VbNetTokenType::IntegerLiteral)
            | Some(VbNetTokenType::FloatLiteral)
            | Some(VbNetTokenType::StringLiteral)
            | Some(VbNetTokenType::CharLiteral)
            | Some(VbNetTokenType::BooleanLiteral)
            | Some(VbNetTokenType::DateLiteral)
            | Some(VbNetTokenType::NothingLiteral) => {
                state.bump();
                state.finish_at(checkpoint, VbNetElementType::Expression);
            }
            Some(VbNetTokenType::LeftParen) => {
                state.bump();
                self.parse_expression(state)?;
                state.expect(VbNetTokenType::RightParen)?;
                state.finish_at(checkpoint, VbNetElementType::ParenthesizedExpression);
            }
            Some(VbNetTokenType::New) => {
                state.bump();
                while state.not_at_end() && state.at(VbNetTokenType::Identifier) {
                    state.bump();
                    if state.at(VbNetTokenType::Dot) { state.bump() } else { break }
                }
                if state.at(VbNetTokenType::LeftParen) {
                    state.bump();
                    while state.at(VbNetTokenType::Whitespace) {
                        state.bump();
                    }
                    if state.at(VbNetTokenType::Of) {
                        state.bump();
                        while state.not_at_end() {
                            if state.at(VbNetTokenType::RightParen) {
                                state.bump();
                                break;
                            }
                            if state.at(VbNetTokenType::Identifier) {
                                state.bump();
                            }
                            else if state.at(VbNetTokenType::Dot) {
                                state.bump();
                            }
                            else if state.at(VbNetTokenType::Comma) {
                                state.bump();
                            }
                            else {
                                break;
                            }
                        }
                    }
                    else {
                        while state.not_at_end() && !state.at(VbNetTokenType::RightParen) {
                            self.parse_expression(state)?;
                            if state.at(VbNetTokenType::Comma) { state.bump() } else { break }
                        }
                        if state.at(VbNetTokenType::RightParen) {
                            state.bump();
                        }
                    }
                }
                state.finish_at(checkpoint, VbNetElementType::NewExpression);
            }
            Some(VbNetTokenType::Me) | Some(VbNetTokenType::MyBase) | Some(VbNetTokenType::MyClass) => {
                state.bump();
                state.finish_at(checkpoint, VbNetElementType::Expression);
            }
            _ => {
                if state.not_at_end() {
                    state.bump();
                    state.finish_at(checkpoint, VbNetElementType::Error);
                }
            }
        }

        Ok(())
    }

    pub(crate) fn parse_linq<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(VbNetTokenType::From)?;

        while state.not_at_end() {
            if state.at(VbNetTokenType::Select) {
                state.bump();
                while state.not_at_end() && !state.at(VbNetTokenType::End) && !state.at(VbNetTokenType::Return) {
                    state.bump();
                }
                break;
            }
            else if state.at(VbNetTokenType::Where) {
                state.bump();
                while state.not_at_end() && !state.at(VbNetTokenType::Select) && !state.at(VbNetTokenType::Group) && !state.at(VbNetTokenType::Join) && !state.at(VbNetTokenType::Let) {
                    state.bump();
                }
            }
            else if state.at(VbNetTokenType::Group) {
                state.bump();
                if state.at(VbNetTokenType::By) {
                    state.bump();
                    while state.not_at_end() && !state.at(VbNetTokenType::Into) && !state.at(VbNetTokenType::Select) {
                        state.bump();
                    }
                    if state.at(VbNetTokenType::Into) {
                        state.bump();
                        while state.not_at_end() && !state.at(VbNetTokenType::Select) {
                            state.bump();
                        }
                    }
                }
            }
            else if state.at(VbNetTokenType::Join) {
                state.bump();
                while state.not_at_end() && !state.at(VbNetTokenType::On) {
                    state.bump();
                }
                if state.at(VbNetTokenType::On) {
                    state.bump();
                    while state.not_at_end() && !state.at(VbNetTokenType::Select) && !state.at(VbNetTokenType::Where) && !state.at(VbNetTokenType::Group) {
                        state.bump();
                    }
                }
            }
            else if state.at(VbNetTokenType::Let) {
                state.bump();
                while state.not_at_end() && !state.at(VbNetTokenType::Select) && !state.at(VbNetTokenType::Where) && !state.at(VbNetTokenType::Group) && !state.at(VbNetTokenType::Join) {
                    state.bump();
                }
            }
            else if state.at(VbNetTokenType::Order) {
                state.bump();
                if state.at(VbNetTokenType::By) {
                    state.bump();
                    while state.not_at_end() && !state.at(VbNetTokenType::Select) {
                        state.bump();
                    }
                }
            }
            else {
                state.bump();
            }
        }

        state.finish_at(checkpoint, VbNetElementType::Expression);
        Ok(())
    }
}
