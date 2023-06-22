use crate::{kind::TypeScriptSyntaxKind, language::TypeScriptLanguage, lexer::TypeScriptLexer};
use oak_core::{
    GreenNode, OakError, TextEdit, TokenType,
    parser::{
        ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer,
        pratt::{Associativity, Pratt, PrattParser, binary},
    },
    source::Source,
};

pub(crate) type State<'a, S> = ParserState<'a, TypeScriptLanguage, S>;

pub struct TypeScriptParser<'config> {
    pub(crate) config: &'config TypeScriptLanguage,
}

impl<'config> Pratt<TypeScriptLanguage> for TypeScriptParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, TypeScriptLanguage> {
        use crate::kind::TypeScriptSyntaxKind::*;
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(IdentifierName) => {
                state.bump();
                state.finish_at(cp, IdentifierName.into())
            }
            Some(NumericLiteral) | Some(StringLiteral) | Some(True) | Some(False) | Some(Null) => {
                state.bump();
                state.finish_at(cp, NumericLiteral.into()) // 简化处理
            }
            Some(LeftParen) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(RightParen).ok();
                state.finish_at(cp, BinaryExpression.into()) // 简化处理
            }
            _ => {
                state.bump();
                state.finish_at(cp, Error.into())
            }
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, TypeScriptLanguage> {
        self.primary(state)
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, TypeScriptLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, TypeScriptLanguage>> {
        use crate::kind::TypeScriptSyntaxKind::*;
        let kind = state.peek_kind()?;

        let (prec, assoc) = match kind {
            Equal
            | PlusEqual
            | MinusEqual
            | StarEqual
            | SlashEqual
            | PercentEqual
            | StarStarEqual
            | LeftShiftEqual
            | RightShiftEqual
            | UnsignedRightShiftEqual
            | AmpersandEqual
            | PipeEqual
            | CaretEqual
            | AmpersandAmpersandEqual
            | PipePipeEqual
            | QuestionQuestionEqual => (1, Associativity::Right),
            PipePipe => (2, Associativity::Left),
            AmpersandAmpersand => (3, Associativity::Left),
            EqualEqual | NotEqual | EqualEqualEqual | NotEqualEqual => (4, Associativity::Left),
            As => (14, Associativity::Left),
            Plus | Minus => (10, Associativity::Left),
            Star | Slash | Percent => (11, Associativity::Left),
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
                Some(state.finish_at(cp, CallExpression.into()))
            }
            Dot => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(Dot).ok();
                state.expect(IdentifierName).ok();
                Some(state.finish_at(cp, MemberExpression.into()))
            }
            As => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(As).ok();
                // 简单处理类型
                state.advance();
                Some(state.finish_at(cp, AsExpression.into()))
            }
            _ => Some(binary(state, left, kind, prec, assoc, BinaryExpression.into(), |s, p| PrattParser::parse(s, p, self))),
        }
    }
}

impl<'config> TypeScriptParser<'config> {
    pub fn new(config: &'config TypeScriptLanguage) -> Self {
        Self { config }
    }

