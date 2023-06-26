/// Element type definitions for Raku AST nodes.
/// This module provides the [`RakuElementType`] enum which identifies different
/// node kinds in the parsed Abstract Syntax Tree, such as statements, expressions,
/// declarations, and other syntactic constructs.
pub mod element_type;

use crate::{
    language::RakuLanguage,
    lexer::{RakuLexer, token_type::RakuTokenType},
};
pub use element_type::RakuElementType;
use oak_core::{
    GreenNode, OakError, TokenType,
    parser::{Associativity, ParseCache, ParseOutput, Parser, ParserState, Pratt, PrattParser, binary, parse_with_lexer, unary},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, RakuLanguage, S>;

/// Parser for Raku.
pub struct RakuParser {
    _language: RakuLanguage,
}

impl RakuParser {
    /// Creates a new `RakuParser`.
    pub fn new(language: RakuLanguage) -> Self {
        Self { _language: language }
    }

    fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(RakuTokenType::My) | Some(RakuTokenType::Our) | Some(RakuTokenType::Has) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                state.eat(RakuTokenType::Semicolon);
                state.finish_at(cp, RakuElementType::VariableDeclaration);
            }
            Some(RakuTokenType::Sub) | Some(RakuTokenType::Method) => {
                state.bump();
                state.expect(RakuTokenType::Identifier).ok();
                if state.at(RakuTokenType::LeftParen) {
                    self.parse_block(state)?;
                }
                else {
                    self.parse_block(state)?;
                }
                state.finish_at(cp, RakuElementType::FunctionDefinition);
            }
            Some(RakuTokenType::Class) | Some(RakuTokenType::Module) => {
                state.bump();
                state.expect(RakuTokenType::Identifier).ok();
                self.parse_block(state)?;
                state.finish_at(cp, RakuElementType::ClassDefinition);
            }
            Some(RakuTokenType::If) | Some(RakuTokenType::For) | Some(RakuTokenType::While) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                self.parse_block(state)?;
                state.finish_at(cp, RakuElementType::Statement);
            }
            Some(_) => {
                PrattParser::parse(state, 0, self);
                state.eat(RakuTokenType::Semicolon);
                state.finish_at(cp, RakuElementType::Statement);
            }
            None => {}
        }
        Ok(())
    }

    fn parse_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(RakuTokenType::LeftBrace).ok();
        while state.not_at_end() && !state.at(RakuTokenType::RightBrace) {
            self.parse_statement(state)?;
        }
        state.expect(RakuTokenType::RightBrace).ok();
        state.finish_at(cp, RakuElementType::Block);
        Ok(())
    }
}

impl Parser<RakuLanguage> for RakuParser {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<RakuLanguage>) -> ParseOutput<'a, RakuLanguage> {
        let lexer = RakuLexer::new();
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let cp = state.checkpoint();
            while state.not_at_end() {
                self.parse_statement(state)?;
            }
            Ok(state.finish_at(cp, RakuElementType::Root))
        })
    }
}

impl Pratt<RakuLanguage> for RakuParser {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, RakuLanguage> {
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(RakuTokenType::Identifier) => {
                state.bump();
                state.finish_at(cp, RakuElementType::IdentifierExpression)
            }
            Some(RakuTokenType::Number) | Some(RakuTokenType::String) => {
                state.bump();
                state.finish_at(cp, RakuElementType::LiteralExpression)
            }
            Some(RakuTokenType::LeftParen) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(RakuTokenType::RightParen).ok();
                state.finish_at(cp, RakuElementType::Expression)
            }
            Some(RakuTokenType::LeftBrace) => {
                self.parse_block(state).ok();
                state.finish_at(cp, RakuElementType::Expression)
            }
            _ => {
                state.bump();
                state.finish_at(cp, RakuElementType::Error)
            }
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, RakuLanguage> {
        self.primary(state)
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, RakuLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, RakuLanguage>> {
        let kind = state.peek_kind()?;
        let (prec, assoc) = match kind {
            RakuTokenType::Operator => (10, Associativity::Left),
            RakuTokenType::Dot => (15, Associativity::Left),
            RakuTokenType::LeftParen => (20, Associativity::Left),
            _ => return None,
        };

        if prec < min_precedence {
            return None;
        }

        match kind {
            RakuTokenType::LeftParen => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.bump();
                while state.not_at_end() && !state.at(RakuTokenType::RightParen) {
                    PrattParser::parse(state, 0, self);
                    state.eat(RakuTokenType::Comma);
                }
                state.expect(RakuTokenType::RightParen).ok();
                Some(state.finish_at(cp, RakuElementType::CallExpression))
            }
            _ => Some(binary(state, left, kind, prec, assoc, RakuElementType::BinaryExpression.into(), |s, p| PrattParser::parse(s, p, self))),
        }
    }
}
