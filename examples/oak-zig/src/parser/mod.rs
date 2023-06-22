use crate::{kind::ZigSyntaxKind, language::ZigLanguage, lexer::ZigLexer};
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
        use crate::kind::ZigSyntaxKind::*;
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(Identifier) => {
                state.bump();
                state.finish_at(cp, Identifier.into())
            }
            Some(IntegerLiteral) | Some(FloatLiteral) | Some(StringLiteral) | Some(CharLiteral) | Some(BooleanLiteral) | Some(Null) | Some(Undefined) => {
                state.bump();
                state.finish_at(cp, Root.into()) // Simplified
            }
            Some(LeftParen) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(RightParen).ok();
                state.finish_at(cp, Root.into())
            }
            _ => {
                state.bump();
                state.finish_at(cp, Error.into())
            }
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, ZigLanguage> {
        use crate::kind::ZigSyntaxKind::*;
        let kind = match state.peek_kind() {
            Some(k) => k,
            None => return self.primary(state),
        };

        match kind {
            Minus | Tilde | Exclamation | Ampersand | TryKeyword | AwaitKeyword => unary(state, kind, 12, Root.into(), |s, p| PrattParser::parse(s, p, self)),
            _ => self.primary(state),
        }
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, ZigLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, ZigLanguage>> {
        use crate::kind::ZigSyntaxKind::*;
        let kind = state.peek_kind()?;

        let (prec, assoc) = match kind {
            Assign | PlusAssign | MinusAssign | StarAssign | SlashAssign | PercentAssign | AmpersandAssign | PipeAssign | CaretAssign | LessLessAssign | GreaterGreaterAssign => (1, Associativity::Right),
            Or | OrOr => (2, Associativity::Left),
            And | AndAnd => (3, Associativity::Left),
            Equal | NotEqual | Less | Greater | LessEqual | GreaterEqual => (4, Associativity::Left),
            Plus | Minus | PlusPercent | MinusPercent | PlusPlus => (5, Associativity::Left),
            Star | Slash | Percent | StarPercent | StarStar => (6, Associativity::Left),
            CatchKeyword | OrElse => (7, Associativity::Right),
            Dot | LeftParen | LeftBracket => (10, Associativity::Left),
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
                Some(state.finish_at(cp, Root.into()))
            }
            LeftParen => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.bump();
                while state.not_at_end() && !state.at(RightParen) {
                    PrattParser::parse(state, 0, self);
                    if !state.eat(Comma) {
                        break;
                    }
                }
                state.expect(RightParen).ok();
                Some(state.finish_at(cp, Root.into()))
            }
            LeftBracket => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(RightBracket).ok();
                Some(state.finish_at(cp, Root.into()))
            }
            _ => Some(binary(state, left, kind, prec, assoc, Root.into(), |s, p| PrattParser::parse(s, p, self))),
        }
    }
}

impl<'config> Parser<ZigLanguage> for ZigParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<ZigLanguage>) -> ParseOutput<'a, ZigLanguage> {
        let lexer = ZigLexer::new(&self.config);
        parse_with_lexer(&lexer, source, edits, cache, |state| self.parse_root_internal(state))
    }
}

impl<'p> ZigParser<'p> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ZigLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            self.parse_statement(state)?;
        }

        Ok(state.finish_at(checkpoint, ZigSyntaxKind::Root.into()))
    }

    fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::ZigSyntaxKind::*;
        match state.peek_kind() {
            Some(Fn) => self.parse_function_declaration(state)?,
            Some(Const) | Some(Var) => self.parse_variable_declaration(state)?,
            Some(If) => self.parse_if_statement(state)?,
            Some(While) => self.parse_while_statement(state)?,
            Some(For) => self.parse_for_statement(state)?,
            Some(Return) => self.parse_return_statement(state)?,
            Some(LeftBrace) => self.parse_block(state)?,
            _ => {
                PrattParser::parse(state, 0, self);
                state.eat(Semicolon);
            }
        }
        Ok(())
    }

    fn parse_function_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::ZigSyntaxKind::*;
        let cp = state.checkpoint();
        state.expect(Fn).ok();
        state.expect(Identifier).ok();
        state.expect(LeftParen).ok();
        while state.not_at_end() && !state.at(RightParen) {
            state.advance();
        }
        state.expect(RightParen).ok();
        while state.not_at_end() && !state.at(LeftBrace) {
            state.bump();
        }
        self.parse_block(state)?;
        state.finish_at(cp, ZigSyntaxKind::FnDeclaration.into());
        Ok(())
    }

    fn parse_variable_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::ZigSyntaxKind::*;
        let cp = state.checkpoint();
        state.bump(); // const or var
        state.expect(Identifier).ok();
        if state.eat(Colon) {
            while state.not_at_end() && !state.at(Assign) && !state.at(Semicolon) {
                state.bump();
            }
        }
        if state.eat(Assign) {
            PrattParser::parse(state, 0, self);
        }
        state.eat(Semicolon);
        state.finish_at(cp, ZigSyntaxKind::VarDeclaration.into());
        Ok(())
    }

    fn parse_if_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::ZigSyntaxKind::*;
        let cp = state.checkpoint();
        state.expect(If).ok();
        state.expect(LeftParen).ok();
        PrattParser::parse(state, 0, self);
        state.expect(RightParen).ok();
        self.parse_statement(state)?;
        if state.eat(Else) {
            self.parse_statement(state)?;
        }
        state.finish_at(cp, ZigSyntaxKind::IfStatement.into());
        Ok(())
    }

    fn parse_while_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::ZigSyntaxKind::*;
        let cp = state.checkpoint();
        state.expect(While).ok();
        state.expect(LeftParen).ok();
        PrattParser::parse(state, 0, self);
        state.expect(RightParen).ok();
        self.parse_statement(state)?;
        state.finish_at(cp, ZigSyntaxKind::WhileStatement.into());
        Ok(())
    }

    fn parse_for_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::ZigSyntaxKind::*;
        let cp = state.checkpoint();
        state.expect(For).ok();
        state.expect(LeftParen).ok();
        PrattParser::parse(state, 0, self);
        state.expect(RightParen).ok();
        self.parse_statement(state)?;
        state.finish_at(cp, ZigSyntaxKind::ForStatement.into());
        Ok(())
    }

    fn parse_return_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::ZigSyntaxKind::*;
        let cp = state.checkpoint();
        state.expect(Return).ok();
        if !state.at(Semicolon) {
            PrattParser::parse(state, 0, self);
        }
        state.eat(Semicolon);
        state.finish_at(cp, ZigSyntaxKind::ReturnStatement.into());
        Ok(())
    }

    fn parse_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::ZigSyntaxKind::*;
        let cp = state.checkpoint();
        state.expect(LeftBrace).ok();
        while state.not_at_end() && !state.at(RightBrace) {
            self.parse_statement(state)?;
        }
        state.expect(RightBrace).ok();
        state.finish_at(cp, ZigSyntaxKind::Block.into());
        Ok(())
    }
}
