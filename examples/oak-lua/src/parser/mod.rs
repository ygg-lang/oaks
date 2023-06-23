pub mod element_type;
pub use element_type::LuaElementType;

use crate::{language::LuaLanguage, lexer::LuaLexer};
use oak_core::{
    GreenNode, OakError, TextEdit,
    parser::{
        ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer,
        pratt::{Associativity, Pratt, PrattParser, binary, unary},
    },
    source::Source,
};

pub(crate) type State<'a, S> = ParserState<'a, LuaLanguage, S>;

pub struct LuaParser<'config> {
    pub(crate) config: &'config LuaLanguage,
}

impl<'config> LuaParser<'config> {
    pub fn new(config: &'config LuaLanguage) -> Self {
        Self { config }
    }

    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, LuaLanguage>, OakError> {
        use crate::lexer::LuaTokenType::*;
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            self.parse_statement(state).ok();
        }

        Ok(state.finish_at(checkpoint, crate::parser::element_type::LuaElementType::SourceFile))
    }

    fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::LuaTokenType::*;
        match state.peek_kind() {
            Some(Local) => self.parse_local_statement(state)?,
            Some(If) => self.parse_if_statement(state)?,
            Some(While) => self.parse_while_statement(state)?,
            Some(For) => self.parse_for_statement(state)?,
            Some(Repeat) => self.parse_repeat_statement(state)?,
            Some(Function) => self.parse_function_declaration(state)?,
            Some(Return) => self.parse_return_statement(state)?,
            Some(Break) => self.parse_break_statement(state)?,
            Some(Do) => self.parse_do_statement(state)?,
            Some(Goto) => self.parse_goto_statement(state)?,
            Some(ColonColon) => self.parse_label_statement(state)?,
            _ => {
                let cp = state.checkpoint();
                PrattParser::parse(state, 0, self);
                if state.at(Comma) || state.at(Eq) {
                    while state.eat(Comma) {
                        PrattParser::parse(state, 0, self);
                    }
                    state.expect(Eq).ok();
                    PrattParser::parse(state, 0, self);
                    while state.eat(Comma) {
                        PrattParser::parse(state, 0, self);
                    }
                    state.finish_at(cp, crate::parser::element_type::LuaElementType::AssignmentStatement);
                }
                else {
                    state.finish_at(cp, crate::parser::element_type::LuaElementType::ExpressionStatement);
                }
                state.eat(Semicolon);
            }
        }
        Ok(())
    }

    fn parse_local_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::LuaTokenType::*;
        let cp = state.checkpoint();
        state.expect(Local).ok();
        if state.eat(Function) {
            state.expect(Identifier).ok();
            self.parse_function_body(state)?;
        }
        else {
            state.expect(Identifier).ok();
            while state.eat(Comma) {
                state.expect(Identifier).ok();
            }
            if state.eat(Eq) {
                PrattParser::parse(state, 0, self);
                while state.eat(Comma) {
                    PrattParser::parse(state, 0, self);
                }
            }
        }
        state.eat(Semicolon);
        state.finish_at(cp, crate::parser::element_type::LuaElementType::LocalStatement);
        Ok(())
    }

    fn parse_if_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::LuaTokenType::*;
        let cp = state.checkpoint();
        state.expect(If).ok();
        PrattParser::parse(state, 0, self);
        state.expect(Then).ok();
        self.parse_block(state)?;
        while state.eat(Elseif) {
            PrattParser::parse(state, 0, self);
            state.expect(Then).ok();
            self.parse_block(state)?;
        }
        if state.eat(Else) {
            self.parse_block(state)?;
        }
        state.expect(End).ok();
        state.finish_at(cp, crate::parser::element_type::LuaElementType::IfStatement);
        Ok(())
    }

    fn parse_while_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::LuaTokenType::*;
        let cp = state.checkpoint();
        state.expect(While).ok();
        PrattParser::parse(state, 0, self);
        state.expect(Do).ok();
        self.parse_block(state)?;
        state.expect(End).ok();
        state.finish_at(cp, crate::parser::element_type::LuaElementType::WhileStatement);
        Ok(())
    }

    fn parse_for_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::LuaTokenType::*;
        let cp = state.checkpoint();
        state.expect(For).ok();
        state.expect(Identifier).ok();
        if state.eat(Eq) {
            PrattParser::parse(state, 0, self);
            state.expect(Comma).ok();
            PrattParser::parse(state, 0, self);
            if state.eat(Comma) {
                PrattParser::parse(state, 0, self);
            }
        }
        else {
            while state.eat(Comma) {
                state.expect(Identifier).ok();
            }
            state.expect(In).ok();
            PrattParser::parse(state, 0, self);
            while state.eat(Comma) {
                PrattParser::parse(state, 0, self);
            }
        }
        state.expect(Do).ok();
        self.parse_block(state)?;
        state.expect(End).ok();
        state.finish_at(cp, crate::parser::element_type::LuaElementType::ForStatement);
        Ok(())
    }

    fn parse_repeat_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::LuaTokenType::*;
        let cp = state.checkpoint();
        state.expect(Repeat).ok();
        self.parse_block(state)?;
        state.expect(Until).ok();
        PrattParser::parse(state, 0, self);
        state.finish_at(cp, crate::parser::element_type::LuaElementType::RepeatStatement);
        Ok(())
    }

    fn parse_function_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::LuaTokenType::*;
        let cp = state.checkpoint();
        state.expect(Function).ok();
        state.expect(Identifier).ok();
        while state.eat(Dot) {
            state.expect(Identifier).ok();
        }
        if state.eat(Colon) {
            state.expect(Identifier).ok();
        }
        self.parse_function_body(state)?;
        state.finish_at(cp, crate::parser::element_type::LuaElementType::FunctionDeclaration);
        Ok(())
    }

    fn parse_function_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::LuaTokenType::*;
        state.expect(LeftParen).ok();
        if !state.at(RightParen) {
            if state.eat(DotDotDot) {
                // vararg
            }
            else {
                state.expect(Identifier).ok();
                while state.eat(Comma) {
                    if state.eat(DotDotDot) {
                        break;
                    }
                    state.expect(Identifier).ok();
                }
            }
        }
        state.expect(RightParen).ok();
        self.parse_block(state)?;
        state.expect(End).ok();
        Ok(())
    }

    fn parse_return_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::LuaTokenType::*;
        let cp = state.checkpoint();
        state.expect(Return).ok();
        if !state.at(End) && !state.at(Else) && !state.at(Elseif) && !state.at(Until) && !state.at(Semicolon) {
            PrattParser::parse(state, 0, self);
            while state.eat(Comma) {
                PrattParser::parse(state, 0, self);
            }
        }
        state.eat(Semicolon);
        state.finish_at(cp, crate::parser::element_type::LuaElementType::ReturnStatement);
        Ok(())
    }

    fn parse_break_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::LuaTokenType::*;
        let cp = state.checkpoint();
        state.expect(Break).ok();
        state.finish_at(cp, crate::parser::element_type::LuaElementType::BreakStatement);
        Ok(())
    }

    fn parse_do_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::LuaTokenType::*;
        let cp = state.checkpoint();
        state.expect(Do).ok();
        self.parse_block(state)?;
        state.expect(End).ok();
        state.finish_at(cp, crate::parser::element_type::LuaElementType::DoStatement);
        Ok(())
    }

    fn parse_goto_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::LuaTokenType::*;
        let cp = state.checkpoint();
        state.expect(Goto).ok();
        state.expect(Identifier).ok();
        state.finish_at(cp, crate::parser::element_type::LuaElementType::GotoStatement);
        Ok(())
    }

    fn parse_label_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::LuaTokenType::*;
        let cp = state.checkpoint();
        state.expect(ColonColon).ok();
        state.expect(Identifier).ok();
        state.expect(ColonColon).ok();
        state.finish_at(cp, crate::parser::element_type::LuaElementType::LabelStatement);
        Ok(())
    }

    fn parse_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::LuaTokenType::*;
        while state.not_at_end() && !state.at(End) && !state.at(Else) && !state.at(Elseif) && !state.at(Until) {
            self.parse_statement(state)?;
        }
        Ok(())
    }

    fn parse_table_constructor<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, LuaLanguage> {
        use crate::lexer::LuaTokenType::*;
        let cp = state.checkpoint();
        state.expect(LeftBrace).ok();
        while state.not_at_end() && !state.at(RightBrace) {
            let field_cp = state.checkpoint();
            if state.eat(LeftBracket) {
                PrattParser::parse(state, 0, self);
                state.expect(RightBracket).ok();
                state.expect(Eq).ok();
                PrattParser::parse(state, 0, self);
            }
            else if state.at(Identifier) && state.peek_non_trivia_kind_at(1) == Some(Eq) {
                state.bump();
                state.expect(Eq).ok();
                PrattParser::parse(state, 0, self);
            }
            else {
                PrattParser::parse(state, 0, self);
            }
            state.finish_at(field_cp, crate::parser::element_type::LuaElementType::TableField);
            if !state.eat(Comma) && !state.eat(Semicolon) {
                break;
            }
        }
        state.expect(RightBrace).ok();
        state.finish_at(cp, crate::parser::element_type::LuaElementType::TableConstructorExpression)
    }
}

