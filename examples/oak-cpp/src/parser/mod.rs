mod element_type;
pub use element_type::CppElementType;

use crate::{
    language::CppLanguage,
    lexer::{CppLexer, CppTokenType},
};
use oak_core::{
    GreenNode, OakError,
    parser::{Associativity, ParseCache, ParseOutput, Parser, ParserState, Pratt, PrattParser, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, CppLanguage, S>;

pub struct CppParser<'config> {
    pub(crate) config: &'config CppLanguage,
}

impl<'config> CppParser<'config> {
    pub fn new(config: &'config CppLanguage) -> Self {
        Self { config }
    }

    fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::CppTokenType::*;
        match state.peek_kind() {
            Some(Keyword) => {
                state.bump();
                while state.not_at_end() && !state.at(Semicolon) {
                    state.advance();
                }
                state.eat(Semicolon);
            }
            Some(LeftBrace) => self.parse_compound_statement(state)?,
            Some(Preprocessor) => {
                state.bump();
            }
            _ => {
                PrattParser::parse(state, 0, self);
                state.eat(Semicolon);
            }
        }
        Ok(())
    }

    fn parse_compound_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.expect(CppTokenType::LeftBrace).ok();
        while state.not_at_end() && !state.at(CppTokenType::RightBrace) {
            self.parse_statement(state)?;
        }
        state.expect(CppTokenType::RightBrace).ok();
        Ok(())
    }
}

impl<'config> Parser<CppLanguage> for CppParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<CppLanguage>) -> ParseOutput<'a, CppLanguage> {
        let lexer = CppLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let cp = state.checkpoint();
            while state.not_at_end() {
                self.parse_statement(state)?;
            }
            Ok(state.finish_at(cp, CppElementType::SourceFile))
        })
    }
}

impl<'config> Pratt<CppLanguage> for CppParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, CppLanguage> {
        use crate::lexer::CppTokenType::*;
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(Identifier) => {
                state.bump();
                state.finish_at(cp, CppElementType::SourceFile) // 简化处理
            }
            Some(IntegerLiteral) | Some(FloatLiteral) | Some(CharacterLiteral) | Some(StringLiteral) | Some(BooleanLiteral) => {
                state.bump();
                state.finish_at(cp, CppElementType::SourceFile) // 简化处理
            }
            Some(LeftParen) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(RightParen).ok();
                state.finish_at(cp, CppElementType::SourceFile)
            }
            _ => {
                state.bump();
                state.finish_at(cp, CppElementType::Error)
            }
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, CppLanguage> {
        self.primary(state)
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, CppLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, CppLanguage>> {
        use crate::lexer::CppTokenType::*;
        let kind = state.peek_kind()?;

        let (prec, assoc) = match kind {
            Assign | PlusAssign | MinusAssign | StarAssign | SlashAssign | PercentAssign | AndAssign | OrAssign | XorAssign | LeftShiftAssign | RightShiftAssign => (1, Associativity::Right),
            LogicalOr => (2, Associativity::Left),
            LogicalAnd => (3, Associativity::Left),
            Equal | NotEqual | Less | Greater | LessEqual | GreaterEqual => (4, Associativity::Left),
            Plus | Minus => (10, Associativity::Left),
            Star | Slash | Percent => (11, Associativity::Left),
            LeftParen | LeftBracket | Dot | Arrow => (15, Associativity::Left),
            Scope => (16, Associativity::Left),
            _ => return None,
        };

        if prec < min_precedence {
            return None;
        }

        match kind {
            LeftParen => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(LeftParen).ok();
                while state.not_at_end() && !state.at(RightParen) {
                    state.bump();
                }
                state.expect(RightParen).ok();
                Some(state.finish_at(cp, CppElementType::SourceFile))
            }
            _ => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.bump();
                let right = PrattParser::parse(state, prec + (assoc as u8), self);
                state.push_child(right);
                Some(state.finish_at(cp, CppElementType::SourceFile))
            }
        }
    }
}
