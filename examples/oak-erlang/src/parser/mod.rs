pub mod element_type;

use crate::{
    language::ErlangLanguage,
    lexer::{ErlangLexer, token_type::ErlangTokenType},
    parser::element_type::ErlangElementType,
};
use oak_core::{
    GreenNode, OakError, TokenType,
    parser::{Associativity, ParseCache, ParseOutput, Parser, ParserState, Pratt, PrattParser, binary, parse_with_lexer, unary},
    source::{Source, TextEdit},
};

/// Internal parser state type alias.
pub(crate) type State<'a, S> = ParserState<'a, ErlangLanguage, S>;

/// Parser for the Erlang language.
pub struct ErlangParser<'config> {
    /// The Erlang language configuration.
    pub(crate) config: &'config ErlangLanguage,
}

impl<'config> ErlangParser<'config> {
    /// Creates a new `ErlangParser` with the given configuration.
    pub fn new(config: &'config ErlangLanguage) -> Self {
        Self { config }
    }

    fn parse_item<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        match state.peek_kind() {
            Some(ErlangTokenType::Minus) => self.parse_attribute(state),
            Some(ErlangTokenType::Atom) => self.parse_function(state),
            Some(_) => {
                state.bump();
                Ok(())
            }
            None => Ok(()),
        }
    }

    fn parse_attribute<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(ErlangTokenType::Minus).ok();
        match state.peek_kind() {
            Some(ErlangTokenType::Atom) => {
                let atom_text = state.peek_text().unwrap_or(std::borrow::Cow::Borrowed(""));
                match atom_text {
                    std::borrow::Cow::Borrowed("module") => {
                        state.bump();
                        state.expect(ErlangTokenType::LeftParen).ok();
                        state.expect(ErlangTokenType::Atom).ok();
                        state.expect(ErlangTokenType::RightParen).ok();
                        state.expect(ErlangTokenType::Dot).ok();
                        state.finish_at(cp, ErlangElementType::Module);
                    }
                    std::borrow::Cow::Borrowed("export") => {
                        state.bump();
                        state.expect(ErlangTokenType::LeftParen).ok();
                        state.expect(ErlangTokenType::LeftBracket).ok();
                        while state.not_at_end() && !state.at(ErlangTokenType::RightBracket) {
                            state.expect(ErlangTokenType::Atom).ok();
                            state.expect(ErlangTokenType::Slash).ok();
                            state.expect(ErlangTokenType::Number).ok();
                            state.eat(ErlangTokenType::Comma);
                        }
                        state.expect(ErlangTokenType::RightBracket).ok();
                        state.expect(ErlangTokenType::RightParen).ok();
                        state.expect(ErlangTokenType::Dot).ok();
                        state.finish_at(cp, ErlangElementType::Export);
                    }
                    _ => {
                        state.bump();
                        if state.eat(ErlangTokenType::LeftParen) {
                            while state.not_at_end() && !state.at(ErlangTokenType::RightParen) {
                                state.bump();
                            }
                            state.expect(ErlangTokenType::RightParen).ok();
                        }
                        state.expect(ErlangTokenType::Dot).ok();
                        state.finish_at(cp, ErlangElementType::Attribute);
                    }
                }
            }
            _ => {
                state.finish_at(cp, ErlangElementType::Error);
            }
        }
        Ok(())
    }

    fn parse_function<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        loop {
            let clause_cp = state.checkpoint();
            state.expect(ErlangTokenType::Atom).ok();
            state.expect(ErlangTokenType::LeftParen).ok();
            while state.not_at_end() && !state.at(ErlangTokenType::RightParen) {
                self.parse_pattern(state)?;
                state.eat(ErlangTokenType::Comma);
            }
            state.expect(ErlangTokenType::RightParen).ok();
            if state.eat(ErlangTokenType::When) {
                while state.not_at_end() && !state.at(ErlangTokenType::Arrow) {
                    state.bump();
                }
            }
            state.expect(ErlangTokenType::Arrow).ok();
            while state.not_at_end() && !state.at(ErlangTokenType::Semicolon) && !state.at(ErlangTokenType::Dot) {
                PrattParser::parse(state, 0, self);
                state.eat(ErlangTokenType::Comma);
            }
            state.finish_at(clause_cp, ErlangElementType::FunctionClause);

            if state.eat(ErlangTokenType::Semicolon) {
                continue;
            }
            else if state.eat(ErlangTokenType::Dot) {
                break;
            }
            else {
                break;
            }
        }
        state.finish_at(cp, ErlangElementType::Function);
        Ok(())
    }

    fn parse_pattern<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(ErlangTokenType::Atom) | Some(ErlangTokenType::Variable) | Some(ErlangTokenType::Number) | Some(ErlangTokenType::String) => {
                state.bump();
            }
            Some(ErlangTokenType::LeftBrace) => {
                state.bump();
                while state.not_at_end() && !state.at(ErlangTokenType::RightBrace) {
                    self.parse_pattern(state)?;
                    state.eat(ErlangTokenType::Comma);
                }
                state.expect(ErlangTokenType::RightBrace).ok();
            }
            Some(ErlangTokenType::LeftBracket) => {
                state.bump();
                while state.not_at_end() && !state.at(ErlangTokenType::RightBracket) {
                    self.parse_pattern(state)?;
                    if state.eat(ErlangTokenType::Pipe) {
                        self.parse_pattern(state)?;
                        break;
                    }
                    state.eat(ErlangTokenType::Comma);
                }
                state.expect(ErlangTokenType::RightBracket).ok();
            }
            _ => {
                state.bump();
            }
        }
        state.finish_at(cp, ErlangElementType::Pattern);
        Ok(())
    }
}

