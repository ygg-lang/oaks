use crate::{kind::GoSyntaxKind, language::GoLanguage, lexer::GoLexer};
use oak_core::{
    OakError,
    parser::{Associativity, ParseCache, ParseOutput, Parser, ParserState, Pratt, PrattParser, binary, parse_with_lexer, unary},
    source::{Source, TextEdit},
    tree::GreenNode,
};

pub(crate) type State<'a, S> = ParserState<'a, GoLanguage, S>;

/// Go 语言解析器
pub struct GoParser<'config> {
    pub(crate) config: &'config GoLanguage,
}

impl<'config> GoParser<'config> {
    pub fn new(config: &'config GoLanguage) -> Self {
        Self { config }
    }

    pub(crate) fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::GoSyntaxKind::*;
        match state.peek_kind() {
            Some(Package) => self.parse_package_clause(state)?,
            Some(Import) => self.parse_import_declaration(state)?,
            Some(Func) => self.parse_function_declaration(state)?,
            Some(Var) => self.parse_variable_declaration(state)?,
            Some(Const) => self.parse_const_declaration(state)?,
            Some(Type) => self.parse_type_declaration(state)?,
            Some(If) => self.parse_if_statement(state)?,
            Some(For) => self.parse_for_statement(state)?,
            Some(Switch) => self.parse_switch_statement(state)?,
            Some(Return) => self.parse_return_statement(state)?,
            Some(LeftBrace) => self.parse_block(state)?,
            _ => {
                PrattParser::parse(state, 0, self);
                state.eat(Semicolon);
            }
        }
        Ok(())
    }

    fn parse_package_clause<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(GoSyntaxKind::Package).ok();
        state.expect(GoSyntaxKind::Identifier).ok();
        state.finish_at(cp, GoSyntaxKind::PackageClause.into());
        Ok(())
    }

    fn parse_import_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(GoSyntaxKind::Import).ok();
        if state.eat(GoSyntaxKind::LeftParen) {
            while state.not_at_end() && !state.at(GoSyntaxKind::RightParen) {
                state.advance();
            }
            state.expect(GoSyntaxKind::RightParen).ok();
        }
        else {
            state.advance();
        }
        state.finish_at(cp, GoSyntaxKind::ImportDeclaration.into());
        Ok(())
    }

    fn parse_function_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(GoSyntaxKind::Func).ok();
        while state.not_at_end() && !state.at(GoSyntaxKind::LeftBrace) {
            state.advance();
        }
        self.parse_block(state)?;
        state.finish_at(cp, GoSyntaxKind::FunctionDeclaration.into());
        Ok(())
    }

    fn parse_variable_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(GoSyntaxKind::Var).ok();
        while state.at(GoSyntaxKind::Identifier) {
            state.bump();
            if !state.eat(GoSyntaxKind::Semicolon) {
                break;
            }
        }
        state.eat(GoSyntaxKind::Semicolon);
        state.finish_at(cp, GoSyntaxKind::VariableDeclaration.into());
        Ok(())
    }

    fn parse_const_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(GoSyntaxKind::Const).ok();
        while state.at(GoSyntaxKind::Identifier) {
            state.bump();
            if !state.eat(GoSyntaxKind::Semicolon) {
                break;
            }
        }
        state.eat(GoSyntaxKind::Semicolon);
        state.finish_at(cp, GoSyntaxKind::ConstDeclaration.into());
        Ok(())
    }

    fn parse_type_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(GoSyntaxKind::Type).ok();
        while state.at(GoSyntaxKind::Identifier) {
            state.bump();
            if !state.eat(GoSyntaxKind::Semicolon) {
                break;
            }
        }
        state.eat(GoSyntaxKind::Semicolon);
        state.finish_at(cp, GoSyntaxKind::TypeDeclaration.into());
        Ok(())
    }

    fn parse_if_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(GoSyntaxKind::If).ok();
        while state.not_at_end() && !state.at(GoSyntaxKind::LeftBrace) {
            state.advance();
        }
        self.parse_block(state)?;
        if state.eat(GoSyntaxKind::Else) {
            if state.at(GoSyntaxKind::If) {
                self.parse_if_statement(state)?;
            }
            else {
                self.parse_block(state)?;
            }
        }
        state.finish_at(cp, GoSyntaxKind::IfStatement.into());
        Ok(())
    }

    fn parse_for_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(GoSyntaxKind::For).ok();
        while state.not_at_end() && !state.at(GoSyntaxKind::LeftBrace) {
            state.advance();
        }
        self.parse_block(state)?;
        state.finish_at(cp, GoSyntaxKind::ForStatement.into());
        Ok(())
    }

    fn parse_switch_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(GoSyntaxKind::Switch).ok();
        while state.not_at_end() && !state.at(GoSyntaxKind::LeftBrace) {
            state.advance();
        }
        self.parse_block(state)?;
        state.finish_at(cp, GoSyntaxKind::SwitchStatement.into());
        Ok(())
    }

    fn parse_return_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(GoSyntaxKind::Return).ok();
        if !state.at(GoSyntaxKind::Semicolon) {
            PrattParser::parse(state, 0, self);
        }
        state.eat(GoSyntaxKind::Semicolon);
        state.finish_at(cp, GoSyntaxKind::ReturnStatement.into());
        Ok(())
    }

    fn parse_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(GoSyntaxKind::LeftBrace).ok();
        while state.not_at_end() && !state.at(GoSyntaxKind::RightBrace) {
            self.parse_statement(state)?;
        }
        state.expect(GoSyntaxKind::RightBrace).ok();
        state.finish_at(cp, GoSyntaxKind::Block.into());
        Ok(())
    }
}

