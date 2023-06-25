pub mod element_type;

use crate::{language::JinjaLanguage, lexer::token_type::JinjaTokenType};
pub use element_type::JinjaElementType;
use oak_core::{
    errors::OakError,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, JinjaLanguage, S>;

pub struct JinjaParser<'config> {
    language: &'config JinjaLanguage,
}

impl<'config> JinjaParser<'config> {
    pub fn new(language: &'config JinjaLanguage) -> Self {
        Self { language }
    }

    fn parse_node<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        match state.peek_kind() {
            Some(JinjaTokenType::VariableStart) => self.parse_variable(state),
            Some(JinjaTokenType::TagStart) => self.parse_tag_statement(state),
            Some(JinjaTokenType::Comment) => {
                let cp = state.checkpoint();
                state.advance();
                state.finish_at(cp, JinjaElementType::Comment);
                Ok(())
            }
            Some(JinjaTokenType::HtmlContent) => {
                let cp = state.checkpoint();
                state.advance();
                state.finish_at(cp, JinjaElementType::HtmlContent);
                Ok(())
            }
            _ => {
                state.advance();
                Ok(())
            }
        }
    }

    fn parse_variable<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(JinjaTokenType::VariableStart)?;
        self.parse_expression(state)?;
        state.expect(JinjaTokenType::VariableEnd)?;
        state.finish_at(cp, JinjaElementType::Variable);
        Ok(())
    }

    fn parse_tag_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(JinjaTokenType::TagStart)?;

        let kind = state.peek_kind();
        match kind {
            Some(JinjaTokenType::If) => self.parse_if_statement(state, cp),
            Some(JinjaTokenType::For) => self.parse_for_statement(state, cp),
            Some(JinjaTokenType::Block) => self.parse_block_statement(state, cp),
            Some(JinjaTokenType::Macro) => self.parse_macro_definition(state, cp),
            Some(JinjaTokenType::Set) => self.parse_set_statement(state, cp),
            _ => {
                while state.not_at_end() && !state.at(JinjaTokenType::TagEnd) {
                    state.advance();
                }
                state.expect(JinjaTokenType::TagEnd)?;
                state.finish_at(cp, JinjaElementType::Tag);
                Ok(())
            }
        }
    }

    fn parse_if_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) -> Result<(), OakError> {
        state.expect(JinjaTokenType::If)?;
        self.parse_expression(state)?;
        state.expect(JinjaTokenType::TagEnd)?;

        while state.not_at_end() {
            if state.at(JinjaTokenType::TagStart) {
                let next_kind = state.peek_kind_at(1);
                if matches!(next_kind, Some(JinjaTokenType::Endif) | Some(JinjaTokenType::Elif) | Some(JinjaTokenType::Else)) {
                    break;
                }
            }
            self.parse_node(state)?;
        }

        match state.peek_kind_at(1) {
            Some(JinjaTokenType::Elif) => {
                state.expect(JinjaTokenType::TagStart)?;
                self.parse_if_statement(state, state.checkpoint())?;
            }
            Some(JinjaTokenType::Else) => {
                state.expect(JinjaTokenType::TagStart)?;
                state.expect(JinjaTokenType::Else)?;
                state.expect(JinjaTokenType::TagEnd)?;
                while state.not_at_end() && !(state.at(JinjaTokenType::TagStart) && state.peek_kind_at(1) == Some(JinjaTokenType::Endif)) {
                    self.parse_node(state)?;
                }
            }
            _ => {}
        }

        if state.at(JinjaTokenType::TagStart) && state.peek_kind_at(1) == Some(JinjaTokenType::Endif) {
            state.expect(JinjaTokenType::TagStart)?;
            state.expect(JinjaTokenType::Endif)?;
            state.expect(JinjaTokenType::TagEnd)?;
        }

        state.finish_at(cp, JinjaElementType::IfStatement);
        Ok(())
    }

    fn parse_for_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) -> Result<(), OakError> {
        state.expect(JinjaTokenType::For)?;
        // Simplified parsing: for x in y
        self.parse_expression(state)?;
        state.expect(JinjaTokenType::In)?;
        self.parse_expression(state)?;
        state.expect(JinjaTokenType::TagEnd)?;

        while state.not_at_end() && !(state.at(JinjaTokenType::TagStart) && state.peek_kind_at(1) == Some(JinjaTokenType::Endfor)) {
            self.parse_node(state)?;
        }

        state.expect(JinjaTokenType::TagStart)?;
        state.expect(JinjaTokenType::Endfor)?;
        state.expect(JinjaTokenType::TagEnd)?;

        state.finish_at(cp, JinjaElementType::ForStatement);
        Ok(())
    }

    fn parse_block_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) -> Result<(), OakError> {
        state.expect(JinjaTokenType::Block)?;
        state.expect(JinjaTokenType::Identifier)?;
        state.expect(JinjaTokenType::TagEnd)?;

        while state.not_at_end() && !(state.at(JinjaTokenType::TagStart) && state.peek_kind_at(1) == Some(JinjaTokenType::Endblock)) {
            self.parse_node(state)?;
        }

        state.expect(JinjaTokenType::TagStart)?;
        state.expect(JinjaTokenType::Endblock)?;
        if state.at(JinjaTokenType::Identifier) {
            state.advance();
        }
        state.expect(JinjaTokenType::TagEnd)?;

        state.finish_at(cp, JinjaElementType::Block);
        Ok(())
    }

    fn parse_macro_definition<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) -> Result<(), OakError> {
        state.expect(JinjaTokenType::Macro)?;
        state.expect(JinjaTokenType::Identifier)?;
        if state.at(JinjaTokenType::LeftParen) {
            state.advance();
            while state.not_at_end() && !state.at(JinjaTokenType::RightParen) {
                state.advance();
            }
            state.expect(JinjaTokenType::RightParen)?;
        }
        state.expect(JinjaTokenType::TagEnd)?;

        while state.not_at_end() && !(state.at(JinjaTokenType::TagStart) && state.peek_kind_at(1) == Some(JinjaTokenType::Endmacro)) {
            self.parse_node(state)?;
        }

        state.expect(JinjaTokenType::TagStart)?;
        state.expect(JinjaTokenType::Endmacro)?;
        state.expect(JinjaTokenType::TagEnd)?;

        state.finish_at(cp, JinjaElementType::MacroDefinition);
        Ok(())
    }

    fn parse_set_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) -> Result<(), OakError> {
        state.expect(JinjaTokenType::Set)?;
        state.expect(JinjaTokenType::Identifier)?;
        state.expect(JinjaTokenType::Equal)?;
        self.parse_expression(state)?;
        state.expect(JinjaTokenType::TagEnd)?;
        state.finish_at(cp, JinjaElementType::SetStatement);
        Ok(())
    }

    fn parse_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        self.parse_binary_expression(state, 0)
    }

    fn parse_binary_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, min_precedence: i32) -> Result<(), OakError> {
        let cp = state.checkpoint();
        self.parse_primary_expression(state)?;

        while let Some(kind) = state.peek_kind() {
            let precedence = self.get_precedence(kind);
            if precedence < min_precedence {
                break;
            }

            state.advance();
            self.parse_binary_expression(state, precedence + 1)?;
            state.finish_at(cp, if kind == JinjaTokenType::Pipe { JinjaElementType::FilterExpression } else { JinjaElementType::BinaryExpression });
        }

        Ok(())
    }

    fn get_precedence(&self, kind: JinjaTokenType) -> i32 {
        match kind {
            JinjaTokenType::Pipe => 1,
            JinjaTokenType::Plus | JinjaTokenType::Minus => 2,
            JinjaTokenType::Star | JinjaTokenType::Slash => 3,
            _ => -1,
        }
    }

    fn parse_primary_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(JinjaTokenType::Identifier) => {
                state.advance();
                if state.at(JinjaTokenType::LeftParen) {
                    self.parse_call_expression(state, cp)?;
                }
                else {
                    state.finish_at(cp, JinjaElementType::Identifier);
                }
            }
            Some(JinjaTokenType::String) | Some(JinjaTokenType::Number) => {
                state.advance();
                state.finish_at(cp, JinjaElementType::Literal);
            }
            Some(JinjaTokenType::LeftParen) => {
                state.advance();
                self.parse_expression(state)?;
                state.expect(JinjaTokenType::RightParen)?;
            }
            _ => {
                while state.not_at_end() && !state.at(JinjaTokenType::TagEnd) && !state.at(JinjaTokenType::VariableEnd) && !state.at(JinjaTokenType::RightParen) && !state.at(JinjaTokenType::Comma) {
                    state.advance();
                }
            }
        }
        Ok(())
    }

    fn parse_call_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) -> Result<(), OakError> {
        state.expect(JinjaTokenType::LeftParen)?;
        while state.not_at_end() && !state.at(JinjaTokenType::RightParen) {
            self.parse_expression(state)?;
            if state.at(JinjaTokenType::Comma) {
                state.advance();
            }
        }
        state.expect(JinjaTokenType::RightParen)?;
        state.finish_at(cp, JinjaElementType::CallExpression);
        Ok(())
    }
}

impl<'config> Parser<JinjaLanguage> for JinjaParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<JinjaLanguage>) -> ParseOutput<'a, JinjaLanguage> {
        let lexer = crate::lexer::JinjaLexer::new(self.language);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let cp = state.checkpoint();
            while state.not_at_end() {
                self.parse_node(state)?;
            }
            Ok(state.finish_at(cp, JinjaElementType::Root))
        })
    }
}
