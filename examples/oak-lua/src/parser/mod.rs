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
        use crate::kind::LuaSyntaxKind::*;
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            self.parse_statement(state).ok();
        }

        Ok(state.finish_at(checkpoint, SourceFile.into()))
    }

    fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::LuaSyntaxKind::*;
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
            _ => {
                PrattParser::parse(state, 0, self);
                state.eat(Semicolon);
            }
        }
        Ok(())
    }

    fn parse_local_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::LuaSyntaxKind::*;
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
        state.finish_at(cp, LocalStatement.into());
        Ok(())
    }

    fn parse_if_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::LuaSyntaxKind::*;
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
        state.finish_at(cp, IfStatement.into());
        Ok(())
    }

    fn parse_while_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::LuaSyntaxKind::*;
        let cp = state.checkpoint();
        state.expect(While).ok();
        PrattParser::parse(state, 0, self);
        state.expect(Do).ok();
        self.parse_block(state)?;
        state.expect(End).ok();
        state.finish_at(cp, WhileStatement.into());
        Ok(())
    }

    fn parse_for_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::LuaSyntaxKind::*;
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
        state.finish_at(cp, ForStatement.into());
        Ok(())
    }

    fn parse_repeat_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::LuaSyntaxKind::*;
        let cp = state.checkpoint();
        state.expect(Repeat).ok();
        self.parse_block(state)?;
        state.expect(Until).ok();
        PrattParser::parse(state, 0, self);
        state.finish_at(cp, RepeatStatement.into());
        Ok(())
    }

    fn parse_function_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::LuaSyntaxKind::*;
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
        state.finish_at(cp, FunctionDeclaration.into());
        Ok(())
    }

    fn parse_function_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::LuaSyntaxKind::*;
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
        use crate::kind::LuaSyntaxKind::*;
        let cp = state.checkpoint();
        state.expect(Return).ok();
        if !state.at(End) && !state.at(Else) && !state.at(Elseif) && !state.at(Until) && !state.at(Semicolon) {
            PrattParser::parse(state, 0, self);
            while state.eat(Comma) {
                PrattParser::parse(state, 0, self);
            }
        }
        state.eat(Semicolon);
        state.finish_at(cp, ReturnStatement.into());
        Ok(())
    }

    fn parse_break_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::LuaSyntaxKind::*;
        let cp = state.checkpoint();
        state.expect(Break).ok();
        state.finish_at(cp, BreakStatement.into());
        Ok(())
    }

    fn parse_do_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::LuaSyntaxKind::*;
        let cp = state.checkpoint();
        state.expect(Do).ok();
        self.parse_block(state)?;
        state.expect(End).ok();
        state.finish_at(cp, DoStatement.into());
        Ok(())
    }

    fn parse_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::LuaSyntaxKind::*;
        while state.not_at_end() && !state.at(End) && !state.at(Else) && !state.at(Elseif) && !state.at(Until) {
            self.parse_statement(state)?;
        }
        Ok(())
    }

    fn parse_table_constructor<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, LuaLanguage> {
        use crate::kind::LuaSyntaxKind::*;
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
            else if state.at(Identifier) && state.peek_at(1).map(|t| t.kind == Eq).unwrap_or(false) {
                state.bump();
                state.bump();
                PrattParser::parse(state, 0, self);
            }
            else {
                PrattParser::parse(state, 0, self);
            }
            state.finish_at(field_cp, TableField.into());
            if !state.eat(Comma) && !state.eat(Semicolon) {
                break;
            }
        }
        state.expect(RightBrace).ok();
        state.finish_at(cp, TableConstructorExpression.into())
    }
}

impl<'config> Pratt<LuaLanguage> for LuaParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, LuaLanguage> {
        use crate::kind::LuaSyntaxKind::*;
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(Nil) | Some(False) | Some(True) | Some(Number) | Some(String) | Some(DotDotDot) => {
                state.bump();
                state.finish_at(cp, LiteralExpression.into())
            }
            Some(Identifier) => {
                state.bump();
                state.finish_at(cp, IdentifierExpression.into())
            }
            Some(Function) => {
                state.bump();
                self.parse_function_body(state).ok();
                state.finish_at(cp, FunctionExpression.into())
            }
            Some(LeftParen) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(RightParen).ok();
                state.finish_at(cp, ParenthesizedExpression.into())
            }
            Some(LeftBrace) => self.parse_table_constructor(state),
            _ => {
                state.bump();
                state.finish_at(cp, Error.into())
            }
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, LuaLanguage> {
        use crate::kind::LuaSyntaxKind::*;
        let kind = state.peek_kind();
        match kind {
            Some(Minus) | Some(Not) | Some(Hash) | Some(Tilde) => {
                let op_kind = kind.unwrap();
                state.bump();
                unary(state, op_kind, 80, UnaryExpression.into(), |s, p| PrattParser::parse(s, p, self))
            }
            _ => self.primary(state),
        }
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, LuaLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, LuaLanguage>> {
        use crate::kind::LuaSyntaxKind::*;
        let kind = state.peek_kind()?;

        let (prec, assoc) = match kind {
            Or => (1, Associativity::Left),
            And => (2, Associativity::Left),
            Lt | Gt | LtEq | GtEq | TildeEq | EqEq => (3, Associativity::Left),
            Pipe => (4, Associativity::Left),
            Tilde => (5, Associativity::Left),
            Ampersand => (6, Associativity::Left),
            LtLt | GtGt => (7, Associativity::Left),
            DotDot => (8, Associativity::Right),
            Plus | Minus => (9, Associativity::Left),
            Star | Slash | SlashSlash | Percent => (10, Associativity::Left),
            Caret => (12, Associativity::Right),
            Dot | Colon | LeftParen | LeftBrace | String | LeftBracket => (13, Associativity::Left),
            _ => return None,
        };

        if prec < min_precedence {
            return None;
        }

        match kind {
            Dot => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.bump();
                state.expect(Identifier).ok();
                Some(state.finish_at(cp, MemberExpression.into()))
            }
            Colon => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.bump();
                state.expect(Identifier).ok();
                Some(state.finish_at(cp, MemberExpression.into()))
            }
            LeftBracket => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(RightBracket).ok();
                Some(state.finish_at(cp, IndexExpression.into()))
            }
            LeftParen | LeftBrace | String => {
                let cp = state.checkpoint();
                state.push_child(left);
                if state.eat(LeftParen) {
                    while state.not_at_end() && !state.at(RightParen) {
                        PrattParser::parse(state, 0, self);
                        if !state.eat(Comma) {
                            break;
                        }
                    }
                    state.expect(RightParen).ok();
                }
                else {
                    state.bump();
                }
                Some(state.finish_at(cp, CallExpression.into()))
            }
            _ => Some(binary(state, left, kind, prec, assoc, BinaryExpression.into(), |s, p| PrattParser::parse(s, p, self))),
        }
    }
}

impl<'config> Parser<LuaLanguage> for LuaParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<LuaLanguage>) -> ParseOutput<'a, LuaLanguage> {
        let lexer = LuaLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
