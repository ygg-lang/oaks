use crate::language::CSharpLanguage;
pub mod element_type;
pub use element_type::CSharpElementType;
use oak_core::{
    GreenNode, OakError,
    parser::{
        ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer,
        pratt::{Associativity, Pratt, PrattParser, binary},
    },
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, CSharpLanguage, S>;

pub struct CSharpParser<'config> {
    pub(crate) _language: &'config CSharpLanguage,
}

impl<'config> Pratt<CSharpLanguage> for CSharpParser<'config> {
    fn primary<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, CSharpLanguage, S>) -> &'a GreenNode<'a, CSharpLanguage> {
        use crate::{lexer::CSharpTokenType::*, parser::CSharpElementType::*};
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(Identifier) => {
                state.bump();
                state.finish_at(cp, IdentifierName)
            }
            Some(Number) | Some(NumberLiteral) | Some(String) | Some(StringLiteral) | Some(TrueKeyword) | Some(FalseKeyword) | Some(NullKeyword) => {
                state.bump();
                state.finish_at(cp, LiteralExpression)
            }
            Some(LeftParen) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(RightParen).ok();
                state.finish_at(cp, BinaryExpression) // 简化处理
            }
            _ => {
                state.bump();
                state.finish_at(cp, Root)
            }
        }
    }

    fn prefix<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, CSharpLanguage, S>) -> &'a GreenNode<'a, CSharpLanguage> {
        self.primary(state)
    }

    fn infix<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, CSharpLanguage, S>, left: &'a GreenNode<'a, CSharpLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, CSharpLanguage>> {
        use crate::{lexer::CSharpTokenType::*, parser::CSharpElementType::*};
        let kind = state.peek_kind()?;

        let (prec, assoc) = match kind {
            Assign | PlusAssign | MinusAssign | StarAssign | SlashAssign | PercentAssign | AndAssign | OrAssign | XorAssign | LeftShiftAssign | RightShiftAssign | QuestionQuestionAssign => (1, Associativity::Right),
            LogicalOr => (2, Associativity::Left),
            LogicalAnd => (3, Associativity::Left),
            Equal | NotEqual | Less | Greater | LessEqual | GreaterEqual | IsKeyword | AsKeyword => (4, Associativity::Left),
            Plus | Minus => (10, Associativity::Left),
            Star | Slash | Percent => (11, Associativity::Left),
            LeftParen | LeftBracket | Dot => (15, Associativity::Left),
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
                    state.bump();
                }
                state.expect(RightParen).ok();
                Some(state.finish_at(cp, InvocationExpression))
            }
            LeftBracket => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(LeftBracket).ok();
                while state.not_at_end() && !state.at(RightBracket) {
                    state.bump();
                }
                state.expect(RightBracket).ok();
                Some(state.finish_at(cp, ElementAccessExpression))
            }
            Dot => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(Dot).ok();
                state.expect(Identifier).ok();
                Some(state.finish_at(cp, MemberAccessExpression))
            }
            Assign | PlusAssign | MinusAssign | StarAssign | SlashAssign | PercentAssign | AndAssign | OrAssign | XorAssign | LeftShiftAssign | RightShiftAssign | QuestionQuestionAssign => {
                Some(binary(state, left, kind, prec, assoc, AssignmentExpression, |s, p| PrattParser::parse(s, p, self)))
            }
            _ => Some(binary(state, left, kind, prec, assoc, BinaryExpression, |s, p| PrattParser::parse(s, p, self))),
        }
    }
}

impl<'config> CSharpParser<'config> {
    pub fn new(language: &'config CSharpLanguage) -> Self {
        Self { _language: language }
    }

    fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::CSharpTokenType::*;
        match state.peek_kind() {
            Some(NamespaceKeyword) => self.parse_namespace_declaration(state)?,
            Some(UsingKeyword) => self.parse_using_directive(state)?,
            Some(ClassKeyword) | Some(StructKeyword) | Some(InterfaceKeyword) | Some(EnumKeyword) | Some(PublicKeyword) | Some(PrivateKeyword) | Some(ProtectedKeyword) | Some(InternalKeyword) | Some(StaticKeyword) => self.parse_declaration(state)?,
            Some(IfKeyword) => self.parse_if_statement(state)?,
            Some(WhileKeyword) => self.parse_while_statement(state)?,
            Some(ForKeyword) => self.parse_for_statement(state)?,
            Some(ReturnKeyword) => self.parse_return_statement(state)?,
            Some(LeftBrace) => self.parse_block(state)?,
            _ => {
                PrattParser::parse(state, 0, self);
                state.eat(Semicolon);
            }
        }
        Ok(())
    }

    fn parse_namespace_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::{lexer::CSharpTokenType::*, parser::CSharpElementType::*};
        let cp = state.checkpoint();
        state.expect(NamespaceKeyword).ok();
        while state.not_at_end() && !state.at(LeftBrace) {
            state.bump();
        }
        self.parse_block(state)?;
        state.finish_at(cp, NamespaceDeclaration);
        Ok(())
    }

    fn parse_using_directive<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::{lexer::CSharpTokenType::*, parser::CSharpElementType::*};
        let cp = state.checkpoint();
        state.expect(UsingKeyword).ok();
        while state.not_at_end() && !state.at(Semicolon) {
            state.bump();
        }
        state.eat(Semicolon);
        state.finish_at(cp, UsingDirective);
        Ok(())
    }

    fn parse_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::{lexer::CSharpTokenType::*, parser::CSharpElementType::*};
        // 处理修饰符
        while state.not_at_end() && matches!(state.peek_kind(), Some(PublicKeyword) | Some(PrivateKeyword) | Some(ProtectedKeyword) | Some(InternalKeyword) | Some(StaticKeyword) | Some(ReadonlyKeyword) | Some(AbstractKeyword)) {
            state.bump();
        }

        match state.peek_kind() {
            Some(ClassKeyword) => {
                let cp = state.checkpoint();
                state.bump();
                state.expect(Identifier).ok();
                while state.not_at_end() && !state.at(LeftBrace) {
                    state.bump();
                }
                self.parse_block(state)?;
                state.finish_at(cp, ClassDeclaration);
            }
            Some(InterfaceKeyword) => {
                let cp = state.checkpoint();
                state.bump();
                state.expect(Identifier).ok();
                self.parse_block(state)?;
                state.finish_at(cp, InterfaceDeclaration);
            }
            _ => {
                while state.not_at_end() && !state.at(Semicolon) && !state.at(LeftBrace) {
                    state.bump();
                }
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
        use crate::{lexer::CSharpTokenType::*, parser::CSharpElementType::*};
        let cp = state.checkpoint();
        state.bump(); // if
        state.expect(LeftParen).ok();
        PrattParser::parse(state, 0, self);
        state.expect(RightParen).ok();
        self.parse_statement(state)?;
        if state.eat(ElseKeyword) {
            self.parse_statement(state)?;
        }
        state.finish_at(cp, IfStatement);
        Ok(())
    }

    fn parse_while_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::{lexer::CSharpTokenType::*, parser::CSharpElementType::*};
        let cp = state.checkpoint();
        state.bump(); // while
        state.expect(LeftParen).ok();
        PrattParser::parse(state, 0, self);
        state.expect(RightParen).ok();
        self.parse_statement(state)?;
        state.finish_at(cp, WhileStatement);
        Ok(())
    }

    fn parse_for_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::{lexer::CSharpTokenType::*, parser::CSharpElementType::*};
        let cp = state.checkpoint();
        state.bump(); // for
        state.expect(LeftParen).ok();
        PrattParser::parse(state, 0, self);
        state.expect(RightParen).ok();
        self.parse_statement(state)?;
        state.finish_at(cp, ForStatement);
        Ok(())
    }

    fn parse_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::{lexer::CSharpTokenType::*, parser::CSharpElementType::*};
        let cp = state.checkpoint();
        state.expect(LeftBrace).ok();
        while state.not_at_end() && !state.at(RightBrace) {
            self.parse_statement(state)?;
        }
        state.expect(RightBrace).ok();
        state.finish_at(cp, Block);
        Ok(())
    }

    fn parse_return_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::{lexer::CSharpTokenType::*, parser::CSharpElementType::*};
        let cp = state.checkpoint();
        state.bump(); // return
        if !state.at(Semicolon) && !state.at(RightBrace) {
            PrattParser::parse(state, 0, self);
        }
        state.eat(Semicolon);
        state.finish_at(cp, ReturnStatement);
        Ok(())
    }
}

impl<'config> Parser<CSharpLanguage> for CSharpParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<CSharpLanguage>) -> ParseOutput<'a, CSharpLanguage> {
        let lexer = crate::lexer::CSharpLexer::new(self._language);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let cp = state.checkpoint();
            while state.not_at_end() {
                self.parse_statement(state)?;
            }
            Ok(state.finish_at(cp, crate::parser::CSharpElementType::Root))
        })
    }
}
