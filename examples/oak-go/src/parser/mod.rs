pub mod element_type;

use crate::{language::GoLanguage, lexer::GoLexer};
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

    fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        while state.not_at_end() {
            if let Some(kind) = state.peek_kind() {
                if kind.is_ignored() {
                    state.bump();
                    continue;
                };
            }
            break;
        }
    }

    pub(crate) fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::{GoElementType as E, GoTokenType as T};
        self.skip_trivia(state);
        match state.peek_kind() {
            Some(T::Package) => self.parse_package_clause(state)?,
            Some(T::Import) => self.parse_import_declaration(state)?,
            Some(T::Func) => self.parse_function_declaration(state)?,
            Some(T::Var) => self.parse_variable_declaration(state)?,
            Some(T::Const) => self.parse_const_declaration(state)?,
            Some(T::Type) => self.parse_type_declaration(state)?,
            Some(T::If) => self.parse_if_statement(state)?,
            Some(T::For) => self.parse_for_statement(state)?,
            Some(T::Switch) => self.parse_switch_statement(state)?,
            Some(T::Return) => self.parse_return_statement(state)?,
            Some(T::LeftBrace) => self.parse_block(state)?,
            _ => {
                let cp = state.checkpoint();
                PrattParser::parse(state, 0, self);

                self.skip_trivia(state);
                match state.peek_kind() {
                    Some(T::Assign) => {
                        state.bump();
                        self.skip_trivia(state);
                        PrattParser::parse(state, 0, self);
                        state.finish_at(cp, E::AssignmentStatement);
                    }
                    Some(T::ColonAssign) => {
                        state.bump();
                        self.skip_trivia(state);
                        PrattParser::parse(state, 0, self);
                        state.finish_at(cp, E::ShortVarDecl);
                    }
                    _ => {
                        // 纯表达式语句
                    }
                }
                self.skip_trivia(state);
                state.eat(T::Semicolon);
            }
        }
        self.skip_trivia(state);
        Ok(())
    }

    fn parse_package_clause<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::{GoElementType as E, GoTokenType as T};
        let cp = state.checkpoint();
        state.expect(T::Package).ok();
        self.skip_trivia(state);
        state.expect(T::Identifier).ok();
        state.finish_at(cp, E::PackageClause);
        Ok(())
    }

    fn parse_import_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::{GoElementType as E, GoTokenType as T};
        let cp = state.checkpoint();
        state.expect(T::Import).ok();
        if state.eat(T::LeftParen) {
            while state.not_at_end() && !state.at(T::RightParen) {
                state.advance();
            }
            state.expect(T::RightParen).ok();
        }
        else {
            state.advance();
        }
        state.finish_at(cp, E::ImportDeclaration);
        Ok(())
    }

    fn parse_function_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::{GoElementType as E, GoTokenType as T};
        let cp = state.checkpoint();
        state.expect(T::Func).ok();
        self.skip_trivia(state);

        // 解析函数名
        if state.at(T::Identifier) {
            state.bump();
            self.skip_trivia(state);
        }

        // 解析参数列表和返回值 (简单跳过直到 {)
        while state.not_at_end() && !state.at(T::LeftBrace) {
            state.bump();
            self.skip_trivia(state);
        }

        self.parse_block(state)?;
        state.finish_at(cp, E::FunctionDeclaration);
        Ok(())
    }

    fn parse_variable_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::{GoElementType as E, GoTokenType as T};
        let cp = state.checkpoint();
        state.expect(T::Var).ok();
        self.skip_trivia(state);

        while state.at(T::Identifier) {
            let vcp = state.checkpoint();
            state.bump(); // name
            self.skip_trivia(state);

            // 可选类型
            if state.at(T::Identifier) {
                state.bump();
                self.skip_trivia(state);
            }

            // 可选赋值
            if state.eat(T::Assign) {
                self.skip_trivia(state);
                PrattParser::parse(state, 0, self);
            }

            state.finish_at(vcp, E::VariableSpec);
            self.skip_trivia(state);

            if !state.eat(T::Comma) {
                break;
            }
            self.skip_trivia(state);
        }

        state.eat(T::Semicolon);
        state.finish_at(cp, E::VariableDeclaration);
        Ok(())
    }

    fn parse_const_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::{GoElementType as E, GoTokenType as T};
        let cp = state.checkpoint();
        state.expect(T::Const).ok();
        while state.at(T::Identifier) {
            state.bump();
            if !state.eat(T::Semicolon) {
                break;
            }
        }
        state.eat(T::Semicolon);
        state.finish_at(cp, E::ConstDeclaration);
        Ok(())
    }

    fn parse_type_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::{GoElementType as E, GoTokenType as T};
        let cp = state.checkpoint();
        state.expect(T::Type).ok();
        while state.at(T::Identifier) {
            state.bump();
            if !state.eat(T::Semicolon) {
                break;
            }
        }
        state.eat(T::Semicolon);
        state.finish_at(cp, E::TypeDeclaration);
        Ok(())
    }

    fn parse_if_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::{GoElementType as E, GoTokenType as T};
        let cp = state.checkpoint();
        state.expect(T::If).ok();
        self.skip_trivia(state);

        // 解析条件
        if !state.at(T::LeftBrace) {
            PrattParser::parse(state, 0, self);
            self.skip_trivia(state);
        }

        self.parse_block(state)?;
        self.skip_trivia(state);

        if state.eat(T::Else) {
            self.skip_trivia(state);
            if state.at(T::If) {
                self.parse_if_statement(state)?;
            }
            else {
                self.parse_block(state)?;
            }
            self.skip_trivia(state);
        }
        state.finish_at(cp, E::IfStatement);
        Ok(())
    }

    fn parse_for_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::{GoElementType as E, GoTokenType as T};
        let cp = state.checkpoint();
        state.expect(T::For).ok();
        self.skip_trivia(state);

        // 尝试解析 init; condition; post
        if !state.at(T::LeftBrace) {
            // 至少解析一个表达式/语句
            PrattParser::parse(state, 0, self);
            self.skip_trivia(state);

            if state.eat(T::Semicolon) {
                self.skip_trivia(state);
                // condition
                if !state.at(T::Semicolon) {
                    PrattParser::parse(state, 0, self);
                    self.skip_trivia(state);
                }
                state.expect(T::Semicolon).ok();
                self.skip_trivia(state);
                // post
                if !state.at(T::LeftBrace) {
                    PrattParser::parse(state, 0, self);
                    self.skip_trivia(state);
                }
            }
        }

        self.parse_block(state)?;
        self.skip_trivia(state);
        state.finish_at(cp, E::ForStatement);
        Ok(())
    }

    fn parse_switch_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::{GoElementType as E, GoTokenType as T};
        let cp = state.checkpoint();
        state.expect(T::Switch).ok();
        while state.not_at_end() && !state.at(T::LeftBrace) {
            state.advance();
        }
        self.parse_block(state)?;
        state.finish_at(cp, E::SwitchStatement);
        Ok(())
    }

    fn parse_return_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::{GoElementType as E, GoTokenType as T};
        let cp = state.checkpoint();
        state.expect(T::Return).ok();
        if !state.at(T::Semicolon) {
            PrattParser::parse(state, 0, self);
        }
        state.eat(T::Semicolon);
        state.finish_at(cp, E::ReturnStatement);
        Ok(())
    }

    fn parse_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::{GoElementType as E, GoTokenType as T};
        let cp = state.checkpoint();
        state.expect(T::LeftBrace).ok();
        self.skip_trivia(state);
        while state.not_at_end() && !state.at(T::RightBrace) {
            self.parse_statement(state)?;
            self.skip_trivia(state);
        }
        state.expect(T::RightBrace).ok();
        state.finish_at(cp, E::Block);
        Ok(())
    }
}

