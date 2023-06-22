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

impl<'config> TypeScriptParser<'config> {
    pub fn new(config: &'config TypeScriptLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Pratt<TypeScriptLanguage> for TypeScriptParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, TypeScriptLanguage> {
        use crate::kind::TypeScriptSyntaxKind::*;
        let kind = self.peek_kind(state);
        let cp = state.checkpoint();
        match kind {
            Some(IdentifierName) => {
                self.expect(state, IdentifierName).ok();
                state.finish_at(cp, IdentifierName.into())
            }
            Some(NumericLiteral) | Some(StringLiteral) | Some(BigIntLiteral) | Some(TemplateString) | Some(True) | Some(False) | Some(Null) | Some(RegexLiteral) => {
                let kind = state.peek_kind().unwrap();
                state.bump();
                state.finish_at(cp, kind.into())
            }
            Some(LeftParen) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                self.expect(state, RightParen).ok();
                state.finish_at(cp, BinaryExpression.into()) // 简化处理
            }
            _ => {
                state.bump();
                state.finish_at(cp, Error.into())
            }
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, TypeScriptLanguage> {
        use crate::kind::TypeScriptSyntaxKind::*;
        let kind = self.peek_kind(state);
        let cp = state.checkpoint();
        match kind {
            Some(Plus) | Some(Minus) | Some(Exclamation) | Some(Tilde) | Some(Typeof) | Some(Void) | Some(Delete) | Some(Await) => {
                state.bump();
                PrattParser::parse(state, 15, self); // High precedence for prefix
                state.finish_at(cp, UnaryExpression.into())
            }
            Some(New) => {
                state.bump();
                PrattParser::parse(state, 17, self); // Higher precedence than call
                if self.eat(state, LeftParen) {
                    while state.not_at_end() && !self.at(state, RightParen) {
                        let acp = state.checkpoint();
                        PrattParser::parse(state, 0, self);
                        state.finish_at(acp, CallArgument.into());
                        self.eat(state, Comma);
                    }
                    self.expect(state, RightParen).ok();
                }
                state.finish_at(cp, NewExpression.into())
            }
            _ => self.primary(state),
        }
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, TypeScriptLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, TypeScriptLanguage>> {
        use crate::kind::TypeScriptSyntaxKind::*;
        let kind = self.peek_kind(state)?;

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
            Question => (2, Associativity::Right),
            PipePipe => (3, Associativity::Left),
            AmpersandAmpersand => (4, Associativity::Left),
            Pipe => (5, Associativity::Left),
            Caret => (6, Associativity::Left),
            Ampersand => (7, Associativity::Left),
            EqualEqual | NotEqual | EqualEqualEqual | NotEqualEqual => (8, Associativity::Left),
            Less | Greater | LessEqual | GreaterEqual | Instanceof | In => (9, Associativity::Left),
            LeftShift | RightShift | UnsignedRightShift => (10, Associativity::Left),
            Plus | Minus => (11, Associativity::Left),
            Star | Slash | Percent => (12, Associativity::Left),
            StarStar => (13, Associativity::Right),
            As => (14, Associativity::Left),
            LeftParen | Dot | LeftBracket | QuestionDot => (16, Associativity::Left),
            _ => return None,
        };

        if prec < min_precedence {
            return None;
        }

        match kind {
            LeftParen => {
                let cp = state.checkpoint();
                state.push_child(left);
                self.expect(state, LeftParen).ok();
                while state.not_at_end() && !self.at(state, RightParen) {
                    self.skip_trivia(state);
                    let acp = state.checkpoint();
                    PrattParser::parse(state, 0, self);
                    state.finish_at(acp, CallArgument.into());
                    self.eat(state, Comma);
                }
                self.expect(state, RightParen).ok();
                Some(state.finish_at(cp, CallExpression.into()))
            }
            Dot | QuestionDot => {
                let cp = state.checkpoint();
                state.push_child(left);
                let op = if kind == Dot { Dot } else { QuestionDot };
                self.expect(state, op).ok();
                self.expect(state, IdentifierName).ok();
                Some(state.finish_at(cp, MemberExpression.into()))
            }
            LeftBracket => {
                let cp = state.checkpoint();
                state.push_child(left);
                self.expect(state, LeftBracket).ok();
                PrattParser::parse(state, 0, self);
                self.expect(state, RightBracket).ok();
                Some(state.finish_at(cp, MemberExpression.into()))
            }
            As => {
                let cp = state.checkpoint();
                state.push_child(left);
                self.expect(state, As).ok();
                // 简单处理类型：跳过接下来的标识符或基本类型
                self.skip_trivia(state);
                if state.at(IdentifierName.into()) {
                    self.expect(state, IdentifierName).ok();
                }
                Some(state.finish_at(cp, AsExpression.into()))
            }
            Question => {
                let cp = state.checkpoint();
                state.push_child(left);
                self.expect(state, Question).ok();
                PrattParser::parse(state, 0, self);
                self.expect(state, Colon).ok();
                PrattParser::parse(state, 0, self);
                Some(state.finish_at(cp, ConditionalExpression.into()))
            }
            _ => Some(binary(state, left, kind, prec, assoc, BinaryExpression.into(), |s, p| PrattParser::parse(s, p, self))),
        }
    }
}

impl<'config> TypeScriptParser<'config> {
    fn peek_kind<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Option<TypeScriptSyntaxKind> {
        self.skip_trivia(state);
        state.peek_kind().map(|k| k.try_into().unwrap())
    }

    fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        while state.not_at_end() && state.current().map(|t| t.kind.is_ignored()).unwrap_or(false) {
            state.bump();
        }
    }

    fn expect<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, kind: TypeScriptSyntaxKind) -> Result<(), OakError> {
        self.skip_trivia(state);
        let cp = state.checkpoint();
        let res = state.expect(kind.into());
        if res.is_ok() {
            state.finish_at(cp, kind.into());
        }
        res
    }

    fn eat<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, kind: TypeScriptSyntaxKind) -> bool {
        self.skip_trivia(state);
        let cp = state.checkpoint();
        let res = state.eat(kind.into());
        if res {
            state.finish_at(cp, kind.into());
        }
        res
    }

    fn at<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, kind: TypeScriptSyntaxKind) -> bool {
        self.skip_trivia(state);
        state.at(kind.into())
    }

    pub(crate) fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::TypeScriptSyntaxKind::*;
        self.skip_trivia(state);
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
                let cp = state.checkpoint();
                PrattParser::parse(state, 0, self);
                self.skip_trivia(state);
                state.eat(Semicolon);
                state.finish_at(cp, ExpressionStatement.into());
            }
        }
        Ok(())
    }

    fn parse_variable_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::TypeScriptSyntaxKind::*;
        let cp = state.checkpoint();
        state.bump(); // var, let, or const
        self.skip_trivia(state);
        while state.at(IdentifierName.into()) {
            self.expect(state, IdentifierName).ok();
            if self.eat(state, Equal) {
                PrattParser::parse(state, 0, self);
            }
            if !self.eat(state, Comma) {
                break;
            }
            self.skip_trivia(state);
        }
        self.eat(state, Semicolon);
        state.finish_at(cp, VariableDeclaration.into());
        Ok(())
    }

    fn parse_function_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::TypeScriptSyntaxKind::*;
        let cp = state.checkpoint();
        state.bump(); // function
        self.skip_trivia(state);
        if self.at(state, IdentifierName) {
            self.expect(state, IdentifierName).ok();
        }
        self.parse_parameters(state)?;
        self.parse_block(state)?;
        state.finish_at(cp, FunctionDeclaration.into());
        Ok(())
    }

    fn parse_class_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::TypeScriptSyntaxKind::*;
        let cp = state.checkpoint();
        state.bump(); // class
        self.expect(state, IdentifierName).ok();
        if self.eat(state, Extends) {
            self.expect(state, IdentifierName).ok();
        }
        self.parse_class_body(state)?;
        state.finish_at(cp, ClassDeclaration.into());
        Ok(())
    }

    fn parse_class_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::TypeScriptSyntaxKind::*;
        let cp = state.checkpoint();
        self.expect(state, LeftBrace).ok();
        while state.not_at_end() && !self.at(state, RightBrace) {
            self.skip_trivia(state);
            let mcp = state.checkpoint();
            self.eat(state, Static);
            self.eat(state, Public);
            self.eat(state, Private);
            self.eat(state, Protected);

            if self.at(state, IdentifierName) {
                self.expect(state, IdentifierName).ok();
                if self.at(state, LeftParen) {
                    self.parse_parameters(state)?;
                    self.skip_trivia(state);
                    if self.eat(state, Colon) {
                        self.expect(state, IdentifierName).ok();
                    }
                    self.parse_block(state)?;
                    state.finish_at(mcp, MethodDeclaration.into());
                }
                else {
                    if self.eat(state, Colon) {
                        self.expect(state, IdentifierName).ok();
                    }
                    if self.eat(state, Equal) {
                        PrattParser::parse(state, 0, self);
                    }
                    self.eat(state, Semicolon);
                    state.finish_at(mcp, PropertyDeclaration.into());
                }
            }
            else if self.at(state, Constructor) {
                state.bump();
                self.parse_parameters(state)?;
                self.skip_trivia(state);
                if self.eat(state, Colon) {
                    self.expect(state, IdentifierName).ok();
                }
                self.parse_block(state)?;
                state.finish_at(mcp, ConstructorDeclaration.into());
            }
            else if self.at(state, Semicolon) {
                state.bump(); // Skip extra semicolons in class body
            }
            else if !self.at(state, RightBrace) && state.not_at_end() {
                // If we encounter something unexpected, try to skip until next member or end of class
                state.bump();
            }
            else {
                break;
            }
        }
        self.expect(state, RightBrace).ok();
        state.finish_at(cp, ClassBody.into());
        Ok(())
    }

    fn parse_interface_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::TypeScriptSyntaxKind::*;
        let cp = state.checkpoint();
        state.bump(); // interface
        self.expect(state, IdentifierName).ok();
        self.parse_block(state)?;
        state.finish_at(cp, InterfaceDeclaration.into());
        Ok(())
    }

    fn parse_enum_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::TypeScriptSyntaxKind::*;
        let cp = state.checkpoint();
        state.bump(); // enum
        self.expect(state, IdentifierName).ok();
        self.parse_block(state)?;
        state.finish_at(cp, EnumDeclaration.into());
        Ok(())
    }

    fn parse_type_alias_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::TypeScriptSyntaxKind::*;
        let cp = state.checkpoint();
        state.bump(); // type
        self.expect(state, IdentifierName).ok();
        self.expect(state, Equal).ok();
        while state.not_at_end() && !self.at(state, Semicolon) {
            self.skip_trivia(state);
            if state.not_at_end() && !self.at(state, Semicolon) {
                state.bump();
            }
            else {
                break;
            }
        }
        self.eat(state, Semicolon);
        state.finish_at(cp, TypeAliasDeclaration.into());
        Ok(())
    }

    fn parse_namespace_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::TypeScriptSyntaxKind::*;
        let cp = state.checkpoint();
        state.bump(); // namespace or module
        self.expect(state, IdentifierName).ok();
        self.parse_block(state)?;
        state.finish_at(cp, NamespaceDeclaration.into());
        Ok(())
    }

    fn parse_import_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::TypeScriptSyntaxKind::*;
        let cp = state.checkpoint();
        state.bump(); // import

        self.skip_trivia(state);
        if self.eat(state, LeftBrace) {
            // import { a, b as c } from '...'
            while state.not_at_end() && !self.at(state, RightBrace) {
                self.skip_trivia(state);
                if self.at(state, IdentifierName) {
                    self.expect(state, IdentifierName).ok();
                    if self.eat(state, As) {
                        self.expect(state, IdentifierName).ok();
                    }
                }
                if !self.eat(state, Comma) {
                    break;
                }
            }
            self.expect(state, RightBrace).ok();
        }
        else if self.at(state, Star) {
            // import * as ns from '...'
            state.bump();
            self.expect(state, As).ok();
            self.expect(state, IdentifierName).ok();
        }
        else if self.at(state, IdentifierName) {
            // import defaultExport from '...'
            self.expect(state, IdentifierName).ok();
        }

        if self.eat(state, From) {
            self.expect(state, StringLiteral).ok();
        }
        else if self.at(state, StringLiteral) {
            // import '...'
            self.expect(state, StringLiteral).ok();
        }

        self.eat(state, Semicolon);
        state.finish_at(cp, ImportDeclaration.into());
        Ok(())
    }

    fn parse_export_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::TypeScriptSyntaxKind::*;
        let cp = state.checkpoint();
        state.bump(); // export

        self.skip_trivia(state);
        if self.eat(state, Default) {
            self.parse_statement(state)?;
        }
        else if self.at(state, LeftBrace) {
            // export { a, b as c }
            state.bump();
            while state.not_at_end() && !self.at(state, RightBrace) {
                self.skip_trivia(state);
                if self.at(state, IdentifierName) {
                    state.bump();
                    if self.eat(state, As) {
                        self.expect(state, IdentifierName).ok();
                    }
                }
                if !self.eat(state, Comma) {
                    break;
                }
            }
            self.expect(state, RightBrace).ok();
            if self.eat(state, From) {
                self.expect(state, StringLiteral).ok();
            }
            self.eat(state, Semicolon);
        }
        else if self.at(state, Star) {
            // export * from '...'
            state.bump();
            if self.eat(state, As) {
                self.expect(state, IdentifierName).ok();
            }
            self.expect(state, From).ok();
            self.expect(state, StringLiteral).ok();
            self.eat(state, Semicolon);
        }
        else {
            self.parse_statement(state)?;
        }

        state.finish_at(cp, ExportDeclaration.into());
        Ok(())
    }

    fn parse_if_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::TypeScriptSyntaxKind::*;
        let cp = state.checkpoint();
        state.bump(); // if
        self.expect(state, LeftParen).ok();
        PrattParser::parse(state, 0, self);
        self.expect(state, RightParen).ok();
        self.parse_statement(state)?;
        if self.eat(state, Else) {
            self.parse_statement(state)?;
        }
        state.finish_at(cp, IfStatement.into());
        Ok(())
    }

    fn parse_for_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::TypeScriptSyntaxKind::*;
        let cp = state.checkpoint();
        state.bump(); // for
        self.expect(state, LeftParen).ok();
        while state.not_at_end() && !self.at(state, RightParen) {
            self.skip_trivia(state);
            if state.not_at_end() && !self.at(state, RightParen) {
                state.bump();
            }
            else {
                break;
            }
        }
        self.expect(state, RightParen).ok();
        self.parse_statement(state)?;
        state.finish_at(cp, ForStatement.into());
        Ok(())
    }

    fn parse_while_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::TypeScriptSyntaxKind::*;
        let cp = state.checkpoint();
        state.bump(); // while
        self.expect(state, LeftParen).ok();
        PrattParser::parse(state, 0, self);
        self.expect(state, RightParen).ok();
        self.parse_statement(state)?;
        state.finish_at(cp, WhileStatement.into());
        Ok(())
    }

    fn parse_parameters<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::TypeScriptSyntaxKind::*;
        self.expect(state, LeftParen).ok();
        while state.not_at_end() && !self.at(state, RightParen) {
            self.skip_trivia(state);
            let cp = state.checkpoint();
            if self.at(state, IdentifierName) {
                self.expect(state, IdentifierName).ok();
                // Skip type annotation
                if self.eat(state, Colon) {
                    while state.not_at_end() && !self.at(state, Comma) && !self.at(state, RightParen) {
                        self.skip_trivia(state);
                        if state.not_at_end() && !self.at(state, Comma) && !self.at(state, RightParen) {
                            state.bump();
                        }
                        else {
                            break;
                        }
                    }
                }
            }
            else {
                state.bump();
            }
            state.finish_at(cp, Parameter.into());
            self.eat(state, Comma);
        }
        self.expect(state, RightParen).ok();
        Ok(())
    }

    fn parse_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::TypeScriptSyntaxKind::*;
        let cp = state.checkpoint();
        self.expect(state, LeftBrace).ok();
        while state.not_at_end() && !self.at(state, RightBrace) {
            self.parse_statement(state)?;
        }
        self.expect(state, RightBrace).ok();
        state.finish_at(cp, BlockStatement.into());
        Ok(())
    }

    fn parse_return_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::TypeScriptSyntaxKind::*;
        let cp = state.checkpoint();
        state.bump(); // return
        if !self.at(state, Semicolon) && !self.at(state, RightBrace) {
            PrattParser::parse(state, 0, self);
        }
        self.eat(state, Semicolon);
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
                self.skip_trivia(state);
                if state.not_at_end() {
                    self.parse_statement(state).ok();
                }
            }
            Ok(state.finish_at(checkpoint, TypeScriptSyntaxKind::SourceFile.into()))
        })
    }
}
