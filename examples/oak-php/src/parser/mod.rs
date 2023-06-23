pub mod element_type;

use crate::{
    language::PhpLanguage,
    lexer::{PhpLexer, PhpTokenType},
    parser::element_type::PhpElementType,
};
use oak_core::{
    GreenNode, OakError,
    parser::{
        ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer,
        pratt::{Associativity, Pratt, PrattParser, binary},
    },
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, PhpLanguage, S>;

/// Parser for the PHP language.
///
/// This parser transforms a stream of tokens into a green tree of [`PhpTokenType`] nodes.
pub struct PhpParser<'config> {
    pub(crate) config: &'config PhpLanguage,
}

impl<'config> PhpParser<'config> {
    /// Creates a new `PhpParser` with the given language configuration.
    pub fn new(config: &'config PhpLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Pratt<PhpLanguage> for PhpParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, PhpLanguage> {
        use crate::lexer::token_type::PhpTokenType::*;
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(Identifier) | Some(Variable) => {
                state.bump();
                state.finish_at(cp, PhpElementType::Identifier)
            }
            Some(NumberLiteral) | Some(StringLiteral) | Some(BooleanLiteral) | Some(NullLiteral) => {
                state.bump();
                state.finish_at(cp, PhpElementType::Literal)
            }
            Some(LeftParen) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(RightParen).ok();
                state.finish_at(cp, PhpElementType::ParenthesizedExpression)
            }
            _ => {
                state.bump();
                state.finish_at(cp, PhpElementType::Error)
            }
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, PhpLanguage> {
        self.primary(state)
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, PhpLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, PhpLanguage>> {
        use crate::lexer::PhpTokenType::*;
        let kind = state.peek_kind()?;

        let (prec, assoc) = match kind {
            Assign | PlusAssign | MinusAssign | MultiplyAssign | DivideAssign | ModuloAssign | PowerAssign | ConcatAssign | BitwiseAndAssign | BitwiseOrAssign | BitwiseXorAssign | LeftShiftAssign | RightShiftAssign | NullCoalesceAssign => {
                (1, Associativity::Right)
            }
            LogicalOr => (2, Associativity::Left),
            LogicalAnd => (3, Associativity::Left),
            Equal | Identical | NotEqual | NotIdentical | Less | Greater | LessEqual | GreaterEqual | Spaceship => (4, Associativity::Left),
            Plus | Minus | Concat => (10, Associativity::Left),
            Multiply | Divide | Modulo => (11, Associativity::Left),
            LeftParen | LeftBracket | Arrow => (15, Associativity::Left),
            _ => return None,
        };

        if prec < min_precedence {
            return None;
        }

        match kind {
            LeftParen => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(LeftParen).ok();
                while state.not_at_end() && !state.at(RightParen) {
                    state.advance();
                }
                state.expect(RightParen).ok();
                Some(state.finish_at(cp, PhpElementType::CallExpression))
            }
            LeftBracket => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(LeftBracket).ok();
                while state.not_at_end() && !state.at(RightBracket) {
                    state.advance();
                }
                state.expect(RightBracket).ok();
                Some(state.finish_at(cp, PhpElementType::ArrayAccessExpression))
            }
            Arrow => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(Arrow).ok();
                state.expect(Identifier).ok();
                Some(state.finish_at(cp, PhpElementType::MemberAccessExpression))
            }
            _ => Some(binary(state, left, kind, prec, assoc, PhpElementType::BinaryExpression, |s, p| PrattParser::parse(s, p, self))),
        }
    }
}