impl<'config> Pratt<GoLanguage> for GoParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, GoLanguage> {
        use crate::{GoElementType as E, GoTokenType as T};
        self.skip_trivia(state);
        let cp = state.checkpoint();
        let kind = match state.peek_kind() {
            Some(k) => k,
            None => {
                state.bump();
                return state.finish_at(cp, E::Error);
            }
        };

        match kind {
            T::Identifier => {
                state.bump();
                state.finish_at(cp, E::Identifier)
            }
            T::IntLiteral | T::FloatLiteral | T::StringLiteral | T::RuneLiteral | T::BoolLiteral | T::NilLiteral => {
                state.bump();
                state.finish_at(cp, kind.into())
            }
            T::LeftParen => {
                state.bump();
                self.skip_trivia(state);
                PrattParser::parse(state, 0, self);
                self.skip_trivia(state);
                state.expect(T::RightParen).ok();
                state.finish_at(cp, E::BinaryExpression) // 或者 ParenExpression
            }
            _ => {
                state.bump();
                state.finish_at(cp, E::Error)
            }
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, GoLanguage> {
        use crate::{GoElementType as E, GoTokenType as T};
        let kind = match state.peek_kind() {
            Some(k) => k,
            None => return self.primary(state),
        };

        let prec = match kind {
            T::Plus | T::Minus | T::LogicalNot | T::Caret | T::Star | T::Ampersand | T::Arrow => 7,
            _ => return self.primary(state),
        };

        unary(state, kind, prec, E::BinaryExpression.into(), |st, p| PrattParser::parse(st, p, self))
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, GoLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, GoLanguage>> {
        use crate::{GoElementType as E, GoTokenType as T};
        self.skip_trivia(state);
        let kind = state.peek_kind()?;

        if kind == T::LeftParen {
            let prec = 8;
            if prec < min_precedence {
                return None;
            }
            let cp = state.checkpoint_before(left);
            state.bump(); // (
            self.skip_trivia(state);

            let arg_cp = state.checkpoint();
            let mut has_args = false;
            while state.not_at_end() && !state.at(T::RightParen) {
                PrattParser::parse(state, 0, self);
                has_args = true;
                self.skip_trivia(state);
                if !state.eat(T::Comma) {
                    break;
                }
                self.skip_trivia(state);
            }
            if has_args {
                state.finish_at(arg_cp, E::ExpressionList);
            }

            state.expect(T::RightParen).ok();
            return Some(state.finish_at(cp, E::CallExpression));
        }

        let (prec, assoc) = match kind {
            T::Assign | T::ColonAssign => (1, Associativity::Right),
            T::LogicalOr => (2, Associativity::Left),
            T::LogicalAnd => (3, Associativity::Left),
            T::Equal | T::NotEqual | T::Less | T::LessEqual | T::Greater | T::GreaterEqual => (4, Associativity::Left),
            T::Plus | T::Minus | T::Pipe | T::Caret => (5, Associativity::Left),
            T::Star | T::Slash | T::Percent | T::LeftShift | T::RightShift | T::Ampersand | T::AmpersandCaret => (6, Associativity::Left),
            T::Dot => (8, Associativity::Left),
            _ => return None,
        };

        if prec < min_precedence {
            return None;
        }

        Some(binary(state, left, kind, prec, assoc, E::BinaryExpression.into(), |st, p| PrattParser::parse(st, p, self)))
    }
}

impl<'config> Parser<GoLanguage> for GoParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<GoLanguage>) -> ParseOutput<'a, GoLanguage> {
        let lexer = GoLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let cp = state.checkpoint();
            while state.not_at_end() {
                self.parse_statement(state)?
            }
            Ok(state.finish_at(cp, crate::parser::element_type::GoElementType::SourceFile))
        })
    }
}