impl<'config> Parser<ErlangLanguage> for ErlangParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<ErlangLanguage>) -> ParseOutput<'a, ErlangLanguage> {
        let lexer = ErlangLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            while state.not_at_end() {
                self.parse_item(state)?
            }

            Ok(state.finish_at(checkpoint, ErlangElementType::Root))
        })
    }
}

impl<'config> Pratt<ErlangLanguage> for ErlangParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, ErlangLanguage> {
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(ErlangTokenType::Atom) | Some(ErlangTokenType::Variable) | Some(ErlangTokenType::Number) | Some(ErlangTokenType::String) => {
                state.bump();
                state.finish_at(cp, ErlangElementType::Expr)
            }
            Some(ErlangTokenType::LeftBrace) => {
                state.bump();
                while state.not_at_end() && !state.at(ErlangTokenType::RightBrace) {
                    PrattParser::parse(state, 0, self);
                    state.eat(ErlangTokenType::Comma);
                }
                state.expect(ErlangTokenType::RightBrace).ok();
                state.finish_at(cp, ErlangElementType::Expr)
            }
            Some(ErlangTokenType::LeftBracket) => {
                state.bump();
                while state.not_at_end() && !state.at(ErlangTokenType::RightBracket) {
                    PrattParser::parse(state, 0, self);
                    if state.eat(ErlangTokenType::Pipe) {
                        PrattParser::parse(state, 0, self);
                        break;
                    }
                    state.eat(ErlangTokenType::Comma);
                }
                state.expect(ErlangTokenType::RightBracket).ok();
                state.finish_at(cp, ErlangElementType::Expr)
            }
            _ => {
                state.bump();
                state.finish_at(cp, ErlangElementType::Error)
            }
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, ErlangLanguage> {
        let kind = match state.peek_kind() {
            Some(k) => k,
            None => return self.primary(state),
        };

        match kind {
            ErlangTokenType::Minus | ErlangTokenType::Plus | ErlangTokenType::Bnot | ErlangTokenType::Not => unary(state, kind, 12, ErlangElementType::Expr.into(), |s, p| PrattParser::parse(s, p, self)),
            _ => self.primary(state),
        }
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, ErlangLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, ErlangLanguage>> {
        let kind = state.peek_kind()?;

        let (prec, assoc) = match kind {
            ErlangTokenType::Colon => (13, Associativity::Left),
            ErlangTokenType::Slash | ErlangTokenType::Star | ErlangTokenType::Div | ErlangTokenType::Rem | ErlangTokenType::Band | ErlangTokenType::And => (11, Associativity::Left),
            ErlangTokenType::Plus | ErlangTokenType::Minus | ErlangTokenType::Bor | ErlangTokenType::Bxor | ErlangTokenType::Bsl | ErlangTokenType::Bsr | ErlangTokenType::Or | ErlangTokenType::Xor => (10, Associativity::Left),
            ErlangTokenType::PlusPlus | ErlangTokenType::MinusMinus => (9, Associativity::Right),
            ErlangTokenType::EqualEqual | ErlangTokenType::SlashEqual | ErlangTokenType::LessEqual | ErlangTokenType::Less | ErlangTokenType::GreaterEqual | ErlangTokenType::Greater | ErlangTokenType::EqualColonEqual | ErlangTokenType::EqualSlashEqual => {
                (8, Associativity::Left)
            }
            ErlangTokenType::Andalso => (7, Associativity::Left),
            ErlangTokenType::Orelse => (6, Associativity::Left),
            ErlangTokenType::Exclamation => (5, Associativity::Right),
            ErlangTokenType::Equal => (4, Associativity::Right),
            _ => return None,
        };

        if prec < min_precedence {
            return None;
        }

        Some(binary(state, left, kind, prec, assoc, ErlangElementType::BinaryExpr.into(), |s, p| PrattParser::parse(s, p, self)))
    }
}
