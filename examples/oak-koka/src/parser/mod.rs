#![doc = include_str!("readme.md")]

/// Element types for the Koka parser.
pub mod element_type;

use crate::{language::KokaLanguage, lexer::KokaLexer};
use oak_core::{
    GreenNode, OakError, TextEdit,
    parser::{
        ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer,
        pratt::{Associativity, Pratt, PrattParser, binary, unary},
    },
    source::Source,
};

pub(crate) type State<'a, S> = ParserState<'a, KokaLanguage, S>;

/// A parser for the Koka language.
pub struct KokaParser<'config> {
    pub(crate) config: &'config KokaLanguage,
}

impl<'config> KokaParser<'config> {
    /// Creates a new Koka parser.
    pub fn new(config: &'config KokaLanguage) -> Self {
        Self { config }
    }

    fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        use crate::lexer::token_type::KokaTokenType::*;
        while let Some(kind) = state.peek_kind() {
            if matches!(kind, Whitespace | Newline | Comment) {
                state.bump();
            }
            else {
                break;
            }
        }
    }

    fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::KokaTokenType::*;
        self.skip_trivia(state);
        match state.peek_kind() {
            Some(Fun) => self.parse_function_declaration(state)?,
            Some(Class) => self.parse_class_declaration(state)?,
            Some(Val) | Some(Var) => self.parse_variable_declaration(state)?,
            Some(If) => self.parse_if_statement(state)?,
            Some(While) => self.parse_while_statement(state)?,
            Some(Return) => self.parse_return_statement(state)?,
            Some(LBrace) => self.parse_block(state)?,
            _ => {
                PrattParser::parse(state, 0, self);
                self.skip_trivia(state);
                state.eat(Semi);
            }
        }
        Ok(())
    }

    fn parse_class_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::KokaTokenType::*;
        let cp = state.checkpoint();
        state.expect(Class).ok();
        self.skip_trivia(state);
        state.expect(Identifier).ok();
        self.skip_trivia(state);
        if state.eat(LParen) {
            while state.not_at_end() && !state.at(RParen) {
                let p_cp = state.checkpoint();
                self.skip_trivia(state);
                state.eat(Val);
                state.eat(Var);
                self.skip_trivia(state);
                state.expect(Identifier).ok();
                self.skip_trivia(state);
                if state.eat(Colon) {
                    self.skip_trivia(state);
                    state.expect(Identifier).ok();
                }
                state.finish_at(p_cp, crate::parser::element_type::KokaElementType::Parameter);
                self.skip_trivia(state);
                state.eat(Comma);
            }
            self.skip_trivia(state);
            state.expect(RParen).ok();
        }
        self.skip_trivia(state);
        if state.at(LBrace) {
            self.parse_block(state)?;
        }
        state.finish_at(cp, crate::parser::element_type::KokaElementType::ClassDeclaration);
        Ok(())
    }

    fn parse_function_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::KokaTokenType::*;
        let cp = state.checkpoint();
        state.expect(Fun).ok();
        self.skip_trivia(state);
        state.expect(Identifier).ok();
        self.skip_trivia(state);
        if state.eat(LParen) {
            while state.not_at_end() && !state.at(RParen) {
                let p_cp = state.checkpoint();
                self.skip_trivia(state);
                state.expect(Identifier).ok();
                self.skip_trivia(state);
                if state.eat(Colon) {
                    self.skip_trivia(state);
                    state.expect(Identifier).ok();
                }
                state.finish_at(p_cp, crate::parser::element_type::KokaElementType::Parameter);
                self.skip_trivia(state);
                state.eat(Comma);
            }
            self.skip_trivia(state);
            state.expect(RParen).ok();
        }
        self.skip_trivia(state);
        if state.eat(Colon) {
            self.skip_trivia(state);
            state.expect(Identifier).ok();
        }
        self.skip_trivia(state);
        self.parse_block(state)?;
        state.finish_at(cp, crate::parser::element_type::KokaElementType::FunctionDeclaration);
        Ok(())
    }

    fn parse_variable_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::KokaTokenType::*;
        let cp = state.checkpoint();
        state.bump(); // val/var
        self.skip_trivia(state);
        state.expect(Identifier).ok();
        self.skip_trivia(state);
        if state.eat(Colon) {
            self.skip_trivia(state);
            state.expect(Identifier).ok();
        }
        self.skip_trivia(state);
        if state.eat(Assign) {
            self.skip_trivia(state);
            PrattParser::parse(state, 0, self);
        }
        self.skip_trivia(state);
        state.eat(Semi);
        state.finish_at(cp, crate::parser::element_type::KokaElementType::VariableDeclaration);
        Ok(())
    }

    fn parse_if_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::KokaTokenType::*;
        let cp = state.checkpoint();
        state.expect(If).ok();
        self.skip_trivia(state);
        state.expect(LParen).ok();
        self.skip_trivia(state);
        PrattParser::parse(state, 0, self);
        self.skip_trivia(state);
        state.expect(RParen).ok();
        self.skip_trivia(state);
        self.parse_statement(state)?;
        self.skip_trivia(state);
        if state.eat(Else) {
            self.skip_trivia(state);
            self.parse_statement(state)?;
        }
        state.finish_at(cp, crate::parser::element_type::KokaElementType::IfStatement);
        Ok(())
    }

    fn parse_while_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::KokaTokenType::*;
        let cp = state.checkpoint();
        state.expect(While).ok();
        self.skip_trivia(state);
        state.expect(LParen).ok();
        self.skip_trivia(state);
        PrattParser::parse(state, 0, self);
        self.skip_trivia(state);
        state.expect(RParen).ok();
        self.skip_trivia(state);
        self.parse_statement(state)?;
        state.finish_at(cp, crate::parser::element_type::KokaElementType::WhileStatement);
        Ok(())
    }

    fn parse_return_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::KokaTokenType::*;
        let cp = state.checkpoint();
        state.expect(Return).ok();
        self.skip_trivia(state);
        if state.not_at_end() && !state.at(Semi) && !state.at(RBrace) {
            PrattParser::parse(state, 0, self);
        }
        self.skip_trivia(state);
        state.eat(Semi);
        state.finish_at(cp, crate::parser::element_type::KokaElementType::ReturnStatement);
        Ok(())
    }

    fn parse_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::KokaTokenType::*;
        let cp = state.checkpoint();
        state.expect(LBrace).ok();
        while state.not_at_end() && !state.at(RBrace) {
            self.skip_trivia(state);
            if state.at(RBrace) {
                break;
            }
            self.parse_statement(state)?;
        }
        self.skip_trivia(state);
        state.expect(RBrace).ok();
        state.finish_at(cp, crate::parser::element_type::KokaElementType::Block);
        Ok(())
    }
}

