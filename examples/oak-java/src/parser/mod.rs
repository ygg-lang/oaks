use oak_core::TokenType;
pub mod element_type;

use crate::{
    language::JavaLanguage,
    lexer::{JavaLexer, token_type::JavaTokenType},
    parser::element_type::JavaElementType,
};
use oak_core::{
    GreenNode, OakError, TextEdit,
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
        use crate::lexer::token_type::JavaTokenType::*;
        self.skip_trivia(state);
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(Identifier) => {
                state.bump();
                state.finish_at(cp, JavaElementType::Identifier)
            }
            Some(IntegerLiteral) | Some(FloatingPointLiteral) | Some(BooleanLiteral) | Some(CharacterLiteral) | Some(StringLiteral) | Some(NullLiteral) => {
                state.bump();
                state.finish_at(cp, JavaElementType::LiteralExpression)
            }
            Some(LeftParen) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(RightParen).ok();
                state.finish_at(cp, JavaElementType::ParenthesizedExpression)
            }
            Some(New) => {
                state.bump(); // new
                self.skip_trivia(state);
                self.parse_type(state).ok();
                self.skip_trivia(state);
                if state.at(LeftBracket) {
                    // Array creation
                    while state.at(LeftBracket) {
                        state.bump();
                        self.skip_trivia(state);
                        if !state.at(RightBracket) {
                            PrattParser::parse(state, 0, self);
                        }
                        self.skip_trivia(state);
                        state.expect(RightBracket).ok();
                        self.skip_trivia(state)
                    }
                    state.finish_at(cp, JavaElementType::ArrayCreation)
                }
                else if state.at(LeftParen) {
                    // Object creation
                    state.bump();
                    self.skip_trivia(state);
                    while state.not_at_end() && !state.at(RightParen) {
                        PrattParser::parse(state, 0, self);
                        self.skip_trivia(state);
                        if !state.eat(Comma) {
                            break;
                        }
                        self.skip_trivia(state)
                    }
                    state.expect(RightParen).ok();
                    state.finish_at(cp, JavaElementType::MethodCall) // Reuse MethodCall for constructor for now
                }
                else {
                    state.finish_at(cp, JavaElementType::Error)
                }
            }
            _ => {
                state.bump();
                state.finish_at(cp, JavaElementType::Error)
            }
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, JavaLanguage> {
        use crate::lexer::token_type::JavaTokenType::*;
        self.skip_trivia(state);
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(Bang) | Some(Tilde) | Some(Plus) | Some(Minus) | Some(PlusPlus) | Some(MinusMinus) => {
                state.bump();
                PrattParser::parse(state, 14, self); // Prefix operators have high precedence
                state.finish_at(cp, JavaElementType::UnaryExpression)
            }
            Some(LeftParen) => {
                // Try to parse as Cast: (Type) Expr
                // For now, a simple heuristic: if it looks like (Identifier), it might be a cast
                // But this is ambiguous with parenthesized expressions.
                // Standard Java parser looks ahead.
                // We'll try to parse a type inside.
                let snapshot = state.checkpoint();
                state.bump(); // (
                self.skip_trivia(state);
                if self.parse_type(state).is_ok() {
                    self.skip_trivia(state);
                    if state.eat(RightParen) {
                        self.skip_trivia(state);
                        // It's a cast if we have an expression following it
                        PrattParser::parse(state, 13, self); // Precedence for cast
                        state.finish_at(cp, JavaElementType::CastExpression)
                    }
                    else {
                        state.restore(snapshot);
                        self.primary(state)
                    }
                }
                else {
                    state.restore(snapshot);
                    self.primary(state)
                }
            }
            _ => self.primary(state),
        }
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, JavaLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, JavaLanguage>> {
        use crate::lexer::token_type::JavaTokenType::*;
        self.skip_trivia(state);
        let kind = state.peek_kind()?;
        eprintln!("DEBUG: Parser infix peeking {:?}", kind);

        let (prec, assoc) = match kind {
            Assign | PlusEquals | MinusEquals | AsteriskEquals | SlashEquals | PercentEquals | LeftShiftEquals | RightShiftEquals | UnsignedRightShiftEquals | AmpersandEquals | PipeEquals | CaretEquals => (1, Associativity::Right),
            Question => (2, Associativity::Right),
            PipePipe => (3, Associativity::Left),
            AmpersandAmpersand => (4, Associativity::Left),
            Pipe => (5, Associativity::Left),
            Caret => (6, Associativity::Left),
            Ampersand => (7, Associativity::Left),
            Equals | BangEquals => (8, Associativity::Left),
            LessThan | GreaterThan | LessThanEquals | GreaterThanEquals | Instanceof => (9, Associativity::Left),
            LeftShift | RightShift | UnsignedRightShift => (10, Associativity::Left),
            Plus | Minus => (11, Associativity::Left),
            Asterisk | Slash | Percent => (12, Associativity::Left),
            PlusPlus | MinusMinus => (15, Associativity::Left), // Postfix
            LeftParen | Dot | LeftBracket => (16, Associativity::Left),
            _ => return None,
        };

        if prec < min_precedence {
            return None;
        }

        match kind {
            PlusPlus | MinusMinus => {
                let cp = state.checkpoint_before(left);
                state.bump();
                Some(state.finish_at(cp, JavaElementType::PostfixExpression))
            }
            Assign | PlusEquals | MinusEquals | AsteriskEquals | SlashEquals | PercentEquals | LeftShiftEquals | RightShiftEquals | UnsignedRightShiftEquals | AmpersandEquals | PipeEquals | CaretEquals => {
                Some(binary(state, left, kind, prec, assoc, JavaElementType::AssignmentExpression.into(), |s, p| PrattParser::parse(s, p, self)))
            }
            Question => {
                let cp = state.checkpoint_before(left);
                state.bump(); // ?
                self.skip_trivia(state);
                PrattParser::parse(state, 0, self); // then branch
                self.skip_trivia(state);
                state.expect(Colon).ok(); // :
                self.skip_trivia(state);
                PrattParser::parse(state, prec, self); // else branch (right assoc)
                Some(state.finish_at(cp, JavaElementType::TernaryExpression))
            }
            LeftParen => {
                let cp = state.checkpoint_before(left);
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
                Some(state.finish_at(cp, JavaElementType::MethodCall))
            }
            Dot => {
                let cp = state.checkpoint_before(left);
                state.expect(Dot).ok();
                self.skip_trivia(state);
                state.expect(Identifier).ok();
                Some(state.finish_at(cp, JavaElementType::MemberSelect))
            }
            LeftBracket => {
                let cp = state.checkpoint_before(left);
                state.expect(LeftBracket).ok();
                self.skip_trivia(state);
                PrattParser::parse(state, 0, self);
                self.skip_trivia(state);
                state.expect(RightBracket).ok();
                Some(state.finish_at(cp, JavaElementType::ArrayAccess))
            }
            _ => Some(binary(state, left, kind, prec, assoc, JavaElementType::BinaryExpression.into(), |s, p| PrattParser::parse(s, p, self))),
        }
    }
}

