use crate::parser::{ActionScriptElementType, ActionScriptParser};
use oak_core::{
    TokenType,
    errors::OakError,
    parser::{ParserState, PrattParser},
    tree::GreenNode,
};

impl<'config> ActionScriptParser<'config> {
    pub(crate) fn parse_source_file<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, crate::language::ActionScriptLanguage, S>) -> Result<&'a GreenNode<'a, crate::language::ActionScriptLanguage>, OakError> {
        let cp = state.checkpoint();
        while state.not_at_end() {
            if state.current().map(|t| t.kind.is_ignored()).unwrap_or(false) {
                state.advance();
                continue;
            }
            self.parse_statement(state)?;
        }
        let root = state.finish_at(cp, ActionScriptElementType::SourceFile.into());
        Ok(root)
    }

    fn parse_statement<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, crate::language::ActionScriptLanguage, S>) -> Result<(), OakError> {
        use crate::lexer::ActionScriptTokenType;
        match state.peek_kind() {
            Some(ActionScriptTokenType::Function) => self.parse_function(state)?,
            Some(ActionScriptTokenType::Import) => self.parse_import_statement(state)?,
            Some(ActionScriptTokenType::Package) => self.parse_package_declaration(state)?,
            Some(ActionScriptTokenType::Class) => self.parse_class_declaration(state)?,
            Some(ActionScriptTokenType::Interface) => self.parse_interface_declaration(state)?,
            Some(ActionScriptTokenType::Var) | Some(ActionScriptTokenType::Const) => self.parse_variable_declaration(state)?,
            Some(ActionScriptTokenType::If) => self.parse_if_statement(state)?,
            Some(ActionScriptTokenType::While) => self.parse_while_statement(state)?,
            Some(ActionScriptTokenType::For) => self.parse_for_statement(state)?,
            Some(ActionScriptTokenType::Return) => self.parse_return_statement(state)?,
            Some(ActionScriptTokenType::LeftBrace) => self.parse_block(state)?,
            _ => {
                PrattParser::parse(state, 0, self);
                state.eat(ActionScriptTokenType::Semicolon);
            }
        }
        Ok(())
    }

    fn parse_function<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, crate::language::ActionScriptLanguage, S>) -> Result<(), OakError> {
        use crate::lexer::ActionScriptTokenType;
        let cp = state.checkpoint();
        state.expect(ActionScriptTokenType::Function).ok();
        state.expect(ActionScriptTokenType::Identifier).ok();
        self.parse_param_list(state)?;
        if state.eat(ActionScriptTokenType::Colon) {
            state.expect(ActionScriptTokenType::Identifier).ok();
        }
        self.parse_block(state)?;
        state.finish_at(cp, ActionScriptElementType::Function.into());
        Ok(())
    }

    fn parse_param_list<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, crate::language::ActionScriptLanguage, S>) -> Result<(), OakError> {
        use crate::lexer::ActionScriptTokenType::*;
        let cp = state.checkpoint();
        state.expect(LeftParen).ok();
        while state.not_at_end() && !state.at(RightParen) {
            state.advance();
        }
        state.expect(RightParen).ok();
        state.finish_at(cp, ActionScriptElementType::ParameterList.into());
        Ok(())
    }

    fn parse_block<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, crate::language::ActionScriptLanguage, S>) -> Result<(), OakError> {
        use crate::lexer::ActionScriptTokenType::*;
        let cp = state.checkpoint();
        state.expect(LeftBrace).ok();
        while state.not_at_end() && !state.at(RightBrace) {
            self.parse_statement(state)?;
        }
        state.expect(RightBrace).ok();
        state.finish_at(cp, ActionScriptElementType::BlockExpression.into());
        Ok(())
    }

    fn parse_import_statement<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, crate::language::ActionScriptLanguage, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(crate::lexer::ActionScriptTokenType::Import).ok();
        while !state.at(crate::lexer::ActionScriptTokenType::Semicolon) && state.not_at_end() {
            state.bump();
        }
        state.eat(crate::lexer::ActionScriptTokenType::Semicolon);
        state.finish_at(cp, ActionScriptElementType::Import.into());
        Ok(())
    }

    fn parse_package_declaration<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, crate::language::ActionScriptLanguage, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(crate::lexer::ActionScriptTokenType::Package).ok();
        if state.at(crate::lexer::ActionScriptTokenType::Identifier) {
            state.bump();
        }
        self.parse_block(state)?;
        state.finish_at(cp, ActionScriptElementType::Package.into());
        Ok(())
    }

    fn parse_class_declaration<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, crate::language::ActionScriptLanguage, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(crate::lexer::ActionScriptTokenType::Class).ok();
        state.expect(crate::lexer::ActionScriptTokenType::Identifier).ok();
        if state.eat(crate::lexer::ActionScriptTokenType::Extends) {
            state.expect(crate::lexer::ActionScriptTokenType::Identifier).ok();
        }
        self.parse_block(state)?;
        state.finish_at(cp, ActionScriptElementType::Class.into());
        Ok(())
    }

    fn parse_interface_declaration<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, crate::language::ActionScriptLanguage, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(crate::lexer::ActionScriptTokenType::Interface).ok();
        state.expect(crate::lexer::ActionScriptTokenType::Identifier).ok();
        self.parse_block(state)?;
        state.finish_at(cp, ActionScriptElementType::Interface.into());
        Ok(())
    }

    fn parse_variable_declaration<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, crate::language::ActionScriptLanguage, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        if state.eat(crate::lexer::ActionScriptTokenType::Var) || state.eat(crate::lexer::ActionScriptTokenType::Const) {
            state.expect(crate::lexer::ActionScriptTokenType::Identifier).ok();
            if state.eat(crate::lexer::ActionScriptTokenType::Colon) {
                state.expect(crate::lexer::ActionScriptTokenType::Identifier).ok();
            }
            if state.eat(crate::lexer::ActionScriptTokenType::Equal) {
                PrattParser::parse(state, 0, self);
            }
            state.eat(crate::lexer::ActionScriptTokenType::Semicolon);
        }
        state.finish_at(cp, ActionScriptElementType::Variable.into());
        Ok(())
    }

    fn parse_if_statement<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, crate::language::ActionScriptLanguage, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(crate::lexer::ActionScriptTokenType::If).ok();
        state.expect(crate::lexer::ActionScriptTokenType::LeftParen).ok();
        PrattParser::parse(state, 0, self);
        state.expect(crate::lexer::ActionScriptTokenType::RightParen).ok();
        self.parse_statement(state)?;
        if state.eat(crate::lexer::ActionScriptTokenType::Else) {
            self.parse_statement(state)?;
        }
        state.finish_at(cp, ActionScriptElementType::IfStatement.into());
        Ok(())
    }

    fn parse_while_statement<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, crate::language::ActionScriptLanguage, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(crate::lexer::ActionScriptTokenType::While).ok();
        state.expect(crate::lexer::ActionScriptTokenType::LeftParen).ok();
        PrattParser::parse(state, 0, self);
        state.expect(crate::lexer::ActionScriptTokenType::RightParen).ok();
        self.parse_statement(state)?;
        state.finish_at(cp, ActionScriptElementType::WhileStatement.into());
        Ok(())
    }

    fn parse_for_statement<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, crate::language::ActionScriptLanguage, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(crate::lexer::ActionScriptTokenType::For).ok();
        state.expect(crate::lexer::ActionScriptTokenType::LeftParen).ok();
        while state.not_at_end() && !state.at(crate::lexer::ActionScriptTokenType::RightParen) {
            state.advance();
        }
        state.expect(crate::lexer::ActionScriptTokenType::RightParen).ok();
        self.parse_statement(state)?;
        state.finish_at(cp, ActionScriptElementType::ForStatement.into());
        Ok(())
    }

    fn parse_return_statement<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, crate::language::ActionScriptLanguage, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(crate::lexer::ActionScriptTokenType::Return).ok();
        if !state.at(crate::lexer::ActionScriptTokenType::Semicolon) {
            PrattParser::parse(state, 0, self);
        }
        state.eat(crate::lexer::ActionScriptTokenType::Semicolon);
        state.finish_at(cp, ActionScriptElementType::ReturnStatement.into());
        Ok(())
    }
}
