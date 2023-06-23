use crate::{
    language::RLanguage,
    lexer::token_type::RTokenType,
    parser::{RParser, State, element_type::RElementType},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> RParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, RLanguage>, OakError> {
        let checkpoint = state.checkpoint();
        while state.not_at_end() {
            self.parse_statement(state);
            while state.at(RTokenType::Newline) || state.at(RTokenType::Semicolon) {
                state.bump()
            }
        }
        Ok(state.finish_at(checkpoint, RElementType::Root))
    }

    fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        if state.at(RTokenType::Identifier) && state.peek_at(1).map(|t| t.kind) == Some(RTokenType::LeftArrow) {
            let checkpoint = state.checkpoint();
            state.bump(); // ident
            state.bump(); // <-
            self.parse_expression(state);
            state.finish_at(checkpoint, RElementType::Assignment);
        }
        else if state.at(RTokenType::Function) {
            let checkpoint = state.checkpoint();
            state.bump(); // function
            if state.at(RTokenType::LeftParen) {
                state.bump();
                while state.not_at(RTokenType::RightParen) && state.not_at_end() {
                    if state.at(RTokenType::Identifier) {
                        state.bump();
                    }
                    if state.at(RTokenType::Comma) {
                        state.bump();
                    }
                }
                if state.at(RTokenType::RightParen) {
                    state.bump();
                }
            }
            if state.at(RTokenType::LeftBrace) {
                let body_checkpoint = state.checkpoint();
                state.bump();
                while state.not_at(RTokenType::RightBrace) && state.not_at_end() {
                    self.parse_statement(state);
                    while state.at(RTokenType::Newline) || state.at(RTokenType::Semicolon) {
                        state.bump();
                    }
                }
                if state.at(RTokenType::RightBrace) {
                    state.bump();
                }
                state.finish_at(body_checkpoint, RElementType::BlockExpression);
            }
            state.finish_at(checkpoint, RElementType::Function);
        }
        else {
            self.parse_expression(state);
        }
    }

    fn parse_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let checkpoint = state.checkpoint();
        if state.at(RTokenType::Identifier) {
            state.bump();
            if state.at(RTokenType::LeftParen) {
                state.bump();
                while state.not_at(RTokenType::RightParen) && state.not_at_end() {
                    self.parse_expression(state);
                    if state.at(RTokenType::Comma) {
                        state.bump();
                    }
                }
                if state.at(RTokenType::RightParen) {
                    state.bump();
                }
                state.finish_at(checkpoint, RElementType::CallExpression);
            }
            else {
                state.finish_at(checkpoint, RElementType::IdentifierExpression);
            }
        }
        else if state.at(RTokenType::IntegerLiteral) || state.at(RTokenType::FloatLiteral) || state.at(RTokenType::StringLiteral) || state.at(RTokenType::BooleanLiteral) {
            state.bump();
            state.finish_at(checkpoint, RElementType::LiteralExpression);
        }
        else {
            state.bump();
        }
    }
}