impl<'config> Pratt<LuaLanguage> for LuaParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, LuaLanguage> {
        use crate::lexer::LuaTokenType::*;
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(Nil) | Some(False) | Some(True) | Some(Number) | Some(String) | Some(DotDotDot) => {
                state.bump();
                state.finish_at(cp, crate::parser::element_type::LuaElementType::LiteralExpression)
            }
            Some(Identifier) => {
                state.bump();
                state.finish_at(cp, crate::parser::element_type::LuaElementType::IdentifierExpression)
            }
            Some(Function) => {
                state.bump();
                self.parse_function_body(state).ok();
                state.finish_at(cp, crate::parser::element_type::LuaElementType::FunctionExpression)
            }
            Some(LeftParen) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(RightParen).ok();
                state.finish_at(cp, crate::parser::element_type::LuaElementType::ParenthesizedExpression)
            }
            Some(LeftBrace) => self.parse_table_constructor(state),
            _ => {
                use crate::lexer::LuaTokenType;
                let current_kind = state.peek_kind();
                state.record_expected("expression");
                if state.not_at_end() {
                    // Don't bump if we are at a token that could be part of the following structure
                    match current_kind {
                        Some(LuaTokenType::RightParen)
                        | Some(LuaTokenType::RightBrace)
                        | Some(LuaTokenType::RightBracket)
                        | Some(LuaTokenType::Comma)
                        | Some(LuaTokenType::Semicolon)
                        | Some(LuaTokenType::End)
                        | Some(LuaTokenType::Else)
                        | Some(LuaTokenType::Elseif)
                        | Some(LuaTokenType::Until) => {
                            // Don't bump, let the caller handle it
                        }
                        _ => {
                            state.bump();
                        }
                    }
                }
                state.finish_at(cp, crate::parser::element_type::LuaElementType::Error)
            }
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, LuaLanguage> {
        use crate::{lexer::LuaTokenType as TT, parser::element_type::LuaElementType as ET};
        let kind = state.peek_kind();
        match kind {
            Some(TT::Minus) | Some(TT::Not) | Some(TT::Hash) | Some(TT::Tilde) => {
                let op_kind = kind.unwrap();
                unary(state, op_kind, 11, ET::UnaryExpression, |s, p| PrattParser::parse(s, p, self))
            }
            _ => self.primary(state),
        }
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, LuaLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, LuaLanguage>> {
        use crate::{lexer::LuaTokenType as TT, parser::element_type::LuaElementType as ET};
        let kind = state.peek_kind()?;

        let (prec, assoc) = match kind {
            TT::Or => (1, Associativity::Left),
            TT::And => (2, Associativity::Left),
            TT::Lt | TT::Gt | TT::LtEq | TT::GtEq | TT::TildeEq | TT::EqEq => (3, Associativity::Left),
            TT::Pipe => (4, Associativity::Left),
            TT::Tilde => (5, Associativity::Left),
            TT::Ampersand => (6, Associativity::Left),
            TT::LtLt | TT::GtGt => (7, Associativity::Left),
            TT::DotDot => (8, Associativity::Right),
            TT::Plus | TT::Minus => (9, Associativity::Left),
            TT::Star | TT::Slash | TT::SlashSlash | TT::Percent => (10, Associativity::Left),
            TT::Caret => (12, Associativity::Right),
            TT::Dot | TT::Colon | TT::LeftParen | TT::LeftBrace | TT::String | TT::LeftBracket => (13, Associativity::Left),
            _ => return None,
        };

        if prec < min_precedence {
            return None;
        }

        match kind {
            TT::Dot => {
                let cp = state.checkpoint_before(left);
                state.bump();
                state.expect(TT::Identifier).ok();
                Some(state.finish_at(cp, ET::MemberExpression))
            }
            TT::Colon => {
                let cp = state.checkpoint_before(left);
                state.bump();
                state.expect(TT::Identifier).ok();
                Some(state.finish_at(cp, ET::MemberExpression))
            }
            TT::LeftBracket => {
                let cp = state.checkpoint_before(left);
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(TT::RightBracket).ok();
                Some(state.finish_at(cp, ET::IndexExpression))
            }
            TT::LeftParen | TT::LeftBrace | TT::String => {
                let cp = state.checkpoint_before(left);
                if state.eat(TT::LeftParen) {
                    if !state.at(TT::RightParen) {
                        PrattParser::parse(state, 0, self);
                        while state.eat(TT::Comma) {
                            PrattParser::parse(state, 0, self);
                        }
                    }
                    state.expect(TT::RightParen).ok();
                }
                else if state.at(TT::LeftBrace) {
                    self.parse_table_constructor(state);
                }
                else if state.at(TT::String) {
                    state.bump();
                }
                else {
                    state.record_expected("arguments");
                }
                Some(state.finish_at(cp, ET::CallExpression))
            }
            _ => Some(binary(state, left, kind, prec, assoc, ET::BinaryExpression, |s, p| PrattParser::parse(s, p, self))),
        }
    }
}

impl<'config> Parser<LuaLanguage> for LuaParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<LuaLanguage>) -> ParseOutput<'a, LuaLanguage> {
        let lexer = LuaLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
