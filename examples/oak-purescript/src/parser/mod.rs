pub mod element_type;

use crate::{
    language::PurescriptLanguage,
    lexer::{PurescriptLexer, token_type::PurescriptTokenType},
    parser::element_type::PurescriptElementType,
};
use oak_core::{
    GreenNode, OakError,
    parser::{Associativity, ParseCache, ParseOutput, Parser, ParserState, Pratt, PrattParser, binary, parse_with_lexer, unary},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, PurescriptLanguage, S>;

/// PureScript parser implementation.
pub struct PurescriptParser<'config> {
    pub(crate) config: &'config PurescriptLanguage,
}

impl<'config> PurescriptParser<'config> {
    /// Creates a new `PurescriptParser`.
    pub fn new(config: &'config PurescriptLanguage) -> Self {
        Self { config }
    }

    fn parse_item<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        match state.peek_kind() {
            Some(PurescriptTokenType::Module) => self.parse_module(state),
            Some(PurescriptTokenType::Import) => self.parse_import(state),
            Some(PurescriptTokenType::Data) => self.parse_data(state),
            Some(PurescriptTokenType::Newtype) => self.parse_newtype(state),
            Some(PurescriptTokenType::Type) => self.parse_type_alias(state),
            Some(PurescriptTokenType::Class) => self.parse_class(state),
            Some(PurescriptTokenType::Instance) => self.parse_instance(state),
            Some(PurescriptTokenType::Foreign) => self.parse_foreign(state),
            Some(PurescriptTokenType::Identifier) | Some(PurescriptTokenType::Operator) => self.parse_value_declaration(state),
            Some(_) => {
                state.bump();
                Ok(())
            }
            None => Ok(()),
        }
    }

    fn parse_module<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(PurescriptTokenType::Module).ok();
        state.expect(PurescriptTokenType::UpperIdentifier).ok();
        while state.eat(PurescriptTokenType::Dot) {
            state.expect(PurescriptTokenType::UpperIdentifier).ok();
        }
        if state.eat(PurescriptTokenType::LeftParen) {
            while state.not_at_end() && !state.at(PurescriptTokenType::RightParen) {
                state.bump();
            }
            state.expect(PurescriptTokenType::RightParen).ok();
        }
        state.expect(PurescriptTokenType::Where).ok();
        state.finish_at(cp, PurescriptElementType::ModuleDeclaration);
        Ok(())
    }

    fn parse_import<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(PurescriptTokenType::Import).ok();
        state.expect(PurescriptTokenType::UpperIdentifier).ok();
        while state.eat(PurescriptTokenType::Dot) {
            state.expect(PurescriptTokenType::UpperIdentifier).ok();
        }
        if state.eat(PurescriptTokenType::LeftParen) {
            while state.not_at_end() && !state.at(PurescriptTokenType::RightParen) {
                state.bump();
            }
            state.expect(PurescriptTokenType::RightParen).ok();
        }
        state.finish_at(cp, PurescriptElementType::ImportDeclaration);
        Ok(())
    }

    fn parse_data<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(PurescriptTokenType::Data).ok();
        state.expect(PurescriptTokenType::UpperIdentifier).ok();
        while state.not_at_end() && !state.at(PurescriptTokenType::Equal) && !state.at(PurescriptTokenType::Where) {
            state.bump();
        }
        if state.eat(PurescriptTokenType::Equal) {
            while state.not_at_end() && !state.at(PurescriptTokenType::Pipe) && !state.at(PurescriptTokenType::Newline) {
                state.bump();
            }
        }
        state.finish_at(cp, PurescriptElementType::DataDeclaration);
        Ok(())
    }

    fn parse_newtype<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(PurescriptTokenType::Newtype).ok();
        state.expect(PurescriptTokenType::UpperIdentifier).ok();
        while state.not_at_end() && !state.at(PurescriptTokenType::Equal) {
            state.bump();
        }
        if state.eat(PurescriptTokenType::Equal) {
            state.expect(PurescriptTokenType::UpperIdentifier).ok();
            self.parse_type(state)?;
        }
        state.finish_at(cp, PurescriptElementType::NewtypeDeclaration);
        Ok(())
    }

    fn parse_type_alias<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(PurescriptTokenType::Type).ok();
        state.expect(PurescriptTokenType::UpperIdentifier).ok();
        while state.not_at_end() && !state.at(PurescriptTokenType::Equal) {
            state.bump();
        }
        if state.eat(PurescriptTokenType::Equal) {
            self.parse_type(state)?;
        }
        state.finish_at(cp, PurescriptElementType::TypeAliasDeclaration);
        Ok(())
    }

    fn parse_class<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(PurescriptTokenType::Class).ok();
        while state.not_at_end() && !state.at(PurescriptTokenType::Where) {
            state.bump();
        }
        if state.eat(PurescriptTokenType::Where) {
            while state.not_at_end() && !state.at(PurescriptTokenType::Newline) {
                state.bump();
            }
        }
        state.finish_at(cp, PurescriptElementType::ClassDeclaration);
        Ok(())
    }

    fn parse_instance<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(PurescriptTokenType::Instance).ok();
        while state.not_at_end() && !state.at(PurescriptTokenType::Where) {
            state.bump();
        }
        if state.eat(PurescriptTokenType::Where) {
            while state.not_at_end() && !state.at(PurescriptTokenType::Newline) {
                state.bump();
            }
        }
        state.finish_at(cp, PurescriptElementType::InstanceDeclaration);
        Ok(())
    }

    fn parse_foreign<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(PurescriptTokenType::Foreign).ok();
        state.expect(PurescriptTokenType::Import).ok();
        while state.not_at_end() && !state.at(PurescriptTokenType::Newline) {
            state.bump();
        }
        state.finish_at(cp, PurescriptElementType::ForeignImportDeclaration);
        Ok(())
    }

    fn parse_value_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        if state.at(PurescriptTokenType::Identifier) {
            let next = state.peek_at(1);
            if matches!(next, Some(t) if t.kind == PurescriptTokenType::ColonColon) {
                state.bump(); // ident
                state.bump(); // ::
                self.parse_type(state)?;
                state.finish_at(cp, PurescriptElementType::TypeSignature);
                return Ok(());
            }
        }

        while state.not_at_end() && !state.at(PurescriptTokenType::Equal) && !state.at(PurescriptTokenType::Pipe) {
            self.parse_pattern(state)?;
        }
        if state.eat(PurescriptTokenType::Equal) {
            PrattParser::parse(state, 0, self);
        }
        else if state.at(PurescriptTokenType::Pipe) {
            while state.eat(PurescriptTokenType::Pipe) {
                PrattParser::parse(state, 0, self);
                state.expect(PurescriptTokenType::Equal).ok();
                PrattParser::parse(state, 0, self);
            }
        }
        if state.eat(PurescriptTokenType::Where) {
            while state.not_at_end() && !state.at(PurescriptTokenType::Newline) {
                state.bump();
            }
        }
        state.finish_at(cp, PurescriptElementType::ValueDeclaration);
        Ok(())
    }

    fn parse_pattern<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(PurescriptTokenType::Identifier)
            | Some(PurescriptTokenType::UpperIdentifier)
            | Some(PurescriptTokenType::IntLiteral)
            | Some(PurescriptTokenType::StringLiteral)
            | Some(PurescriptTokenType::CharLiteral)
            | Some(PurescriptTokenType::True)
            | Some(PurescriptTokenType::False)
            | Some(PurescriptTokenType::Underscore) => {
                state.bump();
            }
            Some(PurescriptTokenType::LeftParen) => {
                state.bump();
                while state.not_at_end() && !state.at(PurescriptTokenType::RightParen) {
                    self.parse_pattern(state)?;
                }
                state.expect(PurescriptTokenType::RightParen).ok();
            }
            Some(PurescriptTokenType::LeftBracket) => {
                state.bump();
                while state.not_at_end() && !state.at(PurescriptTokenType::RightBracket) {
                    self.parse_pattern(state)?;
                    state.eat(PurescriptTokenType::Comma);
                }
                state.expect(PurescriptTokenType::RightBracket).ok();
            }
            _ => {
                state.bump();
            }
        }
        state.finish_at(cp, PurescriptElementType::Pattern);
        Ok(())
    }

    fn parse_type<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        while state.not_at_end() && !state.at(PurescriptTokenType::Newline) && !state.at(PurescriptTokenType::Equal) && !state.at(PurescriptTokenType::Pipe) && !state.at(PurescriptTokenType::Where) {
            state.bump();
        }
        state.finish_at(cp, PurescriptElementType::TypeNode);
        Ok(())
    }
}

