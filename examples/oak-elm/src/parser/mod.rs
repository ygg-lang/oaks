/// Element type definitions for Elm.
pub mod element_type;

use crate::{
    language::ElmLanguage,
    lexer::{ElmLexer, token_type::ElmTokenType},
    parser::element_type::ElmElementType,
};
use oak_core::{
    GreenNode, OakError,
    parser::{Associativity, ParseCache, ParseOutput, Parser, ParserState, Pratt, PrattParser, binary, parse_with_lexer, unary},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, ElmLanguage, S>;

/// A parser for Elm source files.
pub struct ElmParser<'config> {
    pub(crate) config: &'config ElmLanguage,
}

impl<'config> ElmParser<'config> {
    /// Creates a new Elm parser with the given configuration.
    pub fn new(config: &'config ElmLanguage) -> Self {
        Self { config }
    }

    fn parse_item<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        match state.peek_kind() {
            Some(ElmTokenType::Module) => self.parse_module(state),
            Some(ElmTokenType::Import) => self.parse_import(state),
            Some(ElmTokenType::Type) => {
                if state.peek_at(1).map(|t| t.kind) == Some(ElmTokenType::Alias) {
                    self.parse_type_alias(state)
                }
                else {
                    self.parse_type_declaration(state)
                }
            }
            Some(ElmTokenType::Port) => self.parse_port(state),
            Some(ElmTokenType::Identifier) => self.parse_value_declaration(state),
            Some(_) => {
                state.bump();
                Ok(())
            }
            None => Ok(()),
        }
    }

    fn parse_module<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(ElmTokenType::Module).ok();
        state.expect(ElmTokenType::Identifier).ok();
        if state.eat(ElmTokenType::Exposing) {
            state.expect(ElmTokenType::LeftParen).ok();
            while state.not_at_end() && !state.at(ElmTokenType::RightParen) {
                state.bump();
            }
            state.expect(ElmTokenType::RightParen).ok();
        }
        state.expect(ElmTokenType::Where).ok();
        state.finish_at(cp, ElmElementType::Module);
        Ok(())
    }

    fn parse_import<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(ElmTokenType::Import).ok();
        state.expect(ElmTokenType::Identifier).ok();
        if state.eat(ElmTokenType::As) {
            state.expect(ElmTokenType::Identifier).ok();
        }
        if state.eat(ElmTokenType::Exposing) {
            state.expect(ElmTokenType::LeftParen).ok();
            while state.not_at_end() && !state.at(ElmTokenType::RightParen) {
                state.bump();
            }
            state.expect(ElmTokenType::RightParen).ok();
        }
        state.finish_at(cp, ElmElementType::Import);
        Ok(())
    }

    fn parse_type_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(ElmTokenType::Type).ok();
        state.expect(ElmTokenType::Identifier).ok();
        while state.not_at_end() && !state.at(ElmTokenType::Equal) {
            state.bump();
        }
        if state.eat(ElmTokenType::Equal) {
            while state.not_at_end() && !state.at(ElmTokenType::Newline) {
                state.bump();
            }
        }
        state.finish_at(cp, ElmElementType::TypeDeclaration);
        Ok(())
    }

    fn parse_type_alias<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(ElmTokenType::Type).ok();
        state.expect(ElmTokenType::Alias).ok();
        state.expect(ElmTokenType::Identifier).ok();
        while state.not_at_end() && !state.at(ElmTokenType::Equal) {
            state.bump();
        }
        if state.eat(ElmTokenType::Equal) {
            while state.not_at_end() && !state.at(ElmTokenType::Newline) {
                state.bump();
            }
        }
        state.finish_at(cp, ElmElementType::TypeAlias);
        Ok(())
    }

    fn parse_port<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(ElmTokenType::Port).ok();
        state.expect(ElmTokenType::Identifier).ok();
        state.expect(ElmTokenType::Colon).ok();
        while state.not_at_end() && !state.at(ElmTokenType::Newline) {
            state.bump();
        }
        state.finish_at(cp, ElmElementType::FunctionDeclaration);
        Ok(())
    }

    fn parse_value_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        if state.at(ElmTokenType::Identifier) {
            let next = state.peek_at(1);
            if matches!(next, Some(t) if t.kind == ElmTokenType::Colon) {
                state.bump(); // ident
                state.bump(); // :
                while state.not_at_end() && !state.at(ElmTokenType::Newline) {
                    state.bump();
                }
                state.finish_at(cp, ElmElementType::TypeSignature);
                return Ok(());
            }
        }

        while state.not_at_end() && !state.at(ElmTokenType::Equal) {
            self.parse_pattern(state)?;
        }
        if state.eat(ElmTokenType::Equal) {
            PrattParser::parse(state, 0, self);
        }
        state.finish_at(cp, ElmElementType::ValueDeclaration);
        Ok(())
    }

    fn parse_pattern<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(ElmTokenType::Identifier) | Some(ElmTokenType::Number) | Some(ElmTokenType::String) | Some(ElmTokenType::Char) => {
                state.bump();
            }
            Some(ElmTokenType::LeftParen) => {
                state.bump();
                while state.not_at_end() && !state.at(ElmTokenType::RightParen) {
                    self.parse_pattern(state)?;
                }
                state.expect(ElmTokenType::RightParen).ok();
            }
            Some(ElmTokenType::LeftBracket) => {
                state.bump();
                while state.not_at_end() && !state.at(ElmTokenType::RightBracket) {
                    self.parse_pattern(state)?;
                    state.eat(ElmTokenType::Comma);
                }
                state.expect(ElmTokenType::RightBracket).ok();
            }
            _ => {
                state.bump();
            }
        }
        state.finish_at(cp, ElmElementType::Pattern);
        Ok(())
    }
}

