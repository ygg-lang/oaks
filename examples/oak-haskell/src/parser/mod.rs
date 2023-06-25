pub mod element_type;

use crate::{
    language::HaskellLanguage,
    lexer::{HaskellLexer, token_type::HaskellTokenType},
    parser::element_type::HaskellElementType,
};
use oak_core::{
    GreenNode, OakError, TokenType,
    parser::{Associativity, ParseCache, ParseOutput, Parser, ParserState, Pratt, PrattParser, binary, parse_with_lexer, unary},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, HaskellLanguage, S>;

/// A parser for the Haskell language.
pub struct HaskellParser<'config> {
    /// Language configuration.
    pub(crate) config: &'config HaskellLanguage,
}

impl<'config> HaskellParser<'config> {
    /// Creates a new Haskell parser with the given configuration.
    pub fn new(config: &'config HaskellLanguage) -> Self {
        Self { config }
    }

    /// Parses a top-level item in a Haskell source file.
    fn parse_item<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        match state.peek_kind() {
            Some(HaskellTokenType::Module) => {
                self.parse_module(state);
            }
            Some(HaskellTokenType::Import) => {
                self.parse_import(state);
            }
            Some(HaskellTokenType::Data) => {
                self.parse_data(state);
            }
            Some(HaskellTokenType::Type) => {
                self.parse_type_alias(state);
            }
            Some(HaskellTokenType::Identifier) => {
                self.parse_function(state);
            }
            _ => {
                state.bump();
            }
        }
        Ok(())
    }

    fn parse_module<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, HaskellLanguage> {
        let cp = state.checkpoint();
        state.expect(HaskellTokenType::Module).ok();
        state.expect(HaskellTokenType::Constructor).ok(); // Module name
        if state.eat(HaskellTokenType::Where) {
            // Module body handled by parse loop
        }
        state.finish_at(cp, HaskellElementType::ModuleDeclaration)
    }

    fn parse_import<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, HaskellLanguage> {
        let cp = state.checkpoint();
        state.expect(HaskellTokenType::Import).ok();
        state.eat(HaskellTokenType::Qualified);
        state.expect(HaskellTokenType::Constructor).ok(); // Module name
        if state.eat(HaskellTokenType::As) {
            state.expect(HaskellTokenType::Constructor).ok();
        }
        state.finish_at(cp, HaskellElementType::ImportDeclaration)
    }

    fn parse_data<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, HaskellLanguage> {
        let cp = state.checkpoint();
        state.expect(HaskellTokenType::Data).ok();
        state.expect(HaskellTokenType::Constructor).ok(); // Type name
        while state.at(HaskellTokenType::Identifier) {
            state.bump(); // Type params
        }
        if state.eat(HaskellTokenType::Assign) {
            loop {
                state.expect(HaskellTokenType::Constructor).ok();
                while state.at(HaskellTokenType::Constructor) || state.at(HaskellTokenType::Identifier) || state.at(HaskellTokenType::LeftParen) {
                    if state.at(HaskellTokenType::LeftParen) {
                        state.bump();
                        while !state.at(HaskellTokenType::RightParen) && state.not_at_end() {
                            state.bump();
                        }
                        state.eat(HaskellTokenType::RightParen);
                    }
                    else {
                        state.bump();
                    }
                }
                if !state.eat(HaskellTokenType::Pipe) {
                    break;
                }
            }
        }
        state.finish_at(cp, HaskellElementType::DataDeclaration)
    }

    fn parse_type_alias<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, HaskellLanguage> {
        let cp = state.checkpoint();
        state.expect(HaskellTokenType::Type).ok();
        state.expect(HaskellTokenType::Constructor).ok();
        while state.at(HaskellTokenType::Identifier) {
            state.bump();
        }
        state.expect(HaskellTokenType::Assign).ok();
        // Simplified type parsing
        while !state.at(HaskellTokenType::Newline) && state.not_at_end() {
            state.bump();
        }
        state.finish_at(cp, HaskellElementType::TypeAliasDeclaration)
    }

    fn parse_function<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, HaskellLanguage> {
        let cp = state.checkpoint();
        state.expect(HaskellTokenType::Identifier).ok();
        if state.eat(HaskellTokenType::DoubleColon) {
            // Type signature
            while !state.at(HaskellTokenType::Newline) && state.not_at_end() {
                state.bump();
            }
            state.finish_at(cp, HaskellElementType::TypeSignature)
        }
        else {
            // Equation
            while !state.at(HaskellTokenType::Assign) && state.not_at_end() {
                state.bump(); // Patterns
            }
            state.expect(HaskellTokenType::Assign).ok();
            PrattParser::parse(state, 0, self); // Body
            state.finish_at(cp, HaskellElementType::Function)
        }
    }
}

impl<'config> Parser<HaskellLanguage> for HaskellParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<HaskellLanguage>) -> ParseOutput<'a, HaskellLanguage> {
        let lexer = HaskellLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                self.parse_item(state)?
            }

            Ok(state.finish_at(checkpoint, HaskellElementType::Root))
        })
    }
}

impl<'config> Pratt<HaskellLanguage> for HaskellParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, HaskellLanguage> {
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(HaskellTokenType::Identifier) => {
                state.bump();
                state.finish_at(cp, HaskellElementType::IdentifierExpression)
            }
            Some(HaskellTokenType::Constructor) => {
                state.bump();
                state.finish_at(cp, HaskellElementType::IdentifierExpression)
            }
            Some(HaskellTokenType::Integer) | Some(HaskellTokenType::Float) => {
                state.bump();
                state.finish_at(cp, HaskellElementType::LiteralExpression)
            }
            Some(HaskellTokenType::StringLiteral) | Some(HaskellTokenType::CharLiteral) => {
                state.bump();
                state.finish_at(cp, HaskellElementType::LiteralExpression)
            }
            Some(HaskellTokenType::LeftParen) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(HaskellTokenType::RightParen).ok();
                state.finish_at(cp, HaskellElementType::InfixExpression) // Simplified
            }
            _ => {
                state.bump();
                state.finish_at(cp, HaskellElementType::Error)
            }
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, HaskellLanguage> {
        let kind = match state.peek_kind() {
            Some(k) => k,
            None => return self.primary(state),
        };

        match kind {
            HaskellTokenType::Minus => unary(state, kind, 9, HaskellElementType::PrefixExpression.into(), |s, p| PrattParser::parse(s, p, self)),
            _ => self.primary(state),
        }
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, HaskellLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, HaskellLanguage>> {
        let kind = state.peek_kind()?;

        let (prec, assoc) = match kind {
            HaskellTokenType::Dot => (9, Associativity::Right),
            HaskellTokenType::Star | HaskellTokenType::Slash | HaskellTokenType::Percent => (7, Associativity::Left),
            HaskellTokenType::Plus | HaskellTokenType::Minus => (6, Associativity::Left),
            HaskellTokenType::Append => (5, Associativity::Right),
            HaskellTokenType::Equal | HaskellTokenType::NotEqual | HaskellTokenType::Less | HaskellTokenType::LessEqual | HaskellTokenType::Greater | HaskellTokenType::GreaterEqual => (4, Associativity::None),
            HaskellTokenType::And => (3, Associativity::Right),
            HaskellTokenType::Or => (2, Associativity::Right),
            HaskellTokenType::Dollar => (0, Associativity::Right),
            _ => return None,
        };

        if prec < min_precedence {
            return None;
        }

        Some(binary(state, left, kind, prec, assoc, HaskellElementType::InfixExpression.into(), |s, p| PrattParser::parse(s, p, self)))
    }
}
