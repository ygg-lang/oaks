use crate::{kind::PhpSyntaxKind, language::PhpLanguage, lexer::PhpLexer};
use oak_core::{
    GreenNode, OakError,
    parser::{
        ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer,
        pratt::{Associativity, Pratt, PrattParser, binary},
    },
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, PhpLanguage, S>;

pub struct PhpParser<'config> {
    pub(crate) config: &'config PhpLanguage,
}

impl<'config> PhpParser<'config> {
    pub fn new(config: &'config PhpLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Pratt<PhpLanguage> for PhpParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, PhpLanguage> {
        use crate::kind::PhpSyntaxKind::*;
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(Identifier) | Some(Variable) => {
                state.bump();
                state.finish_at(cp, Identifier.into())
            }
            Some(NumberLiteral) | Some(StringLiteral) | Some(BooleanLiteral) | Some(NullLiteral) => {
                state.bump();
                state.finish_at(cp, Literal.into())
            }
            Some(LeftParen) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(RightParen).ok();
                state.finish_at(cp, ParenthesizedExpression.into())
            }
            _ => {
                state.bump();
                state.finish_at(cp, Error.into())
            }
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, PhpLanguage> {
        self.primary(state)
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, PhpLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, PhpLanguage>> {
        use crate::kind::PhpSyntaxKind::*;
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
                Some(state.finish_at(cp, CallExpression.into()))
            }
            LeftBracket => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(LeftBracket).ok();
                while state.not_at_end() && !state.at(RightBracket) {
                    state.advance();
                }
                state.expect(RightBracket).ok();
                Some(state.finish_at(cp, ArrayAccessExpression.into()))
            }
            Arrow => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(Arrow).ok();
                state.expect(Identifier).ok();
                Some(state.finish_at(cp, MemberAccessExpression.into()))
            }
            _ => Some(binary(state, left, kind, prec, assoc, BinaryExpression.into(), |s, p| PrattParser::parse(s, p, self))),
        }
    }
}

