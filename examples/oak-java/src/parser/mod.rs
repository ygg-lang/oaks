use crate::{kind::JavaSyntaxKind, language::JavaLanguage, lexer::JavaLexer};
use oak_core::{
    GreenNode, OakError, TextEdit, TokenType,
    parser::{
        ParseCache, Parser, ParserState,
        pratt::{Associativity, Pratt, PrattParser, binary},
    },
    source::Source,
};

pub(crate) type State<'a, S> = ParserState<'a, JavaLanguage, S>;

pub struct JavaParser<'config> {
    pub(crate) config: &'config JavaLanguage,
}

impl<'config> Pratt<JavaLanguage> for JavaParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, JavaLanguage> {
        use crate::kind::JavaSyntaxKind::*;
        self.skip_trivia(state);
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(Identifier) => {
                state.bump();
                state.finish_at(cp, Identifier.into())
            }
            Some(IntegerLiteral) | Some(FloatingPointLiteral) | Some(BooleanLiteral) | Some(CharacterLiteral) | Some(StringLiteral) | Some(NullLiteral) => {
                state.bump();
                state.finish_at(cp, LiteralExpression.into())
            }
            Some(LeftParen) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(RightParen).ok();
                state.finish_at(cp, CompilationUnit.into()) // 括号表达式暂且如此
            }
            _ => {
                state.bump();
                state.finish_at(cp, Error.into())
            }
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, JavaLanguage> {
        self.primary(state)
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, JavaLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, JavaLanguage>> {
        use crate::kind::JavaSyntaxKind::*;
        self.skip_trivia(state);
        let kind = state.peek_kind()?;
        eprintln!("DEBUG: Parser infix peeking {:?}", kind);

        let (prec, assoc) = match kind {
            Assign | PlusEquals | MinusEquals | AsteriskEquals | SlashEquals | PercentEquals | LeftShiftEquals | RightShiftEquals | UnsignedRightShiftEquals | AmpersandEquals | PipeEquals | CaretEquals => (1, Associativity::Right),
            PipePipe => (2, Associativity::Left),
            AmpersandAmpersand => (3, Associativity::Left),
            Equals | BangEquals | LessThan | GreaterThan | LessThanEquals | GreaterThanEquals => (4, Associativity::Left),
            Plus | Minus => (10, Associativity::Left),
            Asterisk | Slash | Percent => (11, Associativity::Left),
            LeftParen | Dot => (15, Associativity::Left),
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
                self.skip_trivia(state);
                while state.not_at_end() && !state.at(RightParen) {
                    PrattParser::parse(state, 0, self);
                    self.skip_trivia(state);
                    if state.eat(Comma) {
                        self.skip_trivia(state);
                        continue;
                    }
                }
                state.expect(RightParen).ok();
                Some(state.finish_at(cp, MethodCall.into()))
            }
            Dot => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(Dot).ok();
                self.skip_trivia(state);
                state.expect(Identifier).ok();
                Some(state.finish_at(cp, MemberSelect.into()))
            }
            _ => Some(binary(state, left, kind, prec, assoc, BinaryExpression.into(), |s, p| PrattParser::parse(s, p, self))),
        }
    }
}

impl<'config> JavaParser<'config> {
    pub fn new(config: &'config JavaLanguage) -> Self {
        Self { config }
    }

    fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::JavaSyntaxKind::*;
        self.skip_trivia(state);
        let cp = state.checkpoint();
        let pk = state.peek_kind();
        match pk {
            Some(Public) | Some(Private) | Some(Protected) | Some(Static) | Some(Final) | Some(Class) | Some(Interface) | Some(Enum) => {
                self.parse_declaration(state)?;
            }
            Some(If) => {
                self.parse_if_statement(state)?;
                state.finish_at(cp, IfStatement.into());
            }
            Some(While) => {
                self.parse_while_statement(state)?;
                state.finish_at(cp, WhileStatement.into());
            }
            Some(For) => {
                self.parse_for_statement(state)?;
                state.finish_at(cp, ForStatement.into());
            }
            Some(Return) => {
                self.parse_return_statement(state)?;
                state.finish_at(cp, ReturnStatement.into());
            }
            Some(LeftBrace) => {
                self.parse_block_statement(state)?;
            }
            Some(Package) => self.parse_package_declaration(state)?,
            Some(Import) => self.parse_import_declaration(state)?,
            _ => {
                PrattParser::parse(state, 0, self);
                self.skip_trivia(state);
                state.eat(Semicolon);
                state.finish_at(cp, ExpressionStatement.into());
            }
        }
        Ok(())
    }

    fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        while let Some(kind) = state.peek_kind() {
            if kind.is_ignored() {
                state.bump();
            }
            else {
                break;
            }
        }
    }

    fn parse_type<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::JavaSyntaxKind::*;
        self.skip_trivia(state);
        match state.peek_kind() {
            Some(Identifier) | Some(Int) | Some(Boolean) | Some(Void) | Some(Long) | Some(Float) | Some(Double) | Some(Char) | Some(Byte) | Some(Short) => {
                state.bump();
            }
            _ => {
                // Return error?
            }
        }
        self.skip_trivia(state);
        while state.at(Dot) {
            state.bump();
            self.skip_trivia(state);
            state.expect(Identifier).ok();
            self.skip_trivia(state);
        }
        while state.at(LeftBracket) {
            state.bump();
            self.skip_trivia(state);
            state.expect(RightBracket).ok();
            self.skip_trivia(state);
        }
        Ok(())
    }

    fn parse_package_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::JavaSyntaxKind::*;
        state.expect(Package).ok();
        self.skip_trivia(state);
        state.advance_until(Semicolon);
        state.eat(Semicolon);
        Ok(())
    }

    fn parse_import_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::JavaSyntaxKind::*;
        state.expect(Import).ok();
        self.skip_trivia(state);
        state.advance_until(Semicolon);
        state.eat(Semicolon);
        Ok(())
    }

    fn parse_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::JavaSyntaxKind::*;
        let cp = state.checkpoint();
        self.skip_trivia(state);
        // 处理修饰符
        while state.not_at_end() && matches!(state.peek_kind(), Some(Public) | Some(Private) | Some(Protected) | Some(Static) | Some(Final) | Some(Abstract)) {
            state.bump();
            self.skip_trivia(state);
        }

        match state.peek_kind() {
            Some(Class) => {
                state.expect(Class).ok();
                self.skip_trivia(state);
                state.expect(Identifier).ok();
                self.skip_trivia(state);
                if state.eat(Extends) {
                    self.skip_trivia(state);
                    state.expect(Identifier).ok();
                    self.skip_trivia(state);
                }
                if state.eat(Implements) {
                    self.skip_trivia(state);
                    while state.not_at_end() && !state.at(LeftBrace) {
                        state.bump();
                        self.skip_trivia(state);
                    }
                }
                self.parse_block_statement(state)?;
                state.finish_at(cp, ClassDeclaration.into());
            }
            Some(Interface) => {
                state.expect(Interface).ok();
                self.skip_trivia(state);
                state.expect(Identifier).ok();
                self.skip_trivia(state);
                self.parse_block_statement(state)?;
                state.finish_at(cp, InterfaceDeclaration.into());
            }
            Some(Enum) => {
                state.expect(Enum).ok();
                self.skip_trivia(state);
                state.expect(Identifier).ok();
                self.skip_trivia(state);
                self.parse_block_statement(state)?;
                state.finish_at(cp, ClassDeclaration.into()); // 暂且用 ClassDeclaration
            }
            _ => {
                // 可能是方法或字段
                // 此时已经消耗了修饰符，当前应该是类型
                self.parse_type(state).ok();
                self.skip_trivia(state);
                state.expect(Identifier).ok(); // 消耗名称
                self.skip_trivia(state);

                if state.at(LeftParen) {
                    // 方法声明
                    state.bump(); // (
                    self.skip_trivia(state);
                    while state.not_at_end() && !state.at(RightParen) {
                        // 简单的参数解析：Type Name
                        let p_cp = state.checkpoint();
                        self.parse_type(state).ok();
                        self.skip_trivia(state);
                        state.expect(Identifier).ok(); // Name
                        self.skip_trivia(state);
                        // 处理数组类型 []
                        while state.at(LeftBracket) {
                            state.bump();
                            self.skip_trivia(state);
                            state.expect(RightBracket).ok();
                            self.skip_trivia(state);
                        }
                        state.finish_at(p_cp, Parameter.into());
                        if !state.eat(Comma) {
                            break;
                        }
                        self.skip_trivia(state);
                    }
                    state.expect(RightParen).ok();
                    self.skip_trivia(state);
                    if state.at(LeftBrace) {
                        self.parse_block_statement(state)?;
                    }
                    else {
                        state.eat(Semicolon);
                    }
                    state.finish_at(cp, MethodDeclaration.into());
                }
                else {
                    // 字段声明
                    if state.eat(Assign) {
                        self.skip_trivia(state);
                        PrattParser::parse(state, 0, self);
                    }
                    self.skip_trivia(state);
                    state.eat(Semicolon);
                    state.finish_at(cp, FieldDeclaration.into());
                }
            }
        }
        Ok(())
    }

    fn _parse_class_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::JavaSyntaxKind::*;
        let cp = state.checkpoint();
        state.expect(Class).ok();
        state.expect(Identifier).ok();
        if state.eat(Extends) {
            state.expect(Identifier).ok();
        }
        if state.eat(Implements) {
            while state.not_at_end() && !state.at(LeftBrace) {
                state.bump();
            }
        }
        self.parse_block_statement(state)?;
        state.finish_at(cp, ClassDeclaration.into());
        Ok(())
    }

    fn _parse_interface_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::JavaSyntaxKind::*;
        state.expect(Interface).ok();
        state.expect(Identifier).ok();
        self.parse_block_statement(state)?;
        Ok(())
    }

    fn _parse_enum_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::JavaSyntaxKind::*;
        state.expect(Enum).ok();
        state.expect(Identifier).ok();
        self.parse_block_statement(state)?;
        Ok(())
    }

    fn parse_if_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.bump(); // if
        state.expect(crate::kind::JavaSyntaxKind::LeftParen).ok();
        PrattParser::parse(state, 0, self);
        state.expect(crate::kind::JavaSyntaxKind::RightParen).ok();
        self.parse_statement(state)?;
        if state.eat(crate::kind::JavaSyntaxKind::Else) {
            self.parse_statement(state)?;
        }
        Ok(())
    }

    fn parse_while_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.bump(); // while
        state.expect(crate::kind::JavaSyntaxKind::LeftParen).ok();
        PrattParser::parse(state, 0, self);
        state.expect(crate::kind::JavaSyntaxKind::RightParen).ok();
        self.parse_statement(state)?;
        Ok(())
    }

    fn parse_for_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.bump(); // for
        state.expect(crate::kind::JavaSyntaxKind::LeftParen).ok();
        PrattParser::parse(state, 0, self);
        state.expect(crate::kind::JavaSyntaxKind::RightParen).ok();
        self.parse_statement(state)?;
        Ok(())
    }

    fn parse_block_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(crate::kind::JavaSyntaxKind::LeftBrace).ok();
        while state.not_at_end() && !state.at(crate::kind::JavaSyntaxKind::RightBrace) {
            self.skip_trivia(state);
            if state.at(crate::kind::JavaSyntaxKind::RightBrace) {
                break;
            }
            self.parse_statement(state).ok();
            self.skip_trivia(state);
        }
        state.expect(crate::kind::JavaSyntaxKind::RightBrace).ok();
        state.finish_at(cp, crate::kind::JavaSyntaxKind::BlockStatement.into());
        Ok(())
    }

    fn parse_return_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.bump(); // return
        if !state.at(crate::kind::JavaSyntaxKind::Semicolon) && !state.at(crate::kind::JavaSyntaxKind::RightBrace) {
            PrattParser::parse(state, 0, self);
        }
        state.eat(crate::kind::JavaSyntaxKind::Semicolon);
        Ok(())
    }

    fn parse_item<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        self.parse_statement(state)
    }
}

impl<'config> JavaParser<'config> {
    fn parse_root<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, JavaLanguage>, OakError> {
        let checkpoint = state.checkpoint();
        while state.not_at_end() {
            self.parse_item(state).ok();
        }
        Ok(state.finish_at(checkpoint, JavaSyntaxKind::CompilationUnit.into()))
    }
}

impl<'config> Parser<JavaLanguage> for JavaParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<JavaLanguage>) -> oak_core::parser::ParseOutput<'a, JavaLanguage> {
        let lexer = JavaLexer::new(self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root(state))
    }
}