impl<'config> Parser<ElmLanguage> for ElmParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<ElmLanguage>) -> ParseOutput<'a, ElmLanguage> {
        let lexer = ElmLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                self.parse_item(state)?;
            }

            Ok(state.finish_at(checkpoint, ElmElementType::Root))
        })
    }
}

impl<'config> Pratt<ElmLanguage> for ElmParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, ElmLanguage> {
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(ElmTokenType::Identifier) => {
                state.bump();
                state.finish_at(cp, ElmElementType::Identifier)
            }
            Some(ElmTokenType::Number) | Some(ElmTokenType::Float) | Some(ElmTokenType::String) | Some(ElmTokenType::Char) => {
                state.bump();
                state.finish_at(cp, ElmElementType::Literal)
            }
            Some(ElmTokenType::LeftParen) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(ElmTokenType::RightParen).ok();
                state.finish_at(cp, ElmElementType::Expression)
            }
            Some(ElmTokenType::LeftBracket) => {
                state.bump();
                while state.not_at_end() && !state.at(ElmTokenType::RightBracket) {
                    PrattParser::parse(state, 0, self);
                    state.eat(ElmTokenType::Comma);
                }
                state.expect(ElmTokenType::RightBracket).ok();
                state.finish_at(cp, ElmElementType::ListExpression)
            }
            _ => {
                state.bump();
                state.finish_at(cp, ElmElementType::Error)
            }
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, ElmLanguage> {
        let kind = match state.peek_kind() {
            Some(k) => k,
            None => return self.primary(state),
        };

        match kind {
            ElmTokenType::Minus => unary(state, kind, 12, ElmElementType::UnaryExpression.into(), |s, p| PrattParser::parse(s, p, self)),
            _ => self.primary(state),
        }
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, ElmLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, ElmLanguage>> {
        let kind = state.peek_kind()?;

        let (prec, assoc) = match kind {
            ElmTokenType::Dot => (13, Associativity::Left),
            ElmTokenType::Star | ElmTokenType::Slash | ElmTokenType::DoubleSlash | ElmTokenType::Percent => (11, Associativity::Left),
            ElmTokenType::Plus | ElmTokenType::Minus => (10, Associativity::Left),
            ElmTokenType::DoublePlus => (9, Associativity::Right),
            ElmTokenType::EqualEqual | ElmTokenType::NotEqual | ElmTokenType::Less | ElmTokenType::Greater | ElmTokenType::LessEqual | ElmTokenType::GreaterEqual => (8, Associativity::Left),
            ElmTokenType::DoubleAmpersand => (7, Associativity::Right),
            ElmTokenType::DoublePipe => (6, Associativity::Right),
            ElmTokenType::DoubleLess | ElmTokenType::DoubleGreater => (5, Associativity::Left),
            _ => return None,
        };

        if prec < min_precedence {
            return None;
        }

        Some(binary(state, left, kind, prec, assoc, ElmElementType::BinaryExpression.into(), |s, p| PrattParser::parse(s, p, self)))
    }
}
