use crate::{
    lexer::{
        DejavuKeywords,
        token_type::DejavuTokenType::{self, *},
    },
    parser::element_type::DejavuElementType::*,
};
use oak_core::{GreenNode, OakError};

use super::State;

impl super::DejavuParser {
    pub(crate) fn parse_namespace<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        while state.at(At) || state.at(Bolt) {
            self.parse_attribute(state)?;
            self.skip_trivia(state);
        }

        state.expect(Keyword(DejavuKeywords::Namespace))?;
        self.skip_trivia(state);

        if state.at(Bang) {
            state.bump();
            self.skip_trivia(state);
        }

        self.parse_name_path(state)?;
        self.skip_trivia(state);

        if state.at(LeftBrace) {
            self.parse_block_expr_node(state)?;
        }
        else if state.at(Semicolon) {
            state.bump();
        }
        Ok(state.finish_at(cp, Namespace))
    }

    pub(crate) fn parse_attribute<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        if state.at(At) || state.at(Bolt) {
            state.bump();
        }
        else {
            return Err(OakError::custom_error(format!("Expected @ or @, but found {:?}", state.current().map(|t| t.kind))));
        }
        self.skip_trivia(state);
        if state.at(LeftBracket) {
            state.bump();
            self.skip_trivia(state);
            while state.not_at_end() && !state.at(RightBracket) {
                self.parse_name_path(state)?;
                self.skip_trivia(state);
                if state.at(LeftParen) {
                    state.bump();
                    self.skip_trivia(state);
                    while state.not_at_end() && !state.at(RightParen) {
                        let _ = self.parse_expression_internal(state, 0)?;
                        self.skip_trivia(state);
                        if state.at(Comma) {
                            state.bump();
                            self.skip_trivia(state);
                        }
                    }
                    state.expect(RightParen)?;
                    self.skip_trivia(state);
                }
                if state.at(Comma) {
                    state.bump();
                    self.skip_trivia(state);
                }
            }
            state.expect(RightBracket)?
        }
        else {
            self.parse_name_path(state)?;
            self.skip_trivia(state);
            if state.at(LeftParen) {
                state.bump();
                self.skip_trivia(state);
                while state.not_at_end() && !state.at(RightParen) {
                    let _ = self.parse_expression_internal(state, 0)?;
                    self.skip_trivia(state);
                    if state.at(Comma) {
                        state.bump();
                        self.skip_trivia(state);
                    }
                }
                state.expect(RightParen)?;
            }
        }
        Ok(state.finish_at(cp, Attribute))
    }

    pub(crate) fn parse_micro<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        while state.at(At) || state.at(Bolt) {
            self.parse_attribute(state)?;
            self.skip_trivia(state);
        }

        state.expect(Keyword(DejavuKeywords::Micro))?;
        self.skip_trivia(state);

        state.expect(Identifier)?;
        self.skip_trivia(state);

        if state.at(LessThan) {
            self.parse_generic_parameter_list(state)?;
            self.skip_trivia(state);
        }

        if state.at(LeftParen) {
            self.parse_parameter_list(state)?;
        }

        self.skip_trivia(state);

        if state.at(Arrow) {
            state.bump();
            self.skip_trivia(state);
            let cp_type = state.checkpoint();
            self.parse_name_path(state)?;
            state.finish_at(cp_type, Type);
            self.skip_trivia(state);
        }

        if state.at(LeftBrace) {
            self.parse_block_expr_node(state)?;
        }

        Ok(state.finish_at(cp, Micro))
    }

    pub(crate) fn parse_micro_lambda<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        if state.at(Keyword(DejavuKeywords::Micro)) {
            state.bump();
        }
        else {
            state.expect(Keyword(DejavuKeywords::Lambda))?;
        }
        self.skip_trivia(state);

        if state.at(Identifier) {
            state.bump();
            self.skip_trivia(state);
        }

        if state.at(LeftParen) {
            self.parse_parameter_list(state)?;
        }
        self.skip_trivia(state);

        if state.at(Arrow) {
            state.bump();
            self.skip_trivia(state);
            let cp_type = state.checkpoint();
            self.parse_name_path(state)?;
            state.finish_at(cp_type, Type);
            self.skip_trivia(state);
        }

        if state.at(LeftBrace) {
            self.parse_block_expr_node(state)?;
        }

        Ok(state.finish_at(cp, Micro))
    }

    pub(crate) fn parse_mezzo<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        while state.at(At) || state.at(Bolt) {
            self.parse_attribute(state)?;
            self.skip_trivia(state);
        }

        state.expect(Keyword(DejavuKeywords::Mezzo))?;
        self.skip_trivia(state);

        state.expect(Identifier)?;
        self.skip_trivia(state);

        if state.at(LeftParen) {
            self.parse_parameter_list(state)?;
        }

        self.skip_trivia(state);

        if state.at(LeftBrace) {
            self.parse_block_expr_node(state)?;
        }

        Ok(state.finish_at(cp, Mezzo))
    }

    pub(crate) fn parse_widget<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        while state.at(At) || state.at(Bolt) {
            self.parse_attribute(state)?;
            self.skip_trivia(state);
        }

        state.expect(Keyword(DejavuKeywords::Widget))?;
        self.skip_trivia(state);

        state.expect(Identifier)?;
        self.skip_trivia(state);

        if state.at(LeftBrace) {
            self.parse_block_expr_node(state)?;
        }

        Ok(state.finish_at(cp, Widget))
    }

    pub(crate) fn parse_singleton<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        while state.at(At) || state.at(Bolt) {
            self.parse_attribute(state)?;
            self.skip_trivia(state);
        }

        state.expect(Keyword(DejavuKeywords::Singleton))?;
        self.skip_trivia(state);

        state.expect(Identifier)?;
        self.skip_trivia(state);

        if state.at(LeftBrace) {
            self.parse_block_expr_node(state)?;
        }

        Ok(state.finish_at(cp, Class))
    }

    pub(crate) fn parse_trait<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        while state.at(At) || state.at(Bolt) {
            self.parse_attribute(state)?;
            self.skip_trivia(state);
        }

        state.expect(Keyword(DejavuKeywords::Trait))?;
        self.skip_trivia(state);

        state.expect(Identifier)?;
        self.skip_trivia(state);

        if state.at(LessThan) {
            self.parse_generic_parameter_list(state)?;
            self.skip_trivia(state);
        }

        if state.at(LeftParen) {
            state.bump();
            self.skip_trivia(state);
            while state.not_at_end() && !state.at(RightParen) && !state.at(DejavuTokenType::Eof) {
                let cp_type = state.checkpoint();
                self.parse_name_path(state)?;
                state.finish_at(cp_type, Type);
                self.skip_trivia(state);
                if state.at(Comma) {
                    state.bump();
                    self.skip_trivia(state);
                }
            }
            state.expect(RightParen)?;
            self.skip_trivia(state);
        }

        if state.at(LeftBrace) {
            self.parse_block_expr_node(state)?;
        }

        Ok(state.finish_at(cp, Trait))
    }

    pub(crate) fn parse_effect<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        while state.at(At) || state.at(Bolt) {
            self.parse_attribute(state)?;
            self.skip_trivia(state);
        }

        state.expect(Keyword(DejavuKeywords::Effect))?;
        self.skip_trivia(state);

        state.expect(Identifier)?;
        self.skip_trivia(state);

        if state.at(LessThan) {
            self.parse_generic_parameter_list(state)?;
            self.skip_trivia(state);
        }

        if state.at(LeftBrace) {
            self.parse_block_expr_node(state)?;
        }

        Ok(state.finish_at(cp, EffectDefinition))
    }

    pub(crate) fn parse_class<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        while state.at(At) || state.at(Bolt) {
            self.parse_attribute(state)?;
            self.skip_trivia(state);
        }

        state.expect(Keyword(DejavuKeywords::Class))?;
        self.skip_trivia(state);

        state.expect(Identifier)?;
        self.skip_trivia(state);

        if state.at(LessThan) {
            self.parse_generic_parameter_list(state)?;
            self.skip_trivia(state);
        }

        if state.at(LeftParen) {
            state.bump();
            self.skip_trivia(state);
            while state.not_at_end() && !state.at(RightParen) && !state.at(DejavuTokenType::Eof) {
                let cp_type = state.checkpoint();
                self.parse_name_path(state)?;
                state.finish_at(cp_type, Type);
                self.skip_trivia(state);
                if state.at(Comma) {
                    state.bump();
                    self.skip_trivia(state);
                }
            }
            state.expect(RightParen)?;
            self.skip_trivia(state);
        }

        if state.at(LeftBrace) {
            self.parse_block_expr_node(state)?;
        }

        Ok(state.finish_at(cp, Class))
    }

    pub(crate) fn parse_flags<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        while state.at(At) || state.at(Bolt) {
            self.parse_attribute(state)?;
            self.skip_trivia(state)
        }

        state.expect(Keyword(DejavuKeywords::Flags))?;
        self.skip_trivia(state);

        state.expect(Identifier)?;
        self.skip_trivia(state);

        if state.at(LeftBrace) {
            self.parse_variant_block(state)?;
        }
        Ok(state.finish_at(cp, Flags))
    }

    pub(crate) fn parse_enums<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        while state.at(At) || state.at(Bolt) {
            self.parse_attribute(state)?;
            self.skip_trivia(state);
        }

        state.expect(Keyword(DejavuKeywords::Enums))?;
        self.skip_trivia(state);

        state.expect(Identifier)?;
        self.skip_trivia(state);

        if state.at(LeftBrace) {
            self.parse_variant_block(state)?;
        }

        Ok(state.finish_at(cp, Enums))
    }

    pub(crate) fn parse_variant_block<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(LeftBrace)?;
        self.skip_trivia(state);

        while let Some(t) = state.current() {
            if t.kind == RightBrace || t.kind == DejavuTokenType::Eof {
                break;
            }

            if t.kind == At || t.kind == Bolt || t.kind == Identifier {
                self.parse_variant(state)?;
            }
            else {
                state.bump();
            }

            self.skip_trivia(state);

            if state.at(Comma) {
                state.bump();
                self.skip_trivia(state);
            }
        }

        state.expect(RightBrace)?;
        Ok(state.finish_at(cp, BlockExpression))
    }

    pub(crate) fn parse_variant<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        while state.at(At) || state.at(Bolt) {
            self.parse_attribute(state)?;
            self.skip_trivia(state);
        }

        state.expect(Identifier)?;
        self.skip_trivia(state);

        if state.at(Eq) {
            state.bump();
            self.skip_trivia(state);
            let _ = self.parse_expression_internal(state, 0);
            self.skip_trivia(state);
        }

        if state.at(LeftBrace) {
            self.parse_block_expr_node(state)?;
        }

        Ok(state.finish_at(cp, Variant))
    }

    pub(crate) fn parse_generic_parameter_list<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        let use_angle_brackets = state.at(LeftAngle);
        if use_angle_brackets {
            state.expect(LeftAngle)?;
        }
        else {
            state.expect(LessThan)?;
        }
        self.skip_trivia(state);

        let end_token = if use_angle_brackets { RightAngle } else { GreaterThan };
        while state.not_at_end() && !state.at(end_token) {
            state.expect(Identifier)?;
            self.skip_trivia(state);
            if state.at(Comma) {
                state.bump();
                self.skip_trivia(state);
            }
        }

        state.expect(end_token)?;
        Ok(state.finish_at(cp, GenericParameterList))
    }

    pub(crate) fn parse_generic_argument_list<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        let use_angle_brackets = state.at(LeftAngle);
        if use_angle_brackets {
            state.expect(LeftAngle)?;
        }
        else {
            state.expect(LessThan)?;
        }
        self.skip_trivia(state);

        let end_token = if use_angle_brackets { RightAngle } else { GreaterThan };
        while state.not_at_end() && !state.at(end_token) {
            let cp_type = state.checkpoint();
            self.parse_name_path(state)?;
            state.finish_at(cp_type, Type);
            self.skip_trivia(state);
            if state.at(Comma) {
                state.bump();
                self.skip_trivia(state);
            }
        }

        state.expect(end_token)?;
        Ok(state.finish_at(cp, GenericArgumentList))
    }

    pub(crate) fn parse_parameter_list<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        state.expect(LeftParen)?;
        self.skip_trivia(state);

        while let Some(t) = state.current() {
            if t.kind == RightParen || t.kind == DejavuTokenType::Eof {
                break;
            }

            if t.kind == Identifier || t.kind == At || t.kind == Bolt {
                self.parse_parameter(state)?;
                self.skip_trivia(state);

                if state.at(Comma) {
                    state.bump();
                    self.skip_trivia(state);
                }
            }
            else {
                state.bump();
                self.skip_trivia(state);
            }
        }

        state.expect(RightParen)?;

        Ok(state.finish_at(cp, ParameterList))
    }

    pub(crate) fn parse_parameter<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        while state.at(At) || state.at(Bolt) {
            self.parse_attribute(state)?;
            self.skip_trivia(state);
        }

        state.expect(Identifier)?;
        self.skip_trivia(state);
        if state.at(Colon) {
            state.bump();
            self.skip_trivia(state);
            let cp_type = state.checkpoint();
            self.parse_name_path(state)?;
            state.finish_at(cp_type, Type);
            self.skip_trivia(state);
        }
        Ok(state.finish_at(cp, Parameter))
    }

    pub(crate) fn parse_anonymous_class<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(Keyword(DejavuKeywords::Class))?;
        self.skip_trivia(state);

        if state.at(LeftParen) {
            state.bump();
            self.skip_trivia(state);
            while state.not_at_end() && !state.at(RightParen) && !state.at(DejavuTokenType::Eof) {
                state.expect(Identifier)?;
                self.skip_trivia(state);
                if state.at(Comma) {
                    state.bump();
                    self.skip_trivia(state);
                }
            }
            state.expect(RightParen)?;
            self.skip_trivia(state);
        }

        self.parse_block_expr_node(state)?;

        Ok(state.finish_at(cp, AnonymousClass))
    }
}
