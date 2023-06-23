pub mod element_type;

use crate::{
    language::ZigLanguage,
    lexer::{ZigLexer, token_type::ZigTokenType},
    parser::element_type::ZigElementType,
};
use oak_core::{
    GreenNode, OakError,
    parser::{
        ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer,
        pratt::{Associativity, Pratt, PrattParser, binary, unary},
    },
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, ZigLanguage, S>;

pub struct ZigParser<'config> {
    pub(crate) config: &'config ZigLanguage,
}

impl<'config> ZigParser<'config> {
    pub fn new(config: &'config ZigLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Pratt<ZigLanguage> for ZigParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, ZigLanguage> {
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(ZigTokenType::Identifier) => {
                state.bump();
                state.finish_at(cp, ZigElementType::Identifier)
            }
            Some(ZigTokenType::IntegerLiteral) | Some(ZigTokenType::FloatLiteral) | Some(ZigTokenType::StringLiteral) | Some(ZigTokenType::CharLiteral) | Some(ZigTokenType::BooleanLiteral) | Some(ZigTokenType::Null) | Some(ZigTokenType::Undefined) => {
                state.bump();
                state.finish_at(cp, ZigElementType::Literal)
            }
            Some(ZigTokenType::LeftParen) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(ZigTokenType::RightParen).ok();
                state.finish_at(cp, ZigElementType::Root)
            }
            _ => {
                state.bump();
                state.finish_at(cp, ZigElementType::Error)
            }
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, ZigLanguage> {
        let kind = match state.peek_kind() {
            Some(k) => k,
            None => return self.primary(state),
        };

        match kind {
            ZigTokenType::Minus | ZigTokenType::Tilde | ZigTokenType::Exclamation | ZigTokenType::Ampersand | ZigTokenType::TryKeyword | ZigTokenType::AwaitKeyword => {
                // unary expects kind to be ZigLanguage::TokenType
                unary(state, kind, 12, ZigElementType::UnaryExpr, |s, p| PrattParser::parse(s, p, self))
            }
            _ => self.primary(state),
        }
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, ZigLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, ZigLanguage>> {
        let kind = state.peek_kind()?;

        let (prec, assoc) = match kind {
            ZigTokenType::Assign
            | ZigTokenType::PlusAssign
            | ZigTokenType::MinusAssign
            | ZigTokenType::StarAssign
            | ZigTokenType::SlashAssign
            | ZigTokenType::PercentAssign
            | ZigTokenType::AmpersandAssign
            | ZigTokenType::PipeAssign
            | ZigTokenType::CaretAssign
            | ZigTokenType::LessLessAssign
            | ZigTokenType::GreaterGreaterAssign => (1, Associativity::Right),
            ZigTokenType::Or | ZigTokenType::OrOr => (2, Associativity::Left),
            ZigTokenType::And | ZigTokenType::AndAnd => (3, Associativity::Left),
            ZigTokenType::Equal | ZigTokenType::NotEqual | ZigTokenType::Less | ZigTokenType::Greater | ZigTokenType::LessEqual | ZigTokenType::GreaterEqual => (4, Associativity::Left),
            ZigTokenType::Plus | ZigTokenType::Minus | ZigTokenType::PlusPercent | ZigTokenType::MinusPercent | ZigTokenType::PlusPlus => (5, Associativity::Left),
            ZigTokenType::Star | ZigTokenType::Slash | ZigTokenType::Percent | ZigTokenType::StarPercent | ZigTokenType::StarStar => (6, Associativity::Left),
            ZigTokenType::CatchKeyword | ZigTokenType::OrElse => (7, Associativity::Right),
            ZigTokenType::Dot | ZigTokenType::LeftParen | ZigTokenType::LeftBracket => (10, Associativity::Left),
            _ => return None,
        };

        if prec < min_precedence {
            return None;
        }

        match kind {
            ZigTokenType::Dot => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.bump();
                state.expect(ZigTokenType::Identifier).ok();
                Some(state.finish_at(cp, ZigElementType::Root))
            }
            ZigTokenType::LeftParen => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.bump();
                while state.not_at_end() && !state.at(ZigTokenType::RightParen) {
                    PrattParser::parse(state, 0, self);
                    if !state.eat(ZigTokenType::Comma) {
                        break;
                    }
                }
                state.expect(ZigTokenType::RightParen).ok();
                Some(state.finish_at(cp, ZigElementType::Root))
            }
            ZigTokenType::LeftBracket => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(ZigTokenType::RightBracket).ok();
                Some(state.finish_at(cp, ZigElementType::Root))
            }
            _ => Some(binary(state, left, kind, prec, assoc, ZigElementType::BinaryExpr.into(), |s, p| PrattParser::parse(s, p, self))),
        }
    }
}

