use crate::{SqlLanguage, kind::SqlSyntaxKind};
use oak_core::{
    GreenNode, OakError, Parser, ParserState, TextEdit, TokenType,
    parser::{
        ParseCache, ParseOutput, parse_with_lexer,
        pratt::{Associativity, Pratt, PrattParser, binary},
    },
    source::Source,
};

/// SQL 解析器
pub struct SqlParser<'config> {
    pub(crate) config: &'config SqlLanguage,
}

type State<'a, S> = ParserState<'a, SqlLanguage, S>;

impl<'config> Pratt<SqlLanguage> for SqlParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, SqlLanguage> {
        use crate::kind::SqlSyntaxKind::*;
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(Identifier_) => {
                state.bump();
                state.finish_at(cp, Identifier.into())
            }
            Some(NumberLiteral) | Some(StringLiteral) | Some(BooleanLiteral) | Some(NullLiteral) => {
                state.bump();
                state.finish_at(cp, Expression.into())
            }
            Some(LeftParen) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(RightParen).ok();
                state.finish_at(cp, Expression.into())
            }
            _ => {
                state.bump();
                state.finish_at(cp, ErrorNode.into())
            }
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, SqlLanguage> {
        self.primary(state)
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, SqlLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, SqlLanguage>> {
        use crate::kind::SqlSyntaxKind::*;
        let kind = state.peek_kind()?;

        let (prec, assoc) = match kind {
            Or => (1, Associativity::Left),
            And => (2, Associativity::Left),
            Equal | NotEqual | Less | Greater | LessEqual | GreaterEqual | Like | In | Between | Is => (3, Associativity::Left),
            Plus | Minus => (10, Associativity::Left),
            Star | Slash | Percent => (11, Associativity::Left),
            _ => return None,
        };

        if prec < min_precedence {
            return None;
        }

        Some(binary(state, left, kind, prec, assoc, Expression.into(), |s, p| PrattParser::parse(s, p, self)))
    }
}

impl<'config> SqlParser<'config> {
    pub fn new(config: &'config SqlLanguage) -> Self {
        Self { config }
    }

    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, SqlLanguage>, OakError> {
        let cp = state.checkpoint();
        while state.not_at_end() {
            if state.current().map(|t| t.kind.is_ignored()).unwrap_or(false) {
                state.advance();
                continue;
            }
            self.parse_statement(state)?;
        }
        Ok(state.finish_at(cp, SqlSyntaxKind::Root.into()))
    }

    fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::SqlSyntaxKind::*;
        match state.peek_kind() {
            Some(Select) => self.parse_select(state)?,
            Some(Insert) => self.parse_insert(state)?,
            Some(Update) => self.parse_update(state)?,
            Some(Delete) => self.parse_delete(state)?,
            Some(Create) => self.parse_create(state)?,
            Some(Drop) => self.parse_drop(state)?,
            Some(Alter) => self.parse_alter(state)?,
            _ => {
                state.advance_until(Semicolon);
                state.eat(Semicolon);
            }
        }
        Ok(())
    }

    fn parse_select<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::SqlSyntaxKind::*;
        let cp = state.checkpoint();
        state.expect(Select).ok();
        state.advance_until(From);
        state.expect(From).ok();
        state.expect(Identifier_).ok(); // TableName
        if state.eat(Where) {
            PrattParser::parse(state, 0, self);
        }
        state.eat(Semicolon);
        state.finish_at(cp, SelectStatement.into());
        Ok(())
    }

    fn parse_insert<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // insert
        state.advance_until(crate::kind::SqlSyntaxKind::Semicolon);
        state.eat(crate::kind::SqlSyntaxKind::Semicolon);
        state.finish_at(cp, crate::kind::SqlSyntaxKind::InsertStatement.into());
        Ok(())
    }

    fn parse_update<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // update
        state.advance_until(crate::kind::SqlSyntaxKind::Semicolon);
        state.eat(crate::kind::SqlSyntaxKind::Semicolon);
        state.finish_at(cp, crate::kind::SqlSyntaxKind::UpdateStatement.into());
        Ok(())
    }

    fn parse_delete<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // delete
        state.advance_until(crate::kind::SqlSyntaxKind::Semicolon);
        state.eat(crate::kind::SqlSyntaxKind::Semicolon);
        state.finish_at(cp, crate::kind::SqlSyntaxKind::DeleteStatement.into());
        Ok(())
    }

    fn parse_create<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // create
        state.advance_until(crate::kind::SqlSyntaxKind::Semicolon);
        state.eat(crate::kind::SqlSyntaxKind::Semicolon);
        state.finish_at(cp, crate::kind::SqlSyntaxKind::CreateStatement.into());
        Ok(())
    }

    fn parse_drop<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // drop
        state.advance_until(crate::kind::SqlSyntaxKind::Semicolon);
        state.eat(crate::kind::SqlSyntaxKind::Semicolon);
        state.finish_at(cp, crate::kind::SqlSyntaxKind::DropStatement.into());
        Ok(())
    }

    fn parse_alter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // alter
        state.advance_until(crate::kind::SqlSyntaxKind::Semicolon);
        state.eat(crate::kind::SqlSyntaxKind::Semicolon);
        state.finish_at(cp, crate::kind::SqlSyntaxKind::AlterStatement.into());
        Ok(())
    }
}

impl<'config> Parser<SqlLanguage> for SqlParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<SqlLanguage>) -> ParseOutput<'a, SqlLanguage> {
        let lexer = crate::lexer::SqlLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
