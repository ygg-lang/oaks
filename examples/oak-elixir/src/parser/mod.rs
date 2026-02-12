use crate::{
    language::ElixirLanguage,
    lexer::{ElixirLexer, token_type::ElixirTokenType},
};
use oak_core::{
    GreenNode, OakError, TokenType,
    parser::{Associativity, ParseCache, ParseOutput, Parser, ParserState, Pratt, PrattParser, binary, parse_with_lexer, unary},
    source::{Source, TextEdit},
};

/// Element type module for Elixir.
pub mod element_type;
pub use self::element_type::ElixirElementType;

pub(crate) type State<'a, S> = ParserState<'a, ElixirLanguage, S>;

/// Elixir parser implementation.
pub struct ElixirParser<'config> {
    pub(crate) config: &'config ElixirLanguage,
}

impl<'config> Parser<ElixirLanguage> for ElixirParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<ElixirLanguage>) -> ParseOutput<'a, ElixirLanguage> {
        let lexer = ElixirLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                match state.peek_kind() {
                    Some(ElixirTokenType::Defmodule) => {
                        self.parse_module(state);
                    }
                    Some(ElixirTokenType::Def) | Some(ElixirTokenType::Defp) => {
                        self.parse_function(state);
                    }
                    Some(_) => {
                        PrattParser::parse(state, 0, self);
                    }
                    None => break,
                }
                state.eat(ElixirTokenType::Semicolon);
                state.eat(ElixirTokenType::Newline);
            }

            Ok(state.finish_at(checkpoint, ElixirElementType::Root))
        })
    }
}

impl<'config> ElixirParser<'config> {
    /// Creates a new `ElixirParser`.
    pub fn new(config: &'config ElixirLanguage) -> Self {
        Self { config }
    }

    fn parse_module<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, ElixirLanguage> {
        let cp = state.checkpoint();
        state.expect(ElixirTokenType::Defmodule).ok();
        PrattParser::parse(state, 0, self); // Module name
        state.expect(ElixirTokenType::Do).ok();
        while state.not_at_end() && !state.at(ElixirTokenType::End) {
            match state.peek_kind() {
                Some(ElixirTokenType::Def) | Some(ElixirTokenType::Defp) => {
                    self.parse_function(state);
                }
                Some(_) => {
                    PrattParser::parse(state, 0, self);
                }
                None => break,
            }
            state.eat(ElixirTokenType::Semicolon);
            state.eat(ElixirTokenType::Newline);
        }
        state.expect(ElixirTokenType::End).ok();
        state.finish_at(cp, ElixirElementType::ModuleDefinition)
    }

    fn parse_function<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, ElixirLanguage> {
        let cp = state.checkpoint();
        state.bump(); // def or defp
        PrattParser::parse(state, 0, self); // Function name and params
        if state.eat(ElixirTokenType::Do) {
            while state.not_at_end() && !state.at(ElixirTokenType::End) {
                PrattParser::parse(state, 0, self);
                state.eat(ElixirTokenType::Semicolon);
                state.eat(ElixirTokenType::Newline);
            }
            state.expect(ElixirTokenType::End).ok();
        }
        state.finish_at(cp, ElixirElementType::FunctionDefinition)
    }
}