impl<'config> PhpParser<'config> {
    fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::PhpTokenType::*;
        match state.peek_kind() {
            Some(Namespace) => self.parse_namespace_def(state),
            Some(Use) => self.parse_use_statement(state),
            Some(Class) | Some(Interface) | Some(Trait) | Some(Abstract) | Some(Final) | Some(Public) | Some(Private) | Some(Protected) | Some(Static) => self.parse_declaration(state),
            Some(Function) => self.parse_function_def(state),
            Some(If) => self.parse_if_statement(state),
            Some(While) => self.parse_while_statement(state),
            Some(For) => self.parse_for_statement(state),
            Some(Foreach) => self.parse_foreach_statement(state),
            Some(Return) => self.parse_return_statement(state),
            Some(Echo) => self.parse_echo_statement(state),
            Some(LeftBrace) => self.parse_compound_statement(state),
            _ => self.parse_expression_statement(state),
        }
    }

    fn parse_namespace_def<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(crate::lexer::PhpTokenType::Namespace).ok();
        state.advance_until_any(&[crate::lexer::PhpTokenType::Semicolon, crate::lexer::PhpTokenType::LeftBrace]);
        if state.eat(crate::lexer::PhpTokenType::LeftBrace) {
            while state.not_at_end() && !state.at(crate::lexer::PhpTokenType::RightBrace) {
                self.parse_statement(state)?;
            }
            state.expect(crate::lexer::PhpTokenType::RightBrace).ok();
        }
        else {
            state.eat(crate::lexer::PhpTokenType::Semicolon);
        }
        state.finish_at(cp, PhpElementType::NamespaceDef);
        Ok(())
    }

    fn parse_use_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(crate::lexer::PhpTokenType::Use).ok();
        state.advance_until(crate::lexer::PhpTokenType::Semicolon);
        state.eat(crate::lexer::PhpTokenType::Semicolon);
        state.finish_at(cp, PhpElementType::UseStatement);
        Ok(())
    }

    fn parse_function_def<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(crate::lexer::PhpTokenType::Function).ok();
        state.expect(crate::lexer::PhpTokenType::Identifier).ok();
        state.expect(crate::lexer::PhpTokenType::LeftParen).ok();
        while state.not_at_end() && !state.at(crate::lexer::PhpTokenType::RightParen) {
            state.advance();
        }
        state.expect(crate::lexer::PhpTokenType::RightParen).ok();
        self.parse_compound_statement(state)?;
        state.finish_at(cp, PhpElementType::FunctionDef);
        Ok(())
    }

    fn parse_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::PhpTokenType::*;
        let cp = state.checkpoint();
        while state.not_at_end() && matches!(state.peek_kind(), Some(Public) | Some(Private) | Some(Protected) | Some(Static) | Some(Abstract) | Some(Final)) {
            state.bump();
        }

        match state.peek_kind() {
            Some(Class) => {
                state.bump();
                state.expect(Identifier).ok();
                state.advance_until(LeftBrace);
                self.parse_compound_statement(state)?;
                state.finish_at(cp, PhpElementType::ClassDef);
            }
            Some(Interface) => {
                state.bump();
                state.expect(Identifier).ok();
                self.parse_compound_statement(state)?;
                state.finish_at(cp, PhpElementType::InterfaceDef);
            }
            Some(Function) => self.parse_function_def(state)?,
            _ => {
                state.advance_until_any(&[Semicolon, LeftBrace]);
                if state.eat(LeftBrace) {
                    while state.not_at_end() && !state.at(RightBrace) {
                        self.parse_statement(state)?;
                    }
                    state.expect(RightBrace).ok();
                }
                else {
                    state.eat(Semicolon);
                }
            }
        }
        Ok(())
    }

    fn parse_if_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(crate::lexer::PhpTokenType::If).ok();
        state.expect(crate::lexer::PhpTokenType::LeftParen).ok();
        PrattParser::parse(state, 0, self);
        state.expect(crate::lexer::PhpTokenType::RightParen).ok();
        self.parse_statement(state)?;
        state.finish_at(cp, PhpElementType::IfStatement);
        Ok(())
    }

    fn parse_while_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(crate::lexer::PhpTokenType::While).ok();
        state.expect(crate::lexer::PhpTokenType::LeftParen).ok();
        PrattParser::parse(state, 0, self);
        state.expect(crate::lexer::PhpTokenType::RightParen).ok();
        self.parse_statement(state)?;
        state.finish_at(cp, PhpElementType::WhileStatement);
        Ok(())
    }

    fn parse_for_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(crate::lexer::PhpTokenType::For).ok();
        state.expect(crate::lexer::PhpTokenType::LeftParen).ok();
        state.expect(crate::lexer::PhpTokenType::RightParen).ok();
        self.parse_statement(state)?;
        state.finish_at(cp, PhpElementType::ForStatement);
        Ok(())
    }

    fn parse_foreach_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(crate::lexer::PhpTokenType::Foreach).ok();
        state.expect(crate::lexer::PhpTokenType::LeftParen).ok();
        state.expect(crate::lexer::PhpTokenType::RightParen).ok();
        self.parse_statement(state)?;
        state.finish_at(cp, PhpElementType::ForeachStatement);
        Ok(())
    }

    fn parse_compound_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(crate::lexer::PhpTokenType::LeftBrace).ok();
        while state.not_at_end() && !state.at(crate::lexer::PhpTokenType::RightBrace) {
            self.parse_statement(state)?;
        }
        state.expect(crate::lexer::PhpTokenType::RightBrace).ok();
        state.finish_at(cp, PhpElementType::CompoundStatement);
        Ok(())
    }

    fn parse_return_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(crate::lexer::PhpTokenType::Return).ok();
        PrattParser::parse(state, 0, self);
        state.eat(crate::lexer::PhpTokenType::Semicolon);
        state.finish_at(cp, PhpElementType::ReturnStatement);
        Ok(())
    }

    fn parse_echo_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(crate::lexer::PhpTokenType::Echo).ok();
        PrattParser::parse(state, 0, self);
        state.eat(crate::lexer::PhpTokenType::Semicolon);
        state.finish_at(cp, PhpElementType::EchoStatement);
        Ok(())
    }

    fn parse_expression_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        PrattParser::parse(state, 0, self);
        state.eat(crate::lexer::PhpTokenType::Semicolon);
        Ok(())
    }
}

impl<'config> Parser<PhpLanguage> for PhpParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<PhpLanguage>) -> ParseOutput<'a, PhpLanguage> {
        let lexer = crate::lexer::PhpLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let cp = state.checkpoint();
            while state.not_at_end() {
                if let Err(_) = self.parse_statement(state) {
                    state.advance();
                }
            }
            Ok(state.finish_at(cp, PhpElementType::Root))
        })
    }
}
