/// Element types for the ActionScript language.
pub mod element_type;

pub use element_type::ActionScriptElementType;

use crate::{language::ActionScriptLanguage, lexer::ActionScriptLexer};
use oak_core::{
    GreenNode, TokenType,
    parser::{Associativity, ParseCache, ParseOutput, Parser, ParserState, Pratt, PrattParser, binary, parse_with_lexer, unary},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, ActionScriptLanguage, S>;

/// A parser for the ActionScript language.
pub struct ActionScriptParser<'config> {
    pub(crate) config: &'config ActionScriptLanguage,
}

impl<'config> Pratt<ActionScriptLanguage> for ActionScriptParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, ActionScriptLanguage> {
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(crate::lexer::ActionScriptTokenType::Identifier) => {
                state.bump();
                state.finish_at(cp, crate::parser::element_type::ActionScriptElementType::IdentifierExpression)
            }
            Some(k) if k.is_literal() => {
                state.bump();
                state.finish_at(cp, crate::parser::element_type::ActionScriptElementType::LiteralExpression)
            }
            Some(crate::lexer::ActionScriptTokenType::LeftParen) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(crate::lexer::ActionScriptTokenType::RightParen).ok();
                state.finish_at(cp, crate::parser::element_type::ActionScriptElementType::ParenthesizedExpression)
            }
            _ => {
                state.bump();
                state.finish_at(cp, crate::parser::element_type::ActionScriptElementType::Error)
            }
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, ActionScriptLanguage> {
        use crate::{lexer::ActionScriptTokenType as TT, parser::element_type::ActionScriptElementType as ET};
        let kind = match state.peek_kind() {
            Some(k) => k,
            None => return self.primary(state),
        };

        match kind {
            TT::Minus | TT::LogicalNot | TT::BitwiseNot => unary(state, kind, 13, ET::UnaryExpression.into(), |s, p| PrattParser::parse(s, p, self)),
            _ => self.primary(state),
        }
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, ActionScriptLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, ActionScriptLanguage>> {
        use crate::{lexer::ActionScriptTokenType as TT, parser::element_type::ActionScriptElementType as ET};
        let kind = state.peek_kind()?;

        let (prec, assoc) = match kind {
            TT::Equal
            | TT::PlusAssign
            | TT::MinusAssign
            | TT::StarAssign
            | TT::SlashAssign
            | TT::PercentAssign
            | TT::LeftShiftAssign
            | TT::RightShiftAssign
            | TT::UnsignedRightShiftAssign
            | TT::BitwiseAndAssign
            | TT::BitwiseOrAssign
            | TT::BitwiseXorAssign => (1, Associativity::Right),
            TT::LogicalOr => (3, Associativity::Left),
            TT::LogicalAnd => (4, Associativity::Left),
            TT::EqualEqual | TT::NotEqual | TT::EqualEqualEqual | TT::NotEqualEqual => (5, Associativity::Left),
            TT::LessThan | TT::LessEqual | TT::GreaterThan | TT::GreaterEqual | TT::Is | TT::Instanceof => (6, Associativity::Left),
            TT::BitwiseOr => (7, Associativity::Left),
            TT::BitwiseXor => (8, Associativity::Left),
            TT::BitwiseAnd => (9, Associativity::Left),
            TT::LeftShift | TT::RightShift | TT::UnsignedRightShift => (10, Associativity::Left),
            TT::Plus | TT::Minus => (11, Associativity::Left),
            TT::Star | TT::Slash | TT::Percent => (12, Associativity::Left),
            TT::LeftParen | TT::LeftBracket | TT::Dot => (14, Associativity::Left),
            _ => return None,
        };

        if prec < min_precedence {
            return None;
        }

        match kind {
            TT::LeftParen => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(TT::LeftParen).ok();
                if !state.at(TT::RightParen) {
                    loop {
                        PrattParser::parse(state, 0, self);
                        if !state.eat(TT::Comma) {
                            break;
                        }
                    }
                }
                state.expect(TT::RightParen).ok();
                Some(state.finish_at(cp, ET::CallExpression))
            }
            TT::LeftBracket => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(TT::LeftBracket).ok();
                PrattParser::parse(state, 0, self);
                state.expect(TT::RightBracket).ok();
                Some(state.finish_at(cp, ET::IndexExpression))
            }
            TT::Dot => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(TT::Dot).ok();
                state.expect(TT::Identifier).ok();
                Some(state.finish_at(cp, ET::FieldExpression))
            }
            _ => Some(binary(state, left, kind, prec, assoc, ET::BinaryExpression.into(), |s, p| PrattParser::parse(s, p, self))),
        }
    }
}

impl<'config> ActionScriptParser<'config> {
    /// Creates a new ActionScript parser with the given configuration.
    pub fn new(config: &'config ActionScriptLanguage) -> Self {
        Self { config }
    }

    pub(crate) fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
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