impl<'config> Pratt<ElixirLanguage> for ElixirParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, ElixirLanguage> {
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(ElixirTokenType::Identifier) => {
                state.bump();
                state.finish_at(cp, ElixirElementType::IdentifierExpression)
            }
            Some(ElixirTokenType::Atom) => {
                state.bump();
                state.finish_at(cp, ElixirElementType::LiteralExpression)
            }
            Some(ElixirTokenType::Number) | Some(ElixirTokenType::Float) => {
                state.bump();
                state.finish_at(cp, ElixirElementType::LiteralExpression)
            }
            Some(ElixirTokenType::String) | Some(ElixirTokenType::Character) => {
                state.bump();
                state.finish_at(cp, ElixirElementType::LiteralExpression)
            }
            Some(ElixirTokenType::LeftParen) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(ElixirTokenType::RightParen).ok();
                state.finish_at(cp, ElixirElementType::BlockExpression)
            }
            Some(ElixirTokenType::LeftBracket) => {
                state.bump();
                if !state.at(ElixirTokenType::RightBracket) {
                    loop {
                        PrattParser::parse(state, 0, self);
                        if !state.eat(ElixirTokenType::Comma) {
                            break;
                        }
                    }
                }
                state.expect(ElixirTokenType::RightBracket).ok();
                state.finish_at(cp, ElixirElementType::ListLiteral)
            }
            Some(ElixirTokenType::LeftBrace) => {
                state.bump();
                if !state.at(ElixirTokenType::RightBrace) {
                    loop {
                        PrattParser::parse(state, 0, self);
                        if !state.eat(ElixirTokenType::Comma) {
                            break;
                        }
                    }
                }
                state.expect(ElixirTokenType::RightBrace).ok();
                state.finish_at(cp, ElixirElementType::TupleLiteral)
            }
            Some(ElixirTokenType::Percent) => {
                state.bump();
                if state.eat(ElixirTokenType::LeftBrace) {
                    if !state.at(ElixirTokenType::RightBrace) {
                        loop {
                            PrattParser::parse(state, 0, self);
                            if !state.eat(ElixirTokenType::Comma) {
                                break;
                            }
                        }
                    }
                    state.expect(ElixirTokenType::RightBrace).ok();
                    state.finish_at(cp, ElixirElementType::MapLiteral)
                }
                else {
                    state.finish_at(cp, ElixirElementType::Error)
                }
            }
            _ => {
                state.bump();
                state.finish_at(cp, ElixirElementType::Error)
            }
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, ElixirLanguage> {
        let kind = match state.peek_kind() {
            Some(k) => k,
            None => return self.primary(state),
        };

        match kind {
            ElixirTokenType::Plus | ElixirTokenType::Minus | ElixirTokenType::Bang | ElixirTokenType::At => unary(state, kind, 13, ElixirElementType::UnaryExpression.into(), |s, p| PrattParser::parse(s, p, self)),
            _ => self.primary(state),
        }
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, ElixirLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, ElixirLanguage>> {
        use ElixirTokenType::*;
        let kind = state.peek_kind()?;

        let (prec, assoc) = match kind {
            Eq => (1, Associativity::Right),
            OrOr => (2, Associativity::Left),
            AndAnd => (3, Associativity::Left),
            EqEq | Ne => (4, Associativity::Left),
            Lt | Le | Gt | Ge => (5, Associativity::Left),
            Pipeline => (6, Associativity::Left),
            Concat | PlusPlus | MinusMinus => (7, Associativity::Left),
            Plus | Minus => (8, Associativity::Left),
            Mul | Div => (9, Associativity::Left),
            Dot => (11, Associativity::Left),
            LeftParen => (12, Associativity::Left),
            _ => return None,
        };

        if prec < min_precedence {
            return None;
        }

        match kind {
            LeftParen => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.bump();
                if !state.at(RightParen) {
                    loop {
                        PrattParser::parse(state, 0, self);
                        if !state.eat(Comma) {
                            break;
                        }
                    }
                }
                state.expect(RightParen).ok();
                Some(state.finish_at(cp, ElixirElementType::CallExpression))
            }
            Dot => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.bump();
                PrattParser::parse(state, prec + 1, self);
                Some(state.finish_at(cp, ElixirElementType::AccessExpression))
            }
            Eq => Some(binary(state, left, kind, prec, assoc, ElixirElementType::MatchExpression.into(), |s, p| PrattParser::parse(s, p, self))),
            _ => Some(binary(state, left, kind, prec, assoc, ElixirElementType::BinaryExpression.into(), |s, p| PrattParser::parse(s, p, self))),
        }
    }
}
