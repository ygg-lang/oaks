pub mod element_type;
pub use element_type::CElementType;

use crate::{language::CLanguage, lexer::CTokenType};
use oak_core::{
    GreenNode, OakError, Source,
    parser::{Associativity, ParseCache, ParseOutput, Parser, ParserState, Pratt, PrattParser, binary, parse_with_lexer},
    source::TextEdit,
};

pub(crate) type State<'a, S> = ParserState<'a, CLanguage, S>;

pub struct CParser<'config> {
    pub(crate) config: &'config CLanguage,
}

impl<'config> CParser<'config> {
    pub fn new(config: &'config CLanguage) -> Self {
        Self { config }
    }

    pub(crate) fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::CTokenType::*;
        match state.peek_kind() {
            Some(If) => self.parse_if_statement(state)?,
            Some(While) => self.parse_while_statement(state)?,
            Some(For) => self.parse_for_statement(state)?,
            Some(Return) => self.parse_return_statement(state)?,
            Some(LeftBrace) => self.parse_compound_statement(state)?,
            Some(Struct) | Some(Union) | Some(Enum) | Some(Typedef) | Some(Extern) | Some(Static) | Some(Int) | Some(Char) | Some(Void) | Some(Float) | Some(Double) => self.parse_declaration(state)?,
            _ => {
                PrattParser::parse(state, 0, self);
                state.eat(Semicolon);
            }
        }
        Ok(())
    }

    fn parse_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::CTokenType::*;
        let cp = state.checkpoint();
        while state.not_at_end() && !state.at(CTokenType::Semicolon) && !state.at(CTokenType::LeftBrace) {
            state.advance();
        }
        if state.eat(LeftBrace) {
            while state.not_at_end() && !state.at(RightBrace) {
                self.parse_statement(state)?;
            }
            state.expect(RightBrace).ok();
            state.finish_at(cp, CElementType::FunctionDefinition);
        }
        else {
            state.eat(Semicolon);
            state.finish_at(cp, CElementType::DeclarationStatement);
        }
        Ok(())
    }

    fn parse_if_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // if
        state.expect(CTokenType::LeftParen).ok();
        PrattParser::parse(state, 0, self);
        state.expect(CTokenType::RightParen).ok();
        self.parse_statement(state)?;
        if state.eat(CTokenType::Else) {
            self.parse_statement(state)?;
        }
        state.finish_at(cp, CElementType::IfStatement);
        Ok(())
    }

    fn parse_while_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // while
        state.expect(CTokenType::LeftParen).ok();
        PrattParser::parse(state, 0, self);
        state.expect(CTokenType::RightParen).ok();
        self.parse_statement(state)?;
        state.finish_at(cp, CElementType::WhileStatement);
        Ok(())
    }

    fn parse_for_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // for
        state.expect(CTokenType::LeftParen).ok();
        while state.not_at_end() && !state.at(CTokenType::RightParen) {
            state.advance();
        }
        state.eat(CTokenType::RightParen);
        self.parse_statement(state)?;
        state.finish_at(cp, CElementType::ForStatement);
        Ok(())
    }

    fn parse_return_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // return
        if !state.at(CTokenType::Semicolon) {
            PrattParser::parse(state, 0, self);
        }
        state.eat(CTokenType::Semicolon);
        state.finish_at(cp, CElementType::ReturnStatement);
        Ok(())
    }

    fn parse_compound_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(CTokenType::LeftBrace).ok();
        while state.not_at_end() && !state.at(CTokenType::RightBrace) {
            self.parse_statement(state)?;
        }
        state.expect(CTokenType::RightBrace).ok();
        state.finish_at(cp, CElementType::CompoundStatement);
        Ok(())
    }
}

impl<'config> Pratt<CLanguage> for CParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, CLanguage> {
        use crate::lexer::CTokenType::*;
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(Identifier) => {
                state.bump();
                state.finish_at(cp, CElementType::Token(Identifier))
            }
            Some(IntegerLiteral) | Some(FloatLiteral) | Some(CharLiteral) | Some(StringLiteral) => {
                let _kind = state.peek_kind().unwrap();
                state.bump();
                state.finish_at(cp, CElementType::ExpressionStatement) // 简化处理
            }
            Some(LeftParen) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(RightParen).ok();
                state.finish_at(cp, CElementType::ExpressionStatement)
            }
            _ => {
                state.bump();
                state.finish_at(cp, CElementType::Error)
            }
        }
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, CLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, CLanguage>> {
        use crate::lexer::CTokenType::*;
        let kind = state.peek_kind()?;

        let (prec, assoc) = match kind {
            Assign | PlusAssign | MinusAssign | StarAssign | SlashAssign | PercentAssign | AndAssign | OrAssign | XorAssign | LeftShiftAssign | RightShiftAssign => (1, Associativity::Right),
            LogicalOr => (2, Associativity::Left),
            LogicalAnd => (3, Associativity::Left),
            Equal | NotEqual | Less | Greater | LessEqual | GreaterEqual => (4, Associativity::Left),
            Plus | Minus => (10, Associativity::Left),
            Star | Slash | Percent => (11, Associativity::Left),
            LeftParen | LeftBracket | Dot | Arrow => (15, Associativity::Left),
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
                    state.advance();
                }
                state.expect(RightParen).ok();
                Some(state.finish_at(cp, CElementType::ExpressionStatement))
            }
            LeftBracket => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(LeftBracket).ok();
                while state.not_at_end() && !state.at(RightBracket) {
                    state.advance();
                }
                state.expect(RightBracket).ok();
                Some(state.finish_at(cp, CElementType::ExpressionStatement))
            }
            Dot | Arrow => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(kind).ok();
                state.expect(Identifier).ok();
                Some(state.finish_at(cp, CElementType::ExpressionStatement))
            }
            _ => Some(binary(state, left, kind, prec, assoc, CElementType::ExpressionStatement, |s, p| PrattParser::parse(s, p, self))),
        }
    }
}

impl<'config> Parser<CLanguage> for CParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<CLanguage>) -> ParseOutput<'a, CLanguage> {
        let lexer = crate::lexer::CLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let cp = state.checkpoint();
            while state.not_at_end() {
                self.parse_statement(state).ok();
            }
            Ok(state.finish_at(cp, CElementType::Root))
        })
    }
}
