pub mod element_type;

use crate::{
    language::RegexLanguage,
    lexer::{RegexLexer, token_type::RegexTokenType},
    parser::element_type::RegexElementType,
};
use oak_core::{
    GreenNode, OakError, Source, TextEdit,
    parser::{Associativity, ParseCache, ParseOutput, Parser, ParserState, Pratt, PrattParser, binary, parse_with_lexer},
};

pub(crate) type State<'a, S> = ParserState<'a, RegexLanguage, S>;

/// A parser for regular expressions.
pub struct RegexParser<'config> {
    /// Language configuration
    pub(crate) config: &'config RegexLanguage,
}

impl<'config> RegexParser<'config> {
    /// Creates a new `RegexParser` with the given configuration.
    pub fn new(config: &'config RegexLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Pratt<RegexLanguage> for RegexParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, RegexLanguage> {
        use RegexTokenType::*;
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(Character) => {
                state.bump();
                state.finish_at(cp, crate::parser::element_type::RegexElementType::RegexPattern)
            }
            Some(Dot) => {
                state.bump();
                state.finish_at(cp, crate::parser::element_type::RegexElementType::RegexPattern)
            }
            Some(LParen) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(RParen).ok();
                state.finish_at(cp, crate::parser::element_type::RegexElementType::RegexPattern)
            }
            Some(LBrack) => {
                state.bump();
                while state.not_at_end() && !state.at(RBrack) {
                    state.advance();
                }
                state.expect(RBrack).ok();
                state.finish_at(cp, crate::parser::element_type::RegexElementType::RegexPattern)
            }
            _ => {
                state.bump();
                state.finish_at(cp, crate::parser::element_type::RegexElementType::Error)
            }
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, RegexLanguage> {
        use RegexTokenType::*;
        match state.peek_kind() {
            Some(Hat) | Some(Dollar) => {
                let cp = state.checkpoint();
                state.bump();
                state.finish_at(cp, crate::parser::element_type::RegexElementType::RegexPattern)
            }
            Some(Backslash) => {
                let cp = state.checkpoint();
                state.bump();
                state.advance();
                state.finish_at(cp, crate::parser::element_type::RegexElementType::RegexPattern)
            }
            _ => self.primary(state),
        }
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, RegexLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, RegexLanguage>> {
        use RegexTokenType::*;
        let kind = state.peek_kind()?;

        let (prec, assoc) = match kind {
            Pipe => (1, Associativity::Left),
            Question | Star | Plus => (10, Associativity::Right),
            _ => return None,
        };

        if prec < min_precedence {
            return None;
        }

        match kind {
            Pipe => Some(binary(state, left, Pipe, prec, assoc, RegexPattern.into(), |s, p| PrattParser::parse(s, p, self))),
            Question | Star | Plus => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.bump();
                Some(state.finish_at(cp, crate::parser::element_type::RegexElementType::RegexPattern))
            }
            _ => None,
        }
    }
}

impl<'config> Parser<RegexLanguage> for RegexParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<RegexLanguage>) -> ParseOutput<'a, RegexLanguage> {
        let lexer = RegexLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}

impl<'config> RegexParser<'config> {
    fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, RegexLanguage>, OakError> {
        let cp = state.checkpoint();
        PrattParser::parse(state, 0, self);
        Ok(state.finish_at(cp, crate::parser::element_type::RegexElementType::RegexPattern))
    }
}