impl<'config> Pratt<GoLanguage> for GoParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, GoLanguage> {
        use crate::kind::GoSyntaxKind::*;
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(Identifier) => {
                state.bump();
                state.finish_at(cp, Identifier.into())
            }
            Some(IntLiteral) | Some(FloatLiteral) | Some(StringLiteral) | Some(RuneLiteral) | Some(BoolLiteral) | Some(NilLiteral) => {
                state.bump();
                state.finish_at(cp, BinaryExpression.into()) // 简化处理
            }
            Some(LeftParen) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(RightParen).ok();
                state.finish_at(cp, BinaryExpression.into())
            }
            _ => {
                state.bump();
                state.finish_at(cp, Error.into())
            }
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, GoLanguage> {
        use crate::kind::GoSyntaxKind::*;
        let kind = match state.peek_kind() {
            Some(k) => k,
            None => return self.primary(state),
        };

        let prec = match kind {
            Plus | Minus | LogicalNot | Caret | Star | Ampersand | Arrow => 7,
            _ => return self.primary(state),
        };

        unary(state, kind, prec, BinaryExpression.into(), |st, p| PrattParser::parse(st, p, self))
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, GoLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, GoLanguage>> {
        use crate::kind::GoSyntaxKind::*;
        let kind = state.peek_kind()?;

        let (prec, assoc) = match kind {
            Star | Slash | Percent | LeftShift | RightShift | Ampersand | AmpersandCaret => (6, Associativity::Left),
            Plus | Minus | Pipe | Caret => (5, Associativity::Left),
            Equal | NotEqual | Less | LessEqual | Greater | GreaterEqual => (4, Associativity::Left),
            LogicalAnd => (3, Associativity::Left),
            LogicalOr => (2, Associativity::Left),
            _ => return None,
        };

        if prec < min_precedence {
            return None;
        }

        Some(binary(state, left, kind, prec, assoc, BinaryExpression.into(), |st, p| PrattParser::parse(st, p, self)))
    }
}

impl<'config> Parser<GoLanguage> for GoParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<GoLanguage>) -> ParseOutput<'a, GoLanguage> {
        let lexer = GoLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let cp = state.checkpoint();
            while state.not_at_end() {
                self.parse_statement(state)?;
            }
            Ok(state.finish_at(cp, GoSyntaxKind::SourceFile.into()))
        })
    }
}
