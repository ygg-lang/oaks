pub mod element_type;

use crate::{
    language::DjangoLanguage,
    lexer::{DjangoLexer, token_type::DjangoTokenType},
    parser::element_type::DjangoElementType,
};
use oak_core::{
    errors::OakError,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, DjangoLanguage, S>;

pub struct DjangoParser<'config> {
    pub(crate) config: &'config DjangoLanguage,
}

impl<'config> DjangoParser<'config> {
    pub fn new(config: &'config DjangoLanguage) -> Self {
        Self { config }
    }

    fn parse_node<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        match state.peek_kind() {
            Some(DjangoTokenType::VariableStart) => self.parse_variable(state),
            Some(DjangoTokenType::TagStart) => self.parse_tag_statement(state),
            Some(DjangoTokenType::CommentStart) => self.parse_comment(state),
            Some(DjangoTokenType::HtmlContent) => {
                let cp = state.checkpoint();
                state.bump();
                state.finish_at(cp, DjangoElementType::HtmlContent);
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
        state.expect(DjangoTokenType::VariableStart)?;
        self.parse_expression(state)?;
        state.expect(DjangoTokenType::VariableEnd).ok();
        state.finish_at(cp, DjangoElementType::Variable);
        Ok(())
    }

    fn parse_tag_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(DjangoTokenType::TagStart)?;

        let kind = state.peek_kind();
        match kind {
            Some(DjangoTokenType::If) => self.parse_if_statement(state, cp),
            Some(DjangoTokenType::For) => self.parse_for_statement(state, cp),
            Some(DjangoTokenType::Block) => self.parse_block_statement(state, cp),
            _ => {
                while state.not_at_end() && !state.at(DjangoTokenType::TagEnd) {
                    state.advance();
                }
                state.expect(DjangoTokenType::TagEnd).ok();
                state.finish_at(cp, DjangoElementType::Tag);
                Ok(())
            }
        }
    }

    fn parse_if_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) -> Result<(), OakError> {
        state.expect(DjangoTokenType::If)?;
        self.parse_expression(state)?;
        state.expect(DjangoTokenType::TagEnd).ok();

        while state.not_at_end() {
            if state.at(DjangoTokenType::TagStart) {
                let next_kind = state.peek_kind_at(1);
                if matches!(next_kind, Some(DjangoTokenType::Endif) | Some(DjangoTokenType::Elif) | Some(DjangoTokenType::Else)) {
                    break;
                }
            }
            self.parse_node(state)?;
        }

        match state.peek_kind_at(1) {
            Some(DjangoTokenType::Elif) => {
                state.expect(DjangoTokenType::TagStart)?;
                self.parse_if_statement(state, state.checkpoint())?;
            }
            Some(DjangoTokenType::Else) => {
                state.expect(DjangoTokenType::TagStart)?;
                state.expect(DjangoTokenType::Else)?;
                state.expect(DjangoTokenType::TagEnd).ok();
                while state.not_at_end() && !(state.at(DjangoTokenType::TagStart) && state.peek_kind_at(1) == Some(DjangoTokenType::Endif)) {
                    self.parse_node(state)?;
                }
            }
            _ => {}
        }

        if state.at(DjangoTokenType::TagStart) && state.peek_kind_at(1) == Some(DjangoTokenType::Endif) {
            state.expect(DjangoTokenType::TagStart)?;
            state.expect(DjangoTokenType::Endif)?;
            state.expect(DjangoTokenType::TagEnd).ok();
        }

        state.finish_at(cp, DjangoElementType::IfStatement);
        Ok(())
    }

    fn parse_for_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) -> Result<(), OakError> {
        state.expect(DjangoTokenType::For)?;
        self.parse_expression(state)?;
        state.expect(DjangoTokenType::In)?;
        self.parse_expression(state)?;
        state.expect(DjangoTokenType::TagEnd).ok();

        while state.not_at_end() && !(state.at(DjangoTokenType::TagStart) && state.peek_kind_at(1) == Some(DjangoTokenType::Endfor)) {
            self.parse_node(state)?;
        }

        state.expect(DjangoTokenType::TagStart)?;
        state.expect(DjangoTokenType::Endfor)?;
        state.expect(DjangoTokenType::TagEnd).ok();

        state.finish_at(cp, DjangoElementType::ForStatement);
        Ok(())
    }

    fn parse_block_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) -> Result<(), OakError> {
        state.expect(DjangoTokenType::Block)?;
        state.expect(DjangoTokenType::Identifier)?;
        state.expect(DjangoTokenType::TagEnd).ok();

        while state.not_at_end() && !(state.at(DjangoTokenType::TagStart) && state.peek_kind_at(1) == Some(DjangoTokenType::Endblock)) {
            self.parse_node(state)?;
        }

        state.expect(DjangoTokenType::TagStart)?;
        state.expect(DjangoTokenType::Endblock)?;
        state.expect(DjangoTokenType::TagEnd).ok();

        state.finish_at(cp, DjangoElementType::Block);
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
            state.finish_at(cp, if kind == DjangoTokenType::Pipe { DjangoElementType::FilterExpression } else { DjangoElementType::BinaryExpression });
        }

        Ok(())
    }

    fn get_precedence(&self, kind: DjangoTokenType) -> i32 {
        match kind {
            DjangoTokenType::Pipe => 1,
            // Django does not natively support operations like {{ a + b }}, but we can keep extensibility for the parser
            // Only support Pipe by default
            _ => -1,
        }
    }

    fn parse_primary_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(DjangoTokenType::Identifier) => {
                state.advance();
                state.finish_at(cp, DjangoElementType::Identifier);
            }
            Some(DjangoTokenType::String) | Some(DjangoTokenType::Number) => {
                state.advance();
                state.finish_at(cp, DjangoElementType::Literal);
            }
            _ => {
                while state.not_at_end() && !state.at(DjangoTokenType::TagEnd) && !state.at(DjangoTokenType::VariableEnd) {
                    state.advance();
                }
            }
        }
        Ok(())
    }

    fn parse_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(DjangoTokenType::CommentStart)?;
        while state.not_at_end() && !state.at(DjangoTokenType::CommentEnd) {
            state.advance();
        }
        state.expect(DjangoTokenType::CommentEnd).ok();
        state.finish_at(cp, DjangoElementType::Comment);
        Ok(())
    }
}

impl<'config> Parser<DjangoLanguage> for DjangoParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<DjangoLanguage>) -> ParseOutput<'a, DjangoLanguage> {
        let lexer = DjangoLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            while state.not_at_end() {
                self.parse_node(state)?;
            }
            Ok(state.finish_at(checkpoint, DjangoElementType::Root))
        })
    }
}