impl<'config> Parser<ZigLanguage> for ZigParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<ZigLanguage>) -> ParseOutput<'a, ZigLanguage> {
        let lexer = ZigLexer::new(self.config);
        parse_with_lexer(&lexer, source, edits, cache, |state| self.parse_root_internal(state))
    }
}

impl<'p> ZigParser<'p> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ZigLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            self.parse_statement(state)?;
        }

        Ok(state.finish_at(checkpoint, ZigElementType::Root))
    }

    fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        match state.peek_kind() {
            Some(ZigTokenType::Fn) => self.parse_function_declaration(state)?,
            Some(ZigTokenType::Const) | Some(ZigTokenType::Var) => self.parse_variable_declaration(state)?,
            Some(ZigTokenType::If) => self.parse_if_statement(state)?,
            Some(ZigTokenType::While) => self.parse_while_statement(state)?,
            Some(ZigTokenType::For) => self.parse_for_statement(state)?,
            Some(ZigTokenType::Return) => self.parse_return_statement(state)?,
            Some(ZigTokenType::LeftBrace) => self.parse_block(state)?,
            _ => {
                PrattParser::parse(state, 0, self);
                state.eat(ZigTokenType::Semicolon);
            }
        }
        Ok(())
    }

    fn parse_function_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(ZigTokenType::Fn).ok();
        state.expect(ZigTokenType::Identifier).ok();
        state.expect(ZigTokenType::LeftParen).ok();
        while state.not_at_end() && !state.at(ZigTokenType::RightParen) {
            state.advance();
        }
        state.expect(ZigTokenType::RightParen).ok();
        while state.not_at_end() && !state.at(ZigTokenType::LeftBrace) {
            state.bump();
        }
        self.parse_block(state)?;
        state.finish_at(cp, ZigElementType::FnDeclaration);
        Ok(())
    }

    fn parse_variable_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // const or var
        state.expect(ZigTokenType::Identifier).ok();
        if state.eat(ZigTokenType::Colon) {
            while state.not_at_end() && !state.at(ZigTokenType::Assign) && !state.at(ZigTokenType::Semicolon) {
                state.bump();
            }
        }
        if state.eat(ZigTokenType::Assign) {
            PrattParser::parse(state, 0, self);
        }
        state.eat(ZigTokenType::Semicolon);
        state.finish_at(cp, ZigElementType::VarDeclaration);
        Ok(())
    }

    fn parse_if_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(ZigTokenType::If).ok();
        state.expect(ZigTokenType::LeftParen).ok();
        PrattParser::parse(state, 0, self);
        state.expect(ZigTokenType::RightParen).ok();
        self.parse_statement(state)?;
        if state.eat(ZigTokenType::Else) {
            self.parse_statement(state)?;
        }
        state.finish_at(cp, ZigElementType::IfStatement);
        Ok(())
    }

    fn parse_while_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(ZigTokenType::While).ok();
        state.expect(ZigTokenType::LeftParen).ok();
        PrattParser::parse(state, 0, self);
        state.expect(ZigTokenType::RightParen).ok();
        self.parse_statement(state)?;
        state.finish_at(cp, ZigElementType::WhileStatement);
        Ok(())
    }

    fn parse_for_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(ZigTokenType::For).ok();
        state.expect(ZigTokenType::LeftParen).ok();
        PrattParser::parse(state, 0, self);
        state.expect(ZigTokenType::RightParen).ok();
        self.parse_statement(state)?;
        state.finish_at(cp, ZigElementType::ForStatement);
        Ok(())
    }

    fn parse_return_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(ZigTokenType::Return).ok();
        if !state.at(ZigTokenType::Semicolon) {
            PrattParser::parse(state, 0, self);
        }
        state.eat(ZigTokenType::Semicolon);
        state.finish_at(cp, ZigElementType::ReturnStatement);
        Ok(())
    }

    fn parse_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(ZigTokenType::LeftBrace).ok();
        while state.not_at_end() && !state.at(ZigTokenType::RightBrace) {
            self.parse_statement(state)?;
        }
        state.expect(ZigTokenType::RightBrace).ok();
        state.finish_at(cp, ZigElementType::Block);
        Ok(())
    }
}