    pub(crate) fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::TypeScriptSyntaxKind::*;
        match state.peek_kind() {
            Some(Var) | Some(Let) | Some(Const) => self.parse_variable_declaration(state)?,
            Some(Function) => self.parse_function_declaration(state)?,
            Some(Class) => self.parse_class_declaration(state)?,
            Some(Interface) => self.parse_interface_declaration(state)?,
            Some(Enum) => self.parse_enum_declaration(state)?,
            Some(Type) => self.parse_type_alias_declaration(state)?,
            Some(Namespace) | Some(Module) => self.parse_namespace_declaration(state)?,
            Some(Import) => self.parse_import_declaration(state)?,
            Some(Export) => self.parse_export_declaration(state)?,
            Some(If) => self.parse_if_statement(state)?,
            Some(For) => self.parse_for_statement(state)?,
            Some(While) => self.parse_while_statement(state)?,
            Some(Return) => self.parse_return_statement(state)?,
            Some(LeftBrace) => self.parse_block(state)?,
            _ => {
                PrattParser::parse(state, 0, self);
                state.eat(Semicolon);
            }
        }
        Ok(())
    }

    fn parse_variable_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::TypeScriptSyntaxKind::*;
        let cp = state.checkpoint();
        state.bump(); // var, let, or const
        while state.at(IdentifierName) {
            state.bump();
            if state.eat(Equal) {
                PrattParser::parse(state, 0, self);
            }
            if !state.eat(Comma) {
                break;
            }
        }
        state.eat(Semicolon);
        state.finish_at(cp, VariableDeclaration.into());
        Ok(())
    }

    fn parse_function_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::TypeScriptSyntaxKind::*;
        let cp = state.checkpoint();
        state.bump(); // function
        state.expect(IdentifierName).ok();
        self.parse_parameters(state)?;
        self.parse_block(state)?;
        state.finish_at(cp, FunctionDeclaration.into());
        Ok(())
    }

    fn parse_class_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::TypeScriptSyntaxKind::*;
        let cp = state.checkpoint();
        state.bump(); // class
        state.expect(IdentifierName).ok();
        if state.eat(Extends) {
            state.expect(IdentifierName).ok();
        }
        self.parse_block(state)?;
        state.finish_at(cp, ClassDeclaration.into());
        Ok(())
    }

    fn parse_interface_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::TypeScriptSyntaxKind::*;
        let cp = state.checkpoint();
        state.bump(); // interface
        state.expect(IdentifierName).ok();
        self.parse_block(state)?;
        state.finish_at(cp, InterfaceDeclaration.into());
        Ok(())
    }

    fn parse_enum_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::TypeScriptSyntaxKind::*;
        let cp = state.checkpoint();
        state.bump(); // enum
        state.expect(IdentifierName).ok();
        self.parse_block(state)?;
        state.finish_at(cp, EnumDeclaration.into());
        Ok(())
    }

    fn parse_type_alias_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::TypeScriptSyntaxKind::*;
        let cp = state.checkpoint();
        state.bump(); // type
        state.expect(IdentifierName).ok();
        state.expect(Equal).ok();
        while state.not_at_end() && !state.at(Semicolon) {
            state.advance();
        }
        state.eat(Semicolon);
        state.finish_at(cp, TypeAliasDeclaration.into());
        Ok(())
    }

    fn parse_namespace_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::TypeScriptSyntaxKind::*;
        let cp = state.checkpoint();
        state.bump(); // namespace or module
        state.expect(IdentifierName).ok();
        self.parse_block(state)?;
        state.finish_at(cp, NamespaceDeclaration.into());
        Ok(())
    }

    fn parse_import_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::TypeScriptSyntaxKind::*;
        let cp = state.checkpoint();
        state.bump(); // import
        while state.not_at_end() && !state.at(Semicolon) {
            state.advance();
        }
        state.eat(Semicolon);
        state.finish_at(cp, ImportDeclaration.into());
        Ok(())
    }

    fn parse_export_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::TypeScriptSyntaxKind::*;
        let cp = state.checkpoint();
        state.bump(); // export
        self.parse_statement(state)?;
        state.finish_at(cp, ExportDeclaration.into());
        Ok(())
    }

    fn parse_if_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::TypeScriptSyntaxKind::*;
        let cp = state.checkpoint();
        state.bump(); // if
        state.expect(LeftParen).ok();
        PrattParser::parse(state, 0, self);
        state.expect(RightParen).ok();
        self.parse_statement(state)?;
        if state.eat(Else) {
            self.parse_statement(state)?;
        }
        state.finish_at(cp, IfStatement.into());
        Ok(())
    }

    fn parse_for_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::TypeScriptSyntaxKind::*;
        let cp = state.checkpoint();
        state.bump(); // for
        state.expect(LeftParen).ok();
        while state.not_at_end() && !state.at(RightParen) {
            state.advance();
        }
        state.expect(RightParen).ok();
        self.parse_statement(state)?;
        state.finish_at(cp, ForStatement.into());
        Ok(())
    }

    fn parse_while_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::TypeScriptSyntaxKind::*;
        let cp = state.checkpoint();
        state.bump(); // while
        state.expect(LeftParen).ok();
        PrattParser::parse(state, 0, self);
        state.expect(RightParen).ok();
        self.parse_statement(state)?;
        state.finish_at(cp, WhileStatement.into());
        Ok(())
    }

    fn parse_parameters<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::TypeScriptSyntaxKind::*;
        state.expect(LeftParen).ok();
        while state.not_at_end() && !state.at(RightParen) {
            state.advance();
        }
        state.expect(RightParen).ok();
        Ok(())
    }

    fn parse_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::TypeScriptSyntaxKind::*;
        let cp = state.checkpoint();
        state.expect(LeftBrace).ok();
        while state.not_at_end() && !state.at(RightBrace) {
            self.parse_statement(state)?;
        }
        state.expect(RightBrace).ok();
        state.finish_at(cp, BlockStatement.into());
        Ok(())
    }

    fn parse_return_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::TypeScriptSyntaxKind::*;
        let cp = state.checkpoint();
        state.bump(); // return
        if !state.at(Semicolon) && !state.at(RightBrace) {
            PrattParser::parse(state, 0, self);
        }
        state.eat(Semicolon);
        state.finish_at(cp, ReturnStatement.into());
        Ok(())
    }
}

impl<'config> Parser<TypeScriptLanguage> for TypeScriptParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<TypeScriptLanguage>) -> ParseOutput<'a, TypeScriptLanguage> {
        let lexer = TypeScriptLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            while state.not_at_end() {
                if state.current().map(|t| t.kind.is_ignored()).unwrap_or(false) {
                    state.advance();
                    continue;
                }
                self.parse_statement(state).ok();
            }
            Ok(state.finish_at(checkpoint, TypeScriptSyntaxKind::SourceFile.into()))
        })
    }
}