impl<'config> PhpParser<'config> {
    fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::PhpSyntaxKind::*;
        match state.peek_kind() {
            Some(Namespace) => self.parse_namespace_def(state)?,
            Some(Use) => self.parse_use_statement(state)?,
            Some(Class) | Some(Interface) | Some(Trait) | Some(Abstract) | Some(Final) | Some(Public) | Some(Private) | Some(Protected) | Some(Static) => self.parse_declaration(state)?,
            Some(Function) => self.parse_function_def(state)?,
            Some(If) => self.parse_if_statement(state)?,
            Some(While) => self.parse_while_statement(state)?,
            Some(For) => self.parse_for_statement(state)?,
            Some(Foreach) => self.parse_foreach_statement(state)?,
            Some(Return) => self.parse_return_statement(state)?,
            Some(Echo) => self.parse_echo_statement(state)?,
            Some(LeftBrace) => self.parse_compound_statement(state)?,
            _ => {
                PrattParser::parse(state, 0, self);
                state.eat(Semicolon);
            }
        }
        Ok(())
    }

    fn parse_namespace_def<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(crate::kind::PhpSyntaxKind::Namespace).ok();
        state.advance_until_any(&[crate::kind::PhpSyntaxKind::Semicolon, crate::kind::PhpSyntaxKind::LeftBrace]);
        if state.eat(crate::kind::PhpSyntaxKind::LeftBrace) {
            while state.not_at_end() && !state.at(crate::kind::PhpSyntaxKind::RightBrace) {
                self.parse_statement(state)?;
            }
            state.expect(crate::kind::PhpSyntaxKind::RightBrace).ok();
        }
        else {
            state.eat(crate::kind::PhpSyntaxKind::Semicolon);
        }
        state.finish_at(cp, crate::kind::PhpSyntaxKind::NamespaceDef.into());
        Ok(())
    }

    fn parse_use_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(crate::kind::PhpSyntaxKind::Use).ok();
        state.advance_until(crate::kind::PhpSyntaxKind::Semicolon);
        state.eat(crate::kind::PhpSyntaxKind::Semicolon);
        state.finish_at(cp, crate::kind::PhpSyntaxKind::UseStatement.into());
        Ok(())
    }

    fn parse_function_def<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(crate::kind::PhpSyntaxKind::Function).ok();
        state.expect(crate::kind::PhpSyntaxKind::Identifier).ok();
        state.expect(crate::kind::PhpSyntaxKind::LeftParen).ok();
        state.advance_until(crate::kind::PhpSyntaxKind::RightParen);
        state.bump();
        if state.eat(crate::kind::PhpSyntaxKind::Colon) {
            state.advance();
        }
        self.parse_compound_statement(state)?;
        state.finish_at(cp, crate::kind::PhpSyntaxKind::FunctionDef.into());
        Ok(())
    }

    fn parse_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::PhpSyntaxKind::*;
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
                state.finish_at(cp, ClassDef.into());
            }
            Some(Interface) => {
                state.bump();
                state.expect(Identifier).ok();
                self.parse_compound_statement(state)?;
                state.finish_at(cp, InterfaceDef.into());
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
        state.bump(); // if
        state.expect(crate::kind::PhpSyntaxKind::LeftParen).ok();
        state.advance_until(crate::kind::PhpSyntaxKind::RightParen);
        state.eat(crate::kind::PhpSyntaxKind::RightParen);
        self.parse_statement(state)?;
        while state.eat(crate::kind::PhpSyntaxKind::Elseif) {
            state.expect(crate::kind::PhpSyntaxKind::LeftParen).ok();
            state.advance_until(crate::kind::PhpSyntaxKind::RightParen);
            state.eat(crate::kind::PhpSyntaxKind::RightParen);
            self.parse_statement(state)?;
        }
        if state.eat(crate::kind::PhpSyntaxKind::Else) {
            self.parse_statement(state)?;
        }
        state.finish_at(cp, crate::kind::PhpSyntaxKind::IfStatement.into());
        Ok(())
    }

    fn parse_while_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // while
        state.expect(crate::kind::PhpSyntaxKind::LeftParen).ok();
        state.advance_until(crate::kind::PhpSyntaxKind::RightParen);
        state.eat(crate::kind::PhpSyntaxKind::RightParen);
        self.parse_statement(state)?;
        state.finish_at(cp, crate::kind::PhpSyntaxKind::WhileStatement.into());
        Ok(())
    }

    fn parse_for_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // for
        state.expect(crate::kind::PhpSyntaxKind::LeftParen).ok();
        state.advance_until(crate::kind::PhpSyntaxKind::RightParen);
        state.eat(crate::kind::PhpSyntaxKind::RightParen);
        self.parse_statement(state)?;
        state.finish_at(cp, crate::kind::PhpSyntaxKind::ForStatement.into());
        Ok(())
    }

    fn parse_foreach_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // foreach
        state.expect(crate::kind::PhpSyntaxKind::LeftParen).ok();
        state.advance_until(crate::kind::PhpSyntaxKind::RightParen);
        state.eat(crate::kind::PhpSyntaxKind::RightParen);
        self.parse_statement(state)?;
        state.finish_at(cp, crate::kind::PhpSyntaxKind::ForeachStatement.into());
        Ok(())
    }

    fn parse_compound_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(crate::kind::PhpSyntaxKind::LeftBrace).ok();
        while state.not_at_end() && !state.at(crate::kind::PhpSyntaxKind::RightBrace) {
            self.parse_statement(state)?;
        }
        state.expect(crate::kind::PhpSyntaxKind::RightBrace).ok();
        state.finish_at(cp, crate::kind::PhpSyntaxKind::CompoundStatement.into());
        Ok(())
    }

    fn parse_return_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // return
        if !state.at(crate::kind::PhpSyntaxKind::Semicolon) && !state.at(crate::kind::PhpSyntaxKind::RightBrace) {
            PrattParser::parse(state, 0, self);
        }
        state.eat(crate::kind::PhpSyntaxKind::Semicolon);
        state.finish_at(cp, crate::kind::PhpSyntaxKind::ReturnStatement.into());
        Ok(())
    }

    fn parse_echo_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // echo
        state.advance_until(crate::kind::PhpSyntaxKind::Semicolon);
        state.eat(crate::kind::PhpSyntaxKind::Semicolon);
        state.finish_at(cp, crate::kind::PhpSyntaxKind::EchoStatement.into());
        Ok(())
    }
}

impl<'config> Parser<PhpLanguage> for PhpParser<'config> {
    fn parse<'s, S: Source + ?Sized>(&self, text: &'s S, edits: &[TextEdit], cache: &'s mut impl ParseCache<PhpLanguage>) -> ParseOutput<'s, PhpLanguage> {
        let lexer = PhpLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let cp = state.checkpoint();
            while state.not_at_end() {
                PrattParser::parse(state, 0, self);
            }
            Ok(state.finish_at(cp, PhpSyntaxKind::Root.into()))
        })
    }
}