impl<'config> Parser<PurescriptLanguage> for PurescriptParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<PurescriptLanguage>) -> ParseOutput<'a, PurescriptLanguage> {
        let lexer = PurescriptLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                self.parse_item(state)?;
            }

            Ok(state.finish_at(checkpoint, PurescriptElementType::SourceFile))
        })
    }
}

impl<'config> Pratt<PurescriptLanguage> for PurescriptParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, PurescriptLanguage> {
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(PurescriptTokenType::Identifier) => {
                state.bump();
                state.finish_at(cp, PurescriptElementType::IdentifierExpression)
            }
            Some(PurescriptTokenType::UpperIdentifier) => {
                state.bump();
                state.finish_at(cp, PurescriptElementType::IdentifierExpression)
            }
            Some(PurescriptTokenType::IntLiteral) | Some(PurescriptTokenType::NumberLiteral) | Some(PurescriptTokenType::StringLiteral) | Some(PurescriptTokenType::CharLiteral) | Some(PurescriptTokenType::True) | Some(PurescriptTokenType::False) => {
                state.bump();
                state.finish_at(cp, PurescriptElementType::LiteralExpression)
            }
            Some(PurescriptTokenType::LeftParen) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(PurescriptTokenType::RightParen).ok();
                state.finish_at(cp, PurescriptElementType::Expression)
            }
            Some(PurescriptTokenType::LeftBracket) => {
                state.bump();
                while state.not_at_end() && !state.at(PurescriptTokenType::RightBracket) {
                    PrattParser::parse(state, 0, self);
                    state.eat(PurescriptTokenType::Comma);
                }
                state.expect(PurescriptTokenType::RightBracket).ok();
                state.finish_at(cp, PurescriptElementType::LiteralExpression)
            }
            _ => {
                state.bump();
                state.finish_at(cp, PurescriptElementType::Error)
            }
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, PurescriptLanguage> {
        let kind = match state.peek_kind() {
            Some(k) => k,
            None => return self.primary(state),
        };

        match kind {
            PurescriptTokenType::Minus => unary(state, kind, 12, PurescriptElementType::PrefixExpression.into(), |s, p| PrattParser::parse(s, p, self)),
            _ => self.primary(state),
        }
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, PurescriptLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, PurescriptLanguage>> {
        let kind = state.peek_kind()?;

        let (prec, assoc) = match kind {
            PurescriptTokenType::Dot => (13, Associativity::Left),
            PurescriptTokenType::Star | PurescriptTokenType::Slash | PurescriptTokenType::Percent => (11, Associativity::Left),
            PurescriptTokenType::Plus | PurescriptTokenType::Minus => (10, Associativity::Left),
            PurescriptTokenType::Append => (9, Associativity::Right),
            PurescriptTokenType::EqualEqual | PurescriptTokenType::NotEqual | PurescriptTokenType::Less | PurescriptTokenType::Greater | PurescriptTokenType::LessEqual | PurescriptTokenType::GreaterEqual => (8, Associativity::Left),
            PurescriptTokenType::And => (7, Associativity::Right),
            PurescriptTokenType::Or => (6, Associativity::Right),
            PurescriptTokenType::Apply => (0, Associativity::Right),
            _ => return None,
        };

        if prec < min_precedence {
            return None;
        }

        Some(binary(state, left, kind, prec, assoc, PurescriptElementType::InfixExpression.into(), |s, p| PrattParser::parse(s, p, self)))
    }
}
