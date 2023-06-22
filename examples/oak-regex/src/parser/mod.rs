use crate::{RegexLanguage, RegexSyntaxKind, lexer::RegexLexer};
use oak_core::{
    GreenNode, Source, TextEdit,
    parser::{Associativity, ParseCache, ParseOutput, Parser, ParserState, Pratt, PrattParser, binary},
};

pub(crate) type State<'a, S> = ParserState<'a, RegexLanguage, S>;

#[allow(missing_docs)]
pub struct RegexParser<'config> {
    /// Language configuration
    pub(crate) config: &'config RegexLanguage,
}

impl<'config> RegexParser<'config> {
    pub fn new(config: &'config RegexLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Pratt<RegexLanguage> for RegexParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, RegexLanguage> {
        use RegexSyntaxKind::*;
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(Character) => {
                state.bump();
                state.finish_at(cp, RegexPattern.into())
            }
            Some(Dot) => {
                state.bump();
                state.finish_at(cp, RegexPattern.into())
            }
            Some(LParen) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(RParen).ok();
                state.finish_at(cp, RegexPattern.into())
            }
            Some(LBrack) => {
                state.bump();
                while state.not_at_end() && !state.at(RBrack) {
                    state.advance();
                }
                state.expect(RBrack).ok();
                state.finish_at(cp, RegexPattern.into())
            }
            _ => {
                state.bump();
                state.finish_at(cp, Error.into())
            }
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, RegexLanguage> {
        use RegexSyntaxKind::*;
        match state.peek_kind() {
            Some(Hat) | Some(Dollar) => {
                let cp = state.checkpoint();
                state.bump();
                state.finish_at(cp, RegexPattern.into())
            }
            Some(Backslash) => {
                let cp = state.checkpoint();
                state.bump();
                state.advance();
                state.finish_at(cp, RegexPattern.into())
            }
            _ => self.primary(state),
        }
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, RegexLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, RegexLanguage>> {
        use RegexSyntaxKind::*;
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
                Some(state.finish_at(cp, RegexPattern.into()))
            }
            _ => None,
        }
    }
}

impl<'config> Parser<RegexLanguage> for RegexParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<RegexLanguage>) -> ParseOutput<'a, RegexLanguage> {
        let lexer = RegexLexer::new(&self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            while state.not_at_end() {
                PrattParser::parse(state, 0, self);
            }
            Ok(state.finish_at(checkpoint, RegexSyntaxKind::RegexPattern.into()))
        })
    }
}
