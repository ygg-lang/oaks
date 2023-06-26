use crate::{
    lexer::{
        DejavuKeywords,
        token_type::DejavuTokenType::{self, *},
    },
    parser::element_type::DejavuElementType::*,
};
use oak_core::{GreenNode, OakError, source::Source};

use super::State;

impl super::DejavuParser {
    pub(crate) fn parse_if<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(Keyword(DejavuKeywords::If))?;
        self.skip_trivia(state);

        if state.at(Keyword(DejavuKeywords::Let)) {
            state.bump();
            self.skip_trivia(state);
            self.parse_pattern(state)?;
            self.skip_trivia(state);
            state.expect(Eq)?;
            self.skip_trivia(state);
            let _ = self.parse_expression_internal(state, 8);
        }
        else {
            let _ = self.parse_expression_internal(state, 8);
            self.skip_trivia(state);
            if state.at(Keyword(DejavuKeywords::Is)) {
                let cp_is = state.checkpoint();
                state.bump();
                self.skip_trivia(state);
                self.parse_name_path(state)?;
                state.finish_at(cp_is, BinaryExpression);
            }
        }

        self.skip_trivia(state);
        self.parse_block_expr_node(state)?;
        self.skip_trivia(state);
        if state.at(Keyword(DejavuKeywords::Else)) {
            state.bump();
            self.skip_trivia(state);
            if state.at(Keyword(DejavuKeywords::If)) {
                self.parse_if(state)?;
            }
            else {
                self.parse_block_expr_node(state)?;
            }
        }
        Ok(state.finish_at(cp, IfExpression))
    }

    pub(crate) fn parse_match<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(Keyword(DejavuKeywords::Match))?;
        self.skip_trivia(state);
        let _ = self.parse_expression_internal(state, 8);
        self.skip_trivia(state);

        state.expect(LeftBrace)?;
        self.skip_trivia(state);
        while state.not_at_end() && !state.at(RightBrace) && !state.at(DejavuTokenType::Eof) {
            self.parse_match_arm(state)?;
            self.skip_trivia(state);
        }
        state.expect(RightBrace)?;
        Ok(state.finish_at(cp, MatchExpression))
    }

    pub(crate) fn parse_match_arm<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        if state.at(Keyword(DejavuKeywords::Case)) {
            state.bump();
            self.skip_trivia(state);
            self.parse_pattern(state)?;
        }
        else if state.at(Keyword(DejavuKeywords::Type)) {
            state.bump();
            self.skip_trivia(state);
            self.parse_name_path(state)?;
        }
        else if state.at(Keyword(DejavuKeywords::When)) {
            state.bump();
            self.skip_trivia(state);
            let _ = self.parse_expression_internal(state, 10);
        }
        else if state.at(Keyword(DejavuKeywords::Else)) {
            state.bump();
        }
        else {
            self.parse_pattern(state)?;
        }
        self.skip_trivia(state);

        if state.at(Keyword(DejavuKeywords::When)) {
            state.bump();
            self.skip_trivia(state);
            let _ = self.parse_expression_internal(state, 10);
            self.skip_trivia(state);
        }

        if state.at(Arrow) || state.at(Colon) {
            state.bump();
            self.skip_trivia(state);
            let _ = self.parse_expression_internal(state, 0);
        }
        else if state.at(LeftBrace) {
            self.parse_block_expr_node(state)?;
        }
        else {
            return Err(OakError::custom_error(format!("Expected ->, : or {{, but found {:?}", state.current().map(|t| t.kind))));
        }
        self.skip_trivia(state);
        if state.at(Comma) || state.at(Semicolon) {
            state.bump();
        }
        Ok(state.finish_at(cp, MatchArm))
    }

    pub(crate) fn parse_pattern<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        if state.at(Keyword(DejavuKeywords::Is)) {
            state.bump();
            self.skip_trivia(state);
            self.parse_name_path(state)?;
        }
        else if state.at(Keyword(DejavuKeywords::Else)) {
            state.bump();
        }
        else if state.at(Identifier) {
            self.parse_name_path(state)?;
            self.skip_trivia(state);
            if state.at(LeftBrace) {
                state.bump();
                self.skip_trivia(state);
                while state.not_at_end() && !state.at(RightBrace) && !state.at(DejavuTokenType::Eof) {
                    state.expect(Identifier)?;
                    self.skip_trivia(state);
                    if state.at(Eq) {
                        state.bump();
                        self.skip_trivia(state);
                        self.parse_pattern(state)?;
                        self.skip_trivia(state);
                    }
                    if state.at(Comma) {
                        state.bump();
                        self.skip_trivia(state);
                    }
                }
                state.expect(RightBrace)?;
            }
        }
        else if state.at(IntegerLiteral) || state.at(StringLiteral) || state.at(Keyword(DejavuKeywords::True)) || state.at(Keyword(DejavuKeywords::False)) {
            state.bump();
        }
        else if state.at(Underscore) {
            state.bump();
        }
        else {
            return Err(OakError::custom_error(format!("Expected pattern, found {:?}", state.current().map(|t| t.kind))));
        }
        Ok(state.finish_at(cp, Pattern))
    }

    pub(crate) fn parse_loop<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        let is_while = state.at(Keyword(DejavuKeywords::While));
        let is_for = state.at(Keyword(DejavuKeywords::For));
        let is_loop = state.at(Keyword(DejavuKeywords::Loop));
        state.bump();
        self.skip_trivia(state);

        if is_while && state.at(Keyword(DejavuKeywords::Let)) {
            state.bump();
            self.skip_trivia(state);
            self.parse_pattern(state)?;
            self.skip_trivia(state);
            state.expect(Eq)?;
            self.skip_trivia(state);
            let _ = self.parse_expression_internal(state, 8);
        }
        else if is_for {
            self.parse_pattern(state)?;
            self.skip_trivia(state);
            state.expect(Keyword(DejavuKeywords::In))?;
            self.skip_trivia(state);
            let _ = self.parse_expression_internal(state, 8);
        }
        else if is_loop {
            let pattern_cp = state.checkpoint();
            if self.parse_pattern(state).is_ok() {
                self.skip_trivia(state);
                if state.at(Keyword(DejavuKeywords::In)) {
                    state.bump();
                    self.skip_trivia(state);
                    let _ = self.parse_expression_internal(state, 8);
                }
                else {
                    state.restore(pattern_cp);
                }
            }
            else {
                state.restore(pattern_cp);
            }
        }
        else {
            let _ = self.parse_expression_internal(state, 8);
        }

        self.skip_trivia(state);
        self.parse_block_expr_node(state)?;
        Ok(state.finish_at(cp, LoopExpression))
    }

    pub(crate) fn parse_return<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(Keyword(DejavuKeywords::Return))?;
        self.skip_trivia(state);
        if state.not_at_end() && !state.at(Semicolon) && !state.at(RightBrace) {
            let _ = self.parse_expression_internal(state, 0);
        }
        Ok(state.finish_at(cp, ReturnExpression))
    }

    pub(crate) fn parse_break<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(Keyword(DejavuKeywords::Break))?;
        self.skip_trivia(state);
        if state.at(At) {
            state.bump();
            state.expect(Identifier)?;
            self.skip_trivia(state);
        }
        if state.not_at_end() && !state.at(Semicolon) && !state.at(RightBrace) {
            let _ = self.parse_expression_internal(state, 0);
        }

        Ok(state.finish_at(cp, BreakExpression))
    }

    pub(crate) fn parse_continue<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(Keyword(DejavuKeywords::Continue))?;
        self.skip_trivia(state);
        if state.at(At) {
            state.bump();
            state.expect(Identifier)?;
        }

        Ok(state.finish_at(cp, ContinueExpression))
    }

    pub(crate) fn parse_yield<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(Keyword(DejavuKeywords::Yield))?;
        self.skip_trivia(state);
        if state.at(Star) {
            state.bump();
            self.skip_trivia(state);
        }

        if state.not_at_end() && !state.at(Semicolon) && !state.at(RightBrace) {
            let _ = self.parse_expression_internal(state, 0);
        }
        Ok(state.finish_at(cp, YieldExpression))
    }

    pub(crate) fn parse_raise<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(Keyword(DejavuKeywords::Raise))?;
        self.skip_trivia(state);
        let _ = self.parse_expression_internal(state, 0)?;
        Ok(state.finish_at(cp, RaiseExpression))
    }

    pub(crate) fn parse_resume<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(Keyword(DejavuKeywords::Resume))?;
        self.skip_trivia(state);
        if state.not_at_end() && !state.at(Semicolon) && !state.at(RightBrace) {
            let _ = self.parse_expression_internal(state, 0);
        }
        Ok(state.finish_at(cp, ResumeExpression))
    }

    pub(crate) fn parse_catch<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(Keyword(DejavuKeywords::Try))?;
        self.skip_trivia(state);

        if state.at(Identifier) {
            let cp_type = state.checkpoint();
            self.parse_name_path(state)?;
            if state.at(LessThan) {
                self.parse_generic_argument_list(state)?;
            }
            state.finish_at(cp_type, Type);
            self.skip_trivia(state);
        }

        self.parse_block_expr_node(state)?;
        self.skip_trivia(state);

        while state.at(Dot) {
            let cp_dot = state.checkpoint();
            state.bump();
            self.skip_trivia(state);

            if state.at(Keyword(DejavuKeywords::Catch)) {
                state.bump();
                self.skip_trivia(state);
                state.expect(LeftBrace)?;
                self.skip_trivia(state);
                while state.not_at_end() && !state.at(RightBrace) && !state.at(DejavuTokenType::Eof) {
                    self.parse_match_arm(state)?;
                    self.skip_trivia(state);
                }
                state.expect(RightBrace)?;
                self.skip_trivia(state);
            }
            else {
                state.restore(cp_dot);
                break;
            }
        }
        Ok(state.finish_at(cp, CatchExpression))
    }

    pub(crate) fn parse_block_expr_node<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        if !state.at(LeftBrace) {
            return Err(OakError::custom_error(format!("expected token 'LeftBrace' at {:?}", state.current().map(|t| t.span.start))));
        }
        state.expect(LeftBrace)?;
        self.skip_trivia(state);

        while let Some(t) = state.current() {
            if t.kind == RightBrace || t.kind == DejavuTokenType::Eof {
                break;
            }
            let start_index = state.tokens.index();
            match self.parse_statement(state) {
                Ok(_) => {}
                Err(_) => {
                    if state.tokens.index() == start_index {
                        state.bump();
                    }
                }
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

    pub(crate) fn peek_after_attributes<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Option<DejavuTokenType> {
        let cp = state.checkpoint();
        while state.at(At) || state.at(Bolt) {
            if self.parse_attribute(state).is_err() {
                state.restore(cp);
                return None;
            }
            self.skip_trivia(state);
        }
        let kind = state.current().map(|t| t.kind);
        state.restore(cp);
        kind
    }

    pub(crate) fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        self.skip_trivia(state);
        if let Some(t) = state.current() {
            match t.kind {
                At | Bolt => {
                    let next = self.peek_after_attributes(state);
                    match next {
                        Some(Keyword(DejavuKeywords::Micro)) => self.parse_micro(state),
                        Some(Keyword(DejavuKeywords::Class)) => self.parse_class(state),
                        Some(Keyword(DejavuKeywords::Enums)) => self.parse_enums(state),
                        Some(Keyword(DejavuKeywords::Flags)) => self.parse_flags(state),
                        Some(Keyword(DejavuKeywords::Trait)) => self.parse_trait(state),
                        Some(Keyword(DejavuKeywords::Effect)) => self.parse_effect(state),
                        Some(Keyword(DejavuKeywords::Mezzo)) => self.parse_mezzo(state),
                        Some(Keyword(DejavuKeywords::Widget)) => self.parse_widget(state),
                        Some(Keyword(DejavuKeywords::Namespace)) => self.parse_namespace(state),
                        Some(Keyword(DejavuKeywords::Singleton)) => self.parse_singleton(state),
                        Some(Keyword(DejavuKeywords::Let)) => self.parse_let_statement(state),
                        _ => self.parse_expression_statement(state),
                    }
                }
                Keyword(DejavuKeywords::Let) => self.parse_let_statement(state),
                Keyword(DejavuKeywords::Micro) => self.parse_micro(state),
                Keyword(DejavuKeywords::Class) => self.parse_class(state),
                Keyword(DejavuKeywords::Enums) => self.parse_enums(state),
                Keyword(DejavuKeywords::Flags) => self.parse_flags(state),
                Keyword(DejavuKeywords::Trait) => self.parse_trait(state),
                Keyword(DejavuKeywords::Effect) => self.parse_effect(state),
                Keyword(DejavuKeywords::Mezzo) => self.parse_mezzo(state),
                Keyword(DejavuKeywords::Widget) => self.parse_widget(state),
                Keyword(DejavuKeywords::Namespace) => self.parse_namespace(state),
                Keyword(DejavuKeywords::Singleton) => self.parse_singleton(state),
                Keyword(DejavuKeywords::Using) => self.parse_using_statement(state),
                _ => self.parse_expression_statement(state),
            }
        }
        else {
            Err(OakError::custom_error("Unexpected end of file"))
        }
    }

    pub(crate) fn parse_let_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        while state.at(At) || state.at(Bolt) {
            self.parse_attribute(state)?;
            self.skip_trivia(state);
        }

        state.expect(Keyword(DejavuKeywords::Let))?;
        self.skip_trivia(state);

        self.parse_pattern(state)?;
        self.skip_trivia(state);

        if state.at(Colon) {
            state.bump();
            self.skip_trivia(state);
            self.parse_name_path(state)?;
            self.skip_trivia(state);
        }

        if state.at(Eq) {
            state.bump();
            self.skip_trivia(state);
            let _ = self.parse_expression_internal(state, 0);
            self.skip_trivia(state);
        }

        if state.at(Semicolon) {
            state.bump();
        }

        Ok(state.finish_at(cp, LetStatement))
    }

    pub(crate) fn parse_expression_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        let _ = self.parse_expression_internal(state, 0);
        self.skip_trivia(state);
        if state.at(Semicolon) {
            state.bump();
        }
        Ok(state.finish_at(cp, ExprStatement))
    }

    pub(crate) fn parse_using_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        state.expect(Keyword(DejavuKeywords::Using))?;
        self.skip_trivia(state);

        self.parse_name_path(state)?;
        self.skip_trivia(state);

        state.expect(Semicolon)?;

        Ok(state.finish_at(cp, UsingStatement))
    }

    pub(crate) fn parse_primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        let t = state.current().ok_or_else(|| OakError::custom_error("Unexpected end of file"))?;

        match t.kind {
            Identifier => {
                let cp_name = state.checkpoint();
                state.bump();
                self.skip_trivia(state);
                if state.at(ColonColon) {
                    let cp_path = cp_name;
                    while state.at(ColonColon) {
                        state.bump();
                        self.skip_trivia(state);
                        state.expect(Identifier)?;
                        self.skip_trivia(state);
                    }
                    state.finish_at(cp_path, NamePath);
                    Ok(state.finish_at(cp, PathExpression))
                }
                else {
                    Ok(state.finish_at(cp, IdentifierExpression))
                }
            }
            IntegerLiteral | FloatLiteral | BoolLiteral | Keyword(DejavuKeywords::True) | Keyword(DejavuKeywords::False) => {
                state.bump();
                Ok(state.finish_at(cp, LiteralExpression))
            }
            StringLiteral | CharLiteral => self.parse_string_literal(state),
            LeftParen => {
                state.bump();
                let _ = self.parse_expression_internal(state, 0);
                state.expect(RightParen)?;
                Ok(state.finish_at(cp, ParenthesizedExpression))
            }
            LeftBracket => {
                state.bump();
                self.skip_trivia(state);
                while state.not_at_end() && !state.at(RightBracket) {
                    let _ = self.parse_expression_internal(state, 0);
                    self.skip_trivia(state);
                    if state.at(Comma) {
                        state.bump();
                        self.skip_trivia(state);
                    }
                }
                state.expect(RightBracket)?;
                Ok(state.finish_at(cp, LiteralExpression))
            }
            LeftBrace => self.parse_block_expr_node(state),
            Dollar => {
                state.bump();
                if state.at(IntegerLiteral) {
                    state.bump();
                }
                else if state.at(Keyword(DejavuKeywords::True)) || state.at(Keyword(DejavuKeywords::False)) {
                    state.bump();
                }
                else if state.at(Identifier) {
                    state.bump();
                }
                Ok(state.finish_at(cp, IdentifierExpression))
            }
            Keyword(DejavuKeywords::Class) => self.parse_anonymous_class(state),
            Keyword(DejavuKeywords::Micro) => self.parse_micro_lambda(state),
            Keyword(DejavuKeywords::Lambda) => self.parse_micro_lambda(state),
            Keyword(DejavuKeywords::If) => self.parse_if(state),
            Keyword(DejavuKeywords::Match) => self.parse_match(state),
            Keyword(DejavuKeywords::While) | Keyword(DejavuKeywords::For) | Keyword(DejavuKeywords::Loop) => self.parse_loop(state),
            Keyword(DejavuKeywords::Return) => self.parse_return(state),
            Keyword(DejavuKeywords::Break) => self.parse_break(state),
            Keyword(DejavuKeywords::Continue) => self.parse_continue(state),
            Keyword(DejavuKeywords::Yield) => self.parse_yield(state),
            Keyword(DejavuKeywords::Raise) => self.parse_raise(state),
            Keyword(DejavuKeywords::Resume) => self.parse_resume(state),
            Keyword(DejavuKeywords::Try) => self.parse_catch(state),
            At | Bolt => {
                state.bump();
                state.expect(Identifier)?;
                self.skip_trivia(state);
                if state.at(LeftParen) {
                    let cp_call = cp;
                    state.finish_at(cp, IdentifierExpression);

                    state.expect(LeftParen)?;
                    self.skip_trivia(state);
                    while state.not_at_end() && !state.at(RightParen) {
                        let _ = self.parse_expression_internal(state, 0);
                        self.skip_trivia(state);
                        if state.at(Comma) {
                            state.bump();
                            self.skip_trivia(state);
                        }
                    }
                    state.expect(RightParen)?;
                    self.skip_trivia(state);
                    if state.at(LeftBrace) {
                        self.parse_block_expr_node(state).ok();
                        Ok(state.finish_at(cp_call, ObjectExpression))
                    }
                    else {
                        Ok(state.finish_at(cp_call, CallExpression))
                    }
                }
                else {
                    Ok(state.finish_at(cp, IdentifierExpression))
                }
            }
            _ => Err(OakError::custom_error(format!("Unexpected token: {:?}", t.kind))),
        }
    }

    pub(crate) fn parse_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        state.bump();

        while let Some(t) = state.current() {
            match t.kind {
                StringPart => {
                    let text_cp = state.checkpoint();
                    state.bump();
                    state.finish_at(text_cp, TemplateText);
                }
                InterpolationStart => {
                    self.parse_template_interpolation(state)?;
                }
                TemplateControlStart => {
                    self.parse_template_control(state)?;
                }
                TemplateCommentStart => {
                    self.parse_template_comment(state)?;
                }
                _ => break,
            }
        }

        Ok(state.finish_at(cp, LiteralExpression))
    }

    pub(crate) fn parse_name_path<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        if state.at(Identifier) || matches!(state.current().map(|t| t.kind), Some(Keyword(_))) {
            state.bump();
        }
        else {
            state.expect(Identifier)?;
        }
        self.skip_trivia(state);

        if state.at(LessThan) {
            self.parse_generic_argument_list(state)?;
            self.skip_trivia(state);
        }

        while state.at(ColonColon) || state.at(Dot) {
            state.bump();
            self.skip_trivia(state);
            if state.at(Identifier) || matches!(state.current().map(|t| t.kind), Some(Keyword(_))) {
                state.bump();
            }
            else {
                state.expect(Identifier)?;
            }
            self.skip_trivia(state);

            if state.at(LessThan) {
                self.parse_generic_argument_list(state)?;
                self.skip_trivia(state);
            }
        }

        let node = state.finish_at(cp, NamePath);
        Ok(node)
    }
}