impl<'config> Pratt<KokaLanguage> for KokaParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, KokaLanguage> {
        use crate::lexer::token_type::KokaTokenType::*;
        self.skip_trivia(state);
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(Identifier) => {
                state.bump();
                state.finish_at(cp, crate::parser::element_type::KokaElementType::Identifier)
            }
            Some(StringLiteral) | Some(CharLiteral) | Some(NumberLiteral) | Some(BooleanLiteral) => {
                state.bump();
                state.finish_at(cp, crate::parser::element_type::KokaElementType::IntLiteral) // Simplified
            }
            Some(LParen) => {
                state.bump();
                self.skip_trivia(state);
                PrattParser::parse(state, 0, self);
                self.skip_trivia(state);
                state.expect(RParen).ok();
                state.finish_at(cp, crate::parser::element_type::KokaElementType::SourceFile) // Placeholder
            }
            _ => {
                state.bump();
                state.finish_at(cp, crate::parser::element_type::KokaElementType::Error)
            }
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, KokaLanguage> {
        use crate::lexer::token_type::KokaTokenType::*;
        self.skip_trivia(state);
        let kind = match state.peek_kind() {
            Some(k) => k,
            None => return self.primary(state),
        };

        match kind {
            Plus | Minus | Exclamation | Tilde => unary(state, kind, 12, crate::parser::element_type::KokaElementType::PrefixExpression.into(), |s, p| {
                self.skip_trivia(s);
                PrattParser::parse(s, p, self)
            }),
            _ => self.primary(state),
        }
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, KokaLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, KokaLanguage>> {
        use crate::lexer::token_type::KokaTokenType::*;
        self.skip_trivia(state);
        let kind = state.peek_kind()?;

        let (prec, assoc) = match kind {
            Assign | PlusAssign | MinusAssign | StarAssign | SlashAssign | PercentAssign => (1, Associativity::Right),
            OrOr => (2, Associativity::Left),
            AndAnd => (3, Associativity::Left),
            Lt | Gt | LtEq | GtEq | EqEq | NotEq | Is | As => (5, Associativity::Left),
            Plus | Minus | Pipe | Caret => (6, Associativity::Left),
            Star | Slash | Percent | Ampersand => (7, Associativity::Left),
            Dot | DoubleColon | Range => (8, Associativity::Left),
            LParen | LBracket => (9, Associativity::Left),
            _ => (0, Associativity::Left),
        };

        if prec < min_precedence {
            return None;
        }

        match kind {
            Dot => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.bump();
                self.skip_trivia(state);
                state.expect(Identifier).ok();
                Some(state.finish_at(cp, crate::parser::element_type::KokaElementType::MemberAccessExpression))
            }
            LParen => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.bump();
                while state.not_at_end() && !state.at(RParen) {
                    self.skip_trivia(state);
                    PrattParser::parse(state, 0, self);
                    self.skip_trivia(state);
                    if !state.eat(Comma) {
                        break;
                    }
                }
                self.skip_trivia(state);
                state.expect(RParen).ok();
                Some(state.finish_at(cp, crate::parser::element_type::KokaElementType::CallExpression))
            }
            LBracket => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.bump();
                self.skip_trivia(state);
                PrattParser::parse(state, 0, self);
                self.skip_trivia(state);
                state.expect(RBracket).ok();
                Some(state.finish_at(cp, crate::parser::element_type::KokaElementType::MemberAccessExpression))
            }
            Assign | PlusAssign | MinusAssign | StarAssign | SlashAssign | PercentAssign => Some(binary(state, left, kind, prec, assoc, crate::parser::element_type::KokaElementType::AssignmentExpression.into(), |s, p| {
                self.skip_trivia(s);
                PrattParser::parse(s, p, self)
            })),
            _ => Some(binary(state, left, kind, prec, assoc, crate::parser::element_type::KokaElementType::BinaryExpression.into(), |s, p| {
                self.skip_trivia(s);
                PrattParser::parse(s, p, self)
            })),
        }
    }
}

impl<'config> Parser<KokaLanguage> for KokaParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<KokaLanguage>) -> ParseOutput<'a, KokaLanguage> {
        let lexer = KokaLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                self.skip_trivia(state);
                if state.not_at_end() {
                    self.parse_statement(state).ok();
                }
            }

            Ok(state.finish_at(checkpoint, crate::parser::element_type::KokaElementType::SourceFile))
        })
    }
}
