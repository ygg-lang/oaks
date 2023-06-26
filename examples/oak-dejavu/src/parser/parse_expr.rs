use crate::{lexer::token_type::DejavuTokenType::*, parser::element_type::DejavuElementType};
use oak_core::{GreenNode, OakError, source::Source};

use super::State;

impl super::DejavuParser {
    pub(crate) fn parse_expression_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, _precedence: u8) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        if state.at(Identifier) {
            state.bump();
            Ok(state.finish_at(cp, DejavuElementType::IdentifierExpression))
        }
        else if state.at(StringLiteral) || state.at(CharLiteral) {
            self.parse_string_literal(state)
        }
        else if state.at(IntegerLiteral) {
            state.bump();
            Ok(state.finish_at(cp, DejavuElementType::LiteralExpression))
        }
        else if state.at(LeftParen) {
            state.bump();
            self.parse_expression_internal(state, 0)?;
            state.expect(RightParen)?;
            Ok(state.finish_at(cp, DejavuElementType::ParenthesizedExpression))
        }
        else {
            state.bump();
            Ok(state.finish_at(cp, DejavuElementType::Error))
        }
    }

    pub(crate) fn parse_template_interpolation<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(InterpolationStart)?;
        self.parse_expression_internal(state, 0)?;
        if state.at(InterpolationEnd) {
            state.bump();
        }
        Ok(state.finish_at(cp, DejavuElementType::Interpolation))
    }

    pub(crate) fn parse_template_control<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(TemplateControlStart)?;
        while state.not_at_end() && !state.at(TemplateControlEnd) {
            state.bump();
        }
        if state.at(TemplateControlEnd) {
            state.bump();
        }
        Ok(state.finish_at(cp, DejavuElementType::TemplateControl))
    }

    pub(crate) fn parse_template_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(TemplateCommentStart)?;
        while state.not_at_end() && !state.at(TemplateCommentEnd) {
            state.bump();
        }
        if state.at(TemplateCommentEnd) {
            state.bump();
        }
        Ok(state.finish_at(cp, DejavuElementType::TemplateComment))
    }
}