    fn parse_function<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        use crate::lexer::ActionScriptTokenType;
        let cp = state.checkpoint();
        state.expect(ActionScriptTokenType::Function).ok();
        state.expect(ActionScriptTokenType::Identifier).ok();
        self.parse_param_list(state)?;
        if state.eat(ActionScriptTokenType::Colon) {
            state.expect(ActionScriptTokenType::Identifier).ok();
        }
        self.parse_block(state)?;
        state.finish_at(cp, ActionScriptElementType::Function);
        Ok(())
    }

    fn parse_param_list<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        use crate::lexer::ActionScriptTokenType::*;
        let cp = state.checkpoint();
        state.expect(LeftParen).ok();
        while state.not_at_end() && !state.at(RightParen) {
            state.advance()
        }
        state.expect(RightParen).ok();
        state.finish_at(cp, ActionScriptElementType::ParameterList);
        Ok(())
    }

    fn parse_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        use crate::lexer::ActionScriptTokenType::*;
        let cp = state.checkpoint();
        state.expect(LeftBrace).ok();
        while state.not_at_end() && !state.at(RightBrace) {
            self.parse_statement(state)?
        }
        state.expect(RightBrace).ok();
        state.finish_at(cp, ActionScriptElementType::BlockExpression);
        Ok(())
    }

    fn parse_import_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        let cp = state.checkpoint();
        state.expect(crate::lexer::ActionScriptTokenType::Import).ok();
        while !state.at(crate::lexer::ActionScriptTokenType::Semicolon) && state.not_at_end() {
            state.bump()
        }
        state.eat(crate::lexer::ActionScriptTokenType::Semicolon);
        state.finish_at(cp, ActionScriptElementType::Import);
        Ok(())
    }

    fn parse_package_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        let cp = state.checkpoint();
        state.expect(crate::lexer::ActionScriptTokenType::Package).ok();
        if state.at(crate::lexer::ActionScriptTokenType::Identifier) {
            state.bump()
        }
        self.parse_block(state)?;
        state.finish_at(cp, ActionScriptElementType::Package);
        Ok(())
    }

    fn parse_class_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        let cp = state.checkpoint();
        state.expect(crate::lexer::ActionScriptTokenType::Class).ok();
        state.expect(crate::lexer::ActionScriptTokenType::Identifier).ok();
        if state.eat(crate::lexer::ActionScriptTokenType::Extends) {
            state.expect(crate::lexer::ActionScriptTokenType::Identifier).ok();
        }
        self.parse_block(state)?;
        state.finish_at(cp, ActionScriptElementType::Class);
        Ok(())
    }

    fn parse_interface_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        let cp = state.checkpoint();
        state.expect(crate::lexer::ActionScriptTokenType::Interface).ok();
        state.expect(crate::lexer::ActionScriptTokenType::Identifier).ok();
        self.parse_block(state)?;
        state.finish_at(cp, ActionScriptElementType::Interface);
        Ok(())
    }

    fn parse_variable_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
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
        state.finish_at(cp, ActionScriptElementType::Variable);
        Ok(())
    }

    fn parse_if_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        let cp = state.checkpoint();
        state.expect(crate::lexer::ActionScriptTokenType::If).ok();
        state.expect(crate::lexer::ActionScriptTokenType::LeftParen).ok();
        PrattParser::parse(state, 0, self);
        state.expect(crate::lexer::ActionScriptTokenType::RightParen).ok();
        self.parse_statement(state)?;
        if state.eat(crate::lexer::ActionScriptTokenType::Else) {
            self.parse_statement(state)?;
        }
        state.finish_at(cp, ActionScriptElementType::IfStatement);
        Ok(())
    }

    fn parse_while_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        let cp = state.checkpoint();
        state.expect(crate::lexer::ActionScriptTokenType::While).ok();
        state.expect(crate::lexer::ActionScriptTokenType::LeftParen).ok();
        PrattParser::parse(state, 0, self);
        state.expect(crate::lexer::ActionScriptTokenType::RightParen).ok();
        self.parse_statement(state)?;
        state.finish_at(cp, ActionScriptElementType::WhileStatement);
        Ok(())
    }

    fn parse_for_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        let cp = state.checkpoint();
        state.expect(crate::lexer::ActionScriptTokenType::For).ok();
        state.expect(crate::lexer::ActionScriptTokenType::LeftParen).ok();
        while state.not_at_end() && !state.at(crate::lexer::ActionScriptTokenType::RightParen) {
            state.advance();
        }
        state.expect(crate::lexer::ActionScriptTokenType::RightParen).ok();
        self.parse_statement(state)?;
        state.finish_at(cp, ActionScriptElementType::ForStatement);
        Ok(())
    }

    fn parse_return_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        let cp = state.checkpoint();
        state.expect(crate::lexer::ActionScriptTokenType::Return).ok();
        if !state.at(crate::lexer::ActionScriptTokenType::Semicolon) {
            PrattParser::parse(state, 0, self);
        }
        state.eat(crate::lexer::ActionScriptTokenType::Semicolon);
        state.finish_at(cp, ActionScriptElementType::ReturnStatement);
        Ok(())
    }
}

impl<'config> Parser<ActionScriptLanguage> for ActionScriptParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<ActionScriptLanguage>) -> ParseOutput<'a, ActionScriptLanguage> {
        let lexer = ActionScriptLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let cp = state.checkpoint();
            while state.not_at_end() {
                if state.current().map(|t| t.kind.is_ignored()).unwrap_or(false) {
                    state.advance();
                    continue;
                }
                self.parse_statement(state)?
            }
            Ok(state.finish_at(cp, ActionScriptElementType::SourceFile))
        })
    }
}