impl<'config> JavaParser<'config> {
    pub fn new(config: &'config JavaLanguage) -> Self {
        Self { config }
    }

    fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::JavaTokenType::*;
        self.skip_trivia(state);
        let cp = state.checkpoint();
        let pk = state.peek_kind();
        match pk {
            Some(Public) | Some(Private) | Some(Protected) | Some(Static) | Some(Final) | Some(Abstract) | Some(Class) | Some(Interface) | Some(Enum) | Some(Struct) | Some(Record) => self.parse_declaration(state)?,
            Some(Int) | Some(Boolean) | Some(Void) | Some(Long) | Some(Float) | Some(Double) | Some(Char) | Some(Byte) | Some(Short) => {
                self.parse_variable_declaration(state)?;
                state.finish_at(cp, JavaElementType::VariableDeclaration);
            }
            Some(Identifier) => {
                // Could be a type name for a variable declaration, or an expression
                let snapshot = state.checkpoint();
                if self.parse_type(state).is_ok() {
                    self.skip_trivia(state);
                    if state.at(Identifier) {
                        state.restore(snapshot);
                        self.parse_variable_declaration(state)?;
                        state.finish_at(cp, JavaElementType::VariableDeclaration);
                    }
                    else {
                        state.restore(snapshot);
                        PrattParser::parse(state, 0, self);
                        self.skip_trivia(state);
                        state.eat(Semicolon);
                        state.finish_at(cp, JavaElementType::ExpressionStatement);
                    }
                }
                else {
                    state.restore(snapshot);
                    PrattParser::parse(state, 0, self);
                    self.skip_trivia(state);
                    state.eat(Semicolon);
                    state.finish_at(cp, JavaElementType::ExpressionStatement);
                }
            }
            Some(If) => {
                self.parse_if_statement(state)?;
                state.finish_at(cp, JavaElementType::IfStatement);
            }
            Some(While) => {
                self.parse_while_statement(state)?;
                state.finish_at(cp, JavaElementType::WhileStatement);
            }
            Some(Do) => {
                self.parse_do_while_statement(state)?;
                state.finish_at(cp, JavaElementType::DoWhileStatement);
            }
            Some(For) => {
                self.parse_for_statement(state)?;
                state.finish_at(cp, JavaElementType::ForStatement);
            }
            Some(Switch) => {
                self.parse_switch_statement(state)?;
                state.finish_at(cp, JavaElementType::SwitchStatement);
            }
            Some(Return) => {
                self.parse_return_statement(state)?;
                state.finish_at(cp, JavaElementType::ReturnStatement);
            }
            Some(Break) => {
                state.bump(); // break
                state.eat(Semicolon);
                state.finish_at(cp, JavaElementType::Break);
            }
            Some(Continue) => {
                state.bump(); // continue
                state.eat(Semicolon);
                state.finish_at(cp, JavaElementType::Continue);
            }
            Some(LeftBrace) => {
                self.parse_block_statement(state)?;
            }
            Some(Try) => {
                state.bump(); // try
                self.parse_block_statement(state)?;
                self.skip_trivia(state);
                while state.at(Catch) {
                    let c_cp = state.checkpoint();
                    state.bump(); // catch
                    self.skip_trivia(state);
                    state.expect(LeftParen).ok();
                    self.skip_trivia(state);
                    // Catch parameter: Type Name
                    let p_cp = state.checkpoint();
                    self.parse_type(state).ok();
                    self.skip_trivia(state);
                    state.expect(Identifier).ok();
                    state.finish_at(p_cp, JavaElementType::Parameter);
                    self.skip_trivia(state);
                    state.expect(RightParen).ok();
                    self.skip_trivia(state);
                    self.parse_block_statement(state)?;
                    state.finish_at(c_cp, JavaElementType::CatchClause);
                    self.skip_trivia(state);
                }
                if state.eat(Finally) {
                    self.skip_trivia(state);
                    self.parse_block_statement(state)?;
                }
                state.finish_at(cp, JavaElementType::TryStatement);
            }
            Some(Throw) => {
                state.bump(); // throw
                self.skip_trivia(state);
                PrattParser::parse(state, 0, self);
                self.skip_trivia(state);
                state.eat(Semicolon);
                state.finish_at(cp, JavaElementType::ThrowStatement);
            }
            Some(Package) => {
                self.parse_package_declaration(state)?;
            }
            Some(Import) => {
                self.parse_import_declaration(state)?;
            }
            _ => {
                PrattParser::parse(state, 0, self);
                self.skip_trivia(state);
                state.eat(Semicolon);
                state.finish_at(cp, JavaElementType::ExpressionStatement);
            }
        }
        Ok(())
    }

    fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        while let Some(kind) = state.peek_kind() {
            if kind.is_ignored() { state.bump() } else { break }
        }
    }

    fn parse_type<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::JavaTokenType::*;
        self.skip_trivia(state);
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(Identifier) | Some(Int) | Some(Boolean) | Some(Void) | Some(Long) | Some(Float) | Some(Double) | Some(Char) | Some(Byte) | Some(Short) => state.bump(),
            _ => {
                // Return error?
            }
        }
        self.skip_trivia(state);
        while state.at(Dot) {
            state.bump();
            self.skip_trivia(state);
            state.expect(Identifier).ok();
            self.skip_trivia(state)
        }
        while state.at(LeftBracket) {
            state.bump();
            self.skip_trivia(state);
            state.expect(RightBracket).ok();
            self.skip_trivia(state)
        }
        state.finish_at(cp, JavaElementType::Identifier);
        Ok(())
    }

    fn parse_package_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::JavaTokenType::*;
        let cp = state.checkpoint();
        state.expect(Package).ok();
        self.skip_trivia(state);
        while state.not_at_end() && !state.at(Semicolon) {
            if state.at(Identifier) || state.at(Dot) {
                state.bump()
            }
            else {
                break;
            }
            self.skip_trivia(state)
        }
        state.eat(Semicolon);
        state.finish_at(cp, JavaElementType::Package);
        Ok(())
    }

    fn parse_import_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::JavaTokenType::*;
        let cp = state.checkpoint();
        state.expect(Import).ok();
        self.skip_trivia(state);
        if state.eat(Static) {
            self.skip_trivia(state)
        }
        while state.not_at_end() && !state.at(Semicolon) {
            if state.at(Identifier) || state.at(Dot) || state.at(Asterisk) {
                state.bump()
            }
            else {
                break;
            }
            self.skip_trivia(state)
        }
        state.eat(Semicolon);
        state.finish_at(cp, JavaElementType::Import);
        Ok(())
    }

    fn parse_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::JavaTokenType::*;
        let cp = state.checkpoint();
        self.skip_trivia(state);
        // 处理修饰符
        while state.not_at_end() && matches!(state.peek_kind(), Some(Public) | Some(Private) | Some(Protected) | Some(Static) | Some(Final) | Some(Abstract)) {
            state.bump();
            self.skip_trivia(state)
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
                    self.skip_trivia(state)
                }
                if state.eat(Implements) {
                    self.skip_trivia(state);
                    while state.not_at_end() && !state.at(LeftBrace) {
                        state.bump();
                        self.skip_trivia(state)
                    }
                }
                self.parse_block_statement(state)?;
                state.finish_at(cp, JavaElementType::ClassDeclaration);
            }
            Some(Interface) => {
                state.expect(Interface).ok();
                self.skip_trivia(state);
                state.expect(Identifier).ok();
                self.skip_trivia(state);
                self.parse_block_statement(state)?;
                state.finish_at(cp, JavaElementType::InterfaceDeclaration);
            }
            Some(Enum) => {
                state.expect(Enum).ok();
                self.skip_trivia(state);
                state.expect(Identifier).ok();
                self.skip_trivia(state);
                self.parse_block_statement(state)?;
                state.finish_at(cp, JavaElementType::EnumDeclaration);
            }
            Some(Struct) => {
                state.expect(Struct).ok();
                self.skip_trivia(state);
                state.expect(Identifier).ok();
                self.skip_trivia(state);
                self.parse_block_statement(state)?;
                state.finish_at(cp, JavaElementType::StructDeclaration);
            }
            Some(Record) => {
                state.expect(Record).ok();
                self.skip_trivia(state);
                state.expect(Identifier).ok();
                self.skip_trivia(state);
                self.parse_block_statement(state)?;
                state.finish_at(cp, JavaElementType::RecordDeclaration);
            }
            _ => {
                // 可能是方法或字段
                // 此时已经消耗了修饰符，当前应该是类型
                self.parse_type(state).ok();
                self.skip_trivia(state);
                // 消耗名称并包装为 Identifier 节点
                let name_cp = state.checkpoint();
                state.expect(Identifier).ok();
                state.finish_at(name_cp, JavaElementType::Identifier);
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
                        let pn_cp = state.checkpoint();
                        state.expect(Identifier).ok(); // Name
                        state.finish_at(pn_cp, JavaElementType::Identifier);
                        self.skip_trivia(state);
                        // 处理数组类型 []
                        while state.at(LeftBracket) {
                            state.bump();
                            self.skip_trivia(state);
                            state.expect(RightBracket).ok();
                            self.skip_trivia(state)
                        }
                        state.finish_at(p_cp, JavaElementType::Parameter);
                        if !state.eat(Comma) {
                            break;
                        }
                        self.skip_trivia(state)
                    }
                    state.expect(RightParen).ok();
                    self.skip_trivia(state);
                    if state.eat(Throws) {
                        self.skip_trivia(state);
                        while state.not_at_end() && !state.at(LeftBrace) && !state.at(Semicolon) {
                            let t_cp = state.checkpoint();
                            state.expect(Identifier).ok();
                            state.finish_at(t_cp, JavaElementType::Identifier);
                            self.skip_trivia(state);
                            if !state.eat(Comma) {
                                break;
                            }
                            self.skip_trivia(state)
                        }
                    }
                    self.skip_trivia(state);
                    if state.at(LeftBrace) {
                        self.parse_block_statement(state)?
                    }
                    else {
                        state.eat(Semicolon);
                    }
                    state.finish_at(cp, JavaElementType::MethodDeclaration);
                }
                else {
                    // 字段声明
                    if state.eat(Assign) {
                        self.skip_trivia(state);
                        PrattParser::parse(state, 0, self);
                    }
                    self.skip_trivia(state);
                    state.eat(Semicolon);
                    state.finish_at(cp, JavaElementType::FieldDeclaration);
                }
            }
        }
        Ok(())
    }

    fn _parse_class_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::JavaTokenType::*;
        let cp = state.checkpoint();
        state.expect(Class).ok();
        state.expect(Identifier).ok();
        if state.eat(Extends) {
            state.expect(Identifier).ok();
        }
        if state.eat(Implements) {
            while state.not_at_end() && !state.at(LeftBrace) {
                state.bump()
            }
        }
        self.parse_block_statement(state)?;
        state.finish_at(cp, JavaElementType::ClassDeclaration);
        Ok(())
    }

    fn _parse_interface_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::JavaTokenType::*;
        state.expect(Interface).ok();
        state.expect(Identifier).ok();
        self.parse_block_statement(state)?;
        Ok(())
    }

    fn _parse_enum_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::JavaTokenType::*;
        state.expect(Enum).ok();
        state.expect(Identifier).ok();
        self.parse_block_statement(state)?;
        Ok(())
    }

    fn parse_if_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::JavaTokenType::*;
        state.bump(); // if
        self.skip_trivia(state);
        state.expect(LeftParen).ok();
        self.skip_trivia(state);
        PrattParser::parse(state, 0, self);
        self.skip_trivia(state);
        state.expect(RightParen).ok();
        self.skip_trivia(state);
        self.parse_statement(state)?;
        self.skip_trivia(state);
        if state.eat(Else) {
            self.skip_trivia(state);
            self.parse_statement(state)?;
        }
        Ok(())
    }

    fn parse_while_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::JavaTokenType::*;
        state.bump(); // while
        self.skip_trivia(state);
        state.expect(LeftParen).ok();
        self.skip_trivia(state);
        PrattParser::parse(state, 0, self);
        self.skip_trivia(state);
        state.expect(RightParen).ok();
        self.skip_trivia(state);
        self.parse_statement(state)?;
        Ok(())
    }

    fn parse_do_while_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::JavaTokenType::*;
        state.bump(); // do
        self.skip_trivia(state);
        self.parse_statement(state)?;
        self.skip_trivia(state);
        state.expect(While).ok();
        self.skip_trivia(state);
        state.expect(LeftParen).ok();
        self.skip_trivia(state);
        PrattParser::parse(state, 0, self);
        self.skip_trivia(state);
        state.expect(RightParen).ok();
        self.skip_trivia(state);
        state.eat(Semicolon);
        Ok(())
    }

    fn parse_for_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::JavaTokenType::*;
        state.bump(); // for
        state.expect(LeftParen).ok();
        self.skip_trivia(state);

        // 1. Init
        if !state.at(Semicolon) {
            let cp = state.checkpoint();
            let pk = state.peek_kind();
            match pk {
                Some(Int) | Some(Boolean) | Some(Void) | Some(Long) | Some(Float) | Some(Double) | Some(Char) | Some(Byte) | Some(Short) => {
                    self.parse_variable_declaration(state)?;
                    state.finish_at(cp, JavaElementType::VariableDeclaration);
                }
                Some(Identifier) => {
                    let snapshot = state.checkpoint();
                    if self.parse_type(state).is_ok() && state.at(Identifier) {
                        state.restore(snapshot);
                        self.parse_variable_declaration(state)?;
                        state.finish_at(cp, JavaElementType::VariableDeclaration);
                    }
                    else {
                        state.restore(snapshot);
                        PrattParser::parse(state, 0, self);
                    }
                }
                _ => {
                    PrattParser::parse(state, 0, self);
                }
            }
        }
        state.expect(Semicolon).ok();
        self.skip_trivia(state);

        // 2. Condition
        if !state.at(Semicolon) {
            PrattParser::parse(state, 0, self);
        }
        state.expect(Semicolon).ok();
        self.skip_trivia(state);

        // 3. Update
        if !state.at(RightParen) {
            PrattParser::parse(state, 0, self);
        }
        state.expect(RightParen).ok();
        self.skip_trivia(state);

        // 4. Body
        self.parse_statement(state)?;
        Ok(())
    }

    fn parse_variable_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::JavaTokenType::*;
        self.parse_type(state)?;
        self.skip_trivia(state);
        let cp = state.checkpoint();
        state.expect(Identifier).ok();
        state.finish_at(cp, JavaElementType::Identifier);
        self.skip_trivia(state);
        if state.eat(Assign) {
            self.skip_trivia(state);
            PrattParser::parse(state, 0, self);
        }
        self.skip_trivia(state);
        state.eat(Semicolon);
        Ok(())
    }

    fn parse_switch_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::JavaTokenType::*;
        state.bump(); // switch
        self.skip_trivia(state);
        state.expect(LeftParen).ok();
        self.skip_trivia(state);
        PrattParser::parse(state, 0, self);
        self.skip_trivia(state);
        state.expect(RightParen).ok();
        self.skip_trivia(state);
        state.expect(LeftBrace).ok();
        self.skip_trivia(state);
        while state.not_at_end() && !state.at(RightBrace) {
            self.skip_trivia(state);
            let cp = state.checkpoint();
            if state.eat(Case) {
                self.skip_trivia(state);
                PrattParser::parse(state, 0, self);
                self.skip_trivia(state);
                state.expect(Colon).ok();
                self.skip_trivia(state);
                while state.not_at_end() && !state.at(Case) && !state.at(Default) && !state.at(RightBrace) {
                    self.parse_statement(state).ok();
                    self.skip_trivia(state);
                }
                state.finish_at(cp, JavaElementType::SwitchCase);
            }
            else if state.eat(Default) {
                self.skip_trivia(state);
                state.expect(Colon).ok();
                self.skip_trivia(state);
                while state.not_at_end() && !state.at(Case) && !state.at(Default) && !state.at(RightBrace) {
                    self.parse_statement(state).ok();
                    self.skip_trivia(state)
                }
                state.finish_at(cp, JavaElementType::DefaultCase);
            }
            else {
                state.bump(); // error recovery
                self.skip_trivia(state)
            }
        }
        state.expect(RightBrace).ok();
        Ok(())
    }

    fn parse_block_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(JavaTokenType::LeftBrace).ok();
        while state.not_at_end() && !state.at(JavaTokenType::RightBrace) {
            self.skip_trivia(state);
            if state.at(JavaTokenType::RightBrace) {
                break;
            }
            self.parse_statement(state).ok();
            self.skip_trivia(state)
        }
        state.expect(JavaTokenType::RightBrace).ok();
        state.finish_at(cp, JavaElementType::BlockStatement);
        Ok(())
    }

    fn parse_return_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.bump(); // return
        if !state.at(JavaTokenType::Semicolon) && !state.at(JavaTokenType::RightBrace) {
            PrattParser::parse(state, 0, self);
        }
        state.eat(JavaTokenType::Semicolon);
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
        Ok(state.finish_at(checkpoint, JavaElementType::CompilationUnit))
    }
}

impl<'config> Parser<JavaLanguage> for JavaParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<JavaLanguage>) -> oak_core::parser::ParseOutput<'a, JavaLanguage> {
        let lexer = JavaLexer::new(self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root(state))
    }
}
