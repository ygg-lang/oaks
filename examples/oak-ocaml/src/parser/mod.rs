/// Element types for the OCaml language.
pub mod element_type;

use crate::{
    language::OCamlLanguage,
    lexer::{OCamlLexer, token_type::OCamlTokenType},
    parser::element_type::OCamlElementType,
};
use oak_core::{
    GreenNode, OakError,
    parser::{Associativity, ParseCache, ParseOutput, Parser, ParserState, Pratt, PrattParser, binary, parse_with_lexer, unary},
    source::{Source, TextEdit},
};

/// OCaml parser state.
pub(crate) type State<'a, S> = ParserState<'a, OCamlLanguage, S>;

/// A parser for the OCaml language.
pub struct OCamlParser<'config> {
    /// The language configuration.
    pub language: &'config OCamlLanguage,
}

impl<'config> OCamlParser<'config> {
    /// Creates a new OCaml parser.
    pub fn new(language: &'config OCamlLanguage) -> Self {
        Self { language }
    }

    /// Parses a top-level item.
    fn parse_item<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> Result<(), OakError> {
        let kind = state.peek_kind();
        match kind {
            Some(OCamlTokenType::Let) => self.parse_let_binding(state),
            Some(OCamlTokenType::Module) => self.parse_module_def(state),
            Some(OCamlTokenType::Type) => self.parse_type_definition(state),
            Some(OCamlTokenType::Open) => {
                let cp = state.checkpoint();
                state.bump();
                while state.not_at_end() && !state.at(OCamlTokenType::Semicolon) {
                    state.bump();
                }
                state.finish_at(cp, OCamlElementType::Expression);
                Ok(())
            }
            _ => self.parse_expression_item(state),
        }
    }

    fn parse_expression_item<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        PrattParser::parse(state, 0, self);
        if state.at(OCamlTokenType::Semicolon) {
            state.bump();
            if state.at(OCamlTokenType::Semicolon) {
                state.bump();
            }
        }
        state.finish_at(cp, OCamlElementType::Expression);
        Ok(())
    }

    /// Parses a let binding.
    fn parse_let_binding<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.bump(); // Let
        if state.at(OCamlTokenType::Rec) {
            state.bump(); // Rec
        }
        // Simplified: consume identifier and until = or ;;
        while state.not_at_end() && !state.at(OCamlTokenType::Equal) && !state.at(OCamlTokenType::Semicolon) {
            state.bump();
        }
        if state.at(OCamlTokenType::Equal) {
            state.bump(); // Equal
            PrattParser::parse(state, 0, self);
        }
        state.finish_at(checkpoint, OCamlElementType::LetBinding);
        Ok(())
    }

    /// Parses a module definition.
    fn parse_module_def<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.bump(); // Module
        while state.not_at_end() && !state.at(OCamlTokenType::Equal) && !state.at(OCamlTokenType::Semicolon) {
            state.bump();
        }
        if state.at(OCamlTokenType::Equal) {
            state.bump(); // Equal
            while state.not_at_end() && !state.at(OCamlTokenType::Semicolon) {
                state.bump();
            }
        }
        state.finish_at(checkpoint, OCamlElementType::ModuleDef);
        Ok(())
    }

    /// Parses a type definition.
    fn parse_type_definition<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.bump(); // Type
        while state.not_at_end() && !state.at(OCamlTokenType::Semicolon) {
            state.bump();
        }
        state.finish_at(checkpoint, OCamlElementType::TypeDefinition);
        Ok(())
    }
}

impl<'config> Parser<OCamlLanguage> for OCamlParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<OCamlLanguage>) -> ParseOutput<'a, OCamlLanguage> {
        let lexer = OCamlLexer::new(self.language);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                self.parse_item(state)?;
            }

            Ok(state.finish_at(checkpoint, OCamlElementType::Root))
        })
    }
}

impl<'config> Pratt<OCamlLanguage> for OCamlParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, OCamlLanguage> {
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(OCamlTokenType::Identifier) => {
                state.bump();
                state.finish_at(cp, OCamlElementType::IdentifierExpr)
            }
            Some(OCamlTokenType::IntegerLiteral) | Some(OCamlTokenType::FloatLiteral) | Some(OCamlTokenType::StringLiteral) | Some(OCamlTokenType::CharLiteral) | Some(OCamlTokenType::True) | Some(OCamlTokenType::False) => {
                state.bump();
                state.finish_at(cp, OCamlElementType::LiteralExpr)
            }
            Some(OCamlTokenType::LeftParen) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(OCamlTokenType::RightParen).ok();
                state.finish_at(cp, OCamlElementType::Expression)
            }
            _ => {
                state.bump();
                state.finish_at(cp, OCamlElementType::Error)
            }
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, OCamlLanguage> {
        let kind = match state.peek_kind() {
            Some(k) => k,
            None => return self.primary(state),
        };

        match kind {
            OCamlTokenType::Minus | OCamlTokenType::MinusDot | OCamlTokenType::Tilde => unary(state, kind, 12, OCamlElementType::UnaryExpr.into(), |s, p| PrattParser::parse(s, p, self)),
            _ => self.primary(state),
        }
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, OCamlLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, OCamlLanguage>> {
        let kind = state.peek_kind()?;

        let (prec, assoc) = match kind {
            OCamlTokenType::Dot => (13, Associativity::Left),
            OCamlTokenType::Star | OCamlTokenType::Slash | OCamlTokenType::Percent => (11, Associativity::Left),
            OCamlTokenType::Plus | OCamlTokenType::Minus => (10, Associativity::Left),
            OCamlTokenType::At => (9, Associativity::Right),
            OCamlTokenType::Equal | OCamlTokenType::NotEqual | OCamlTokenType::Less | OCamlTokenType::Greater | OCamlTokenType::LessEqual | OCamlTokenType::GreaterEqual => (8, Associativity::Left),
            OCamlTokenType::And => (7, Associativity::Right),
            OCamlTokenType::Or => (6, Associativity::Right),
            OCamlTokenType::Semicolon => (5, Associativity::Right),
            _ => return None,
        };

        if prec < min_precedence {
            return None;
        }

        Some(binary(state, left, kind, prec, assoc, OCamlElementType::BinaryExpr.into(), |s, p| PrattParser::parse(s, p, self)))
    }
}
