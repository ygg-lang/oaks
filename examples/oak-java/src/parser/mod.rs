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
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(Identifier) => {
                state.bump();
                state.finish_at(cp, Identifier.into())
            }
            Some(IntegerLiteral) | Some(FloatingPointLiteral) | Some(BooleanLiteral) | Some(CharacterLiteral) | Some(StringLiteral) | Some(NullLiteral) => {
                state.bump();
                state.finish_at(cp, CompilationUnit.into()) // 简化处理
            }
            Some(LeftParen) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(RightParen).ok();
                state.finish_at(cp, CompilationUnit.into())
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
        let kind = state.peek_kind()?;

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
                while state.not_at_end() && !state.at(RightParen) {
                    state.advance();
                }
                state.expect(RightParen).ok();
                Some(state.finish_at(cp, CompilationUnit.into()))
            }
            Dot => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(Dot).ok();
                state.expect(Identifier).ok();
                Some(state.finish_at(cp, CompilationUnit.into()))
            }
            _ => Some(binary(state, left, kind, prec, assoc, CompilationUnit.into(), |s, p| PrattParser::parse(s, p, self))),
        }
    }
}

impl<'config> JavaParser<'config> {
    pub fn new(config: &'config JavaLanguage) -> Self {
        Self { config }
    }

    fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::JavaSyntaxKind::*;
        match state.peek_kind() {
            Some(Public) | Some(Private) | Some(Protected) | Some(Static) | Some(Final) | Some(Class) | Some(Interface) | Some(Enum) => self.parse_declaration(state)?,
            Some(If) => self.parse_if_statement(state)?,
            Some(While) => self.parse_while_statement(state)?,
            Some(For) => self.parse_for_statement(state)?,
            Some(Return) => self.parse_return_statement(state)?,
            Some(LeftBrace) => self.parse_block_statement(state)?,
            Some(Package) => self.parse_package_declaration(state)?,
            Some(Import) => self.parse_import_declaration(state)?,
            _ => {
                PrattParser::parse(state, 0, self);
                state.eat(Semicolon);
            }
        }
        Ok(())
    }

    fn parse_package_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::JavaSyntaxKind::*;
        state.expect(Package).ok();
        state.advance_until(Semicolon);
        state.eat(Semicolon);
        Ok(())
    }

    fn parse_import_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::JavaSyntaxKind::*;
        state.expect(Import).ok();
        state.advance_until(Semicolon);
        state.eat(Semicolon);
        Ok(())
    }

    fn parse_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::JavaSyntaxKind::*;
        // 处理修饰符
        while state.not_at_end() && matches!(state.peek_kind(), Some(Public) | Some(Private) | Some(Protected) | Some(Static) | Some(Final) | Some(Abstract)) {
            state.bump();
        }

        match state.peek_kind() {
            Some(Class) => self.parse_class_declaration(state)?,
            Some(Interface) => self.parse_interface_declaration(state)?,
            Some(Enum) => self.parse_enum_declaration(state)?,
            _ => {
                // 可能是方法或字段
                state.advance_until(Semicolon);
                state.eat(Semicolon);
            }
        }
        Ok(())
    }

    fn parse_class_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::JavaSyntaxKind::*;
        state.expect(Class).ok();
        state.expect(Identifier).ok();
        if state.eat(Extends) {
            state.expect(Identifier).ok();
        }
        if state.eat(Implements) {
            while state.not_at_end() && !state.at(LeftBrace) {
                state.advance();
            }
        }
        self.parse_block_statement(state)?;
        Ok(())
    }

    fn parse_interface_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::JavaSyntaxKind::*;
        state.expect(Interface).ok();
        state.expect(Identifier).ok();
        self.parse_block_statement(state)?;
        Ok(())
    }

    fn parse_enum_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
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
        state.expect(crate::kind::JavaSyntaxKind::LeftBrace).ok();
        while state.not_at_end() && !state.at(crate::kind::JavaSyntaxKind::RightBrace) {
            self.parse_statement(state)?;
        }
        state.expect(crate::kind::JavaSyntaxKind::RightBrace).ok();
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
}

impl<'config> Parser<JavaLanguage> for JavaParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<JavaLanguage>) -> oak_core::ParseOutput<'a, JavaLanguage> {
        let lexer = JavaLexer::new(self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            while state.not_at_end() {
                if state.current().map(|t| t.kind.is_ignored()).unwrap_or(false) {
                    state.advance();
                    continue;
                }
                self.parse_statement(state).ok();
            }
            Ok(state.finish_at(checkpoint, JavaSyntaxKind::CompilationUnit.into()))
        })
    }
}
