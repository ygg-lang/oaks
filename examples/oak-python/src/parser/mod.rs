use crate::{kind::PythonSyntaxKind, language::PythonLanguage, lexer::PythonLexer};
use oak_core::{
    OakError,
    parser::{
        ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer,
        pratt::{Associativity, Pratt, PrattParser, binary, unary},
    },
    source::{Source, TextEdit},
    tree::GreenNode,
};

pub(crate) type State<'a, S> = ParserState<'a, PythonLanguage, S>;

pub struct PythonParser<'config> {
    pub(crate) _config: &'config PythonLanguage,
}

impl<'config> PythonParser<'config> {
    pub fn new(config: &'config PythonLanguage) -> Self {
        Self { _config: config }
    }

    fn advance_until<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, kind: PythonSyntaxKind) {
        while state.not_at_end() && !state.at(kind) {
            state.advance();
        }
    }

    pub(crate) fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::PythonSyntaxKind::*;
        let kind = match state.peek_kind() {
            Some(DefKeyword) => Some(FunctionDef),
            Some(ClassKeyword) => Some(ClassDef),
            Some(IfKeyword) => Some(If),
            Some(WhileKeyword) => Some(While),
            Some(ForKeyword) => Some(For),
            Some(ReturnKeyword) => Some(Return),
            Some(ImportKeyword) | Some(FromKeyword) => Some(Import),
            _ => None,
        };

        if let Some(k) = kind {
            state.incremental_node(k.into(), |state| match k {
                FunctionDef => self.parse_function_def_body(state),
                ClassDef => self.parse_class_def_body(state),
                If => self.parse_if_stmt_body(state),
                While => self.parse_while_stmt_body(state),
                For => self.parse_for_stmt_body(state),
                Return => self.parse_return_stmt_body(state),
                Import => self.parse_import_stmt_body(state),
                _ => unreachable!(),
            })
        }
        else {
            PrattParser::parse(state, 0, self);
            state.eat(Newline);
            Ok(())
        }
    }

    fn parse_function_def_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::PythonSyntaxKind::*;
        state.expect(DefKeyword).ok();
        state.expect(Identifier).ok();
        state.expect(LeftParen).ok();
        self.advance_until(state, RightParen);
        state.expect(RightParen).ok();
        state.expect(Colon).ok();
        self.parse_suite(state)?;
        Ok(())
    }

    fn parse_class_def_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::PythonSyntaxKind::*;
        state.expect(ClassKeyword).ok();
        state.expect(Identifier).ok();
        if state.eat(LeftParen) {
            self.advance_until(state, RightParen);
            state.expect(RightParen).ok();
        }
        state.expect(Colon).ok();
        self.parse_suite(state)?;
        Ok(())
    }

    fn parse_if_stmt_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::PythonSyntaxKind::*;
        state.expect(IfKeyword).ok();
        PrattParser::parse(state, 0, self);
        state.expect(Colon).ok();
        self.parse_suite(state)?;
        while state.eat(ElifKeyword) {
            PrattParser::parse(state, 0, self);
            state.expect(Colon).ok();
            self.parse_suite(state)?;
        }
        if state.eat(ElseKeyword) {
            state.expect(Colon).ok();
            self.parse_suite(state)?;
        }
        Ok(())
    }

    fn parse_while_stmt_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::PythonSyntaxKind::*;
        state.expect(WhileKeyword).ok();
        PrattParser::parse(state, 0, self);
        state.expect(Colon).ok();
        self.parse_suite(state)?;
        if state.eat(ElseKeyword) {
            state.expect(Colon).ok();
            self.parse_suite(state)?;
        }
        Ok(())
    }

    fn parse_for_stmt_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::PythonSyntaxKind::*;
        state.expect(ForKeyword).ok();
        PrattParser::parse(state, 0, self);
        state.expect(InKeyword).ok();
        PrattParser::parse(state, 0, self);
        state.expect(Colon).ok();
        self.parse_suite(state)?;
        if state.eat(ElseKeyword) {
            state.expect(Colon).ok();
            self.parse_suite(state)?;
        }
        Ok(())
    }

    fn parse_return_stmt_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::PythonSyntaxKind::*;
        state.expect(ReturnKeyword).ok();
        if !state.at(Newline) && !state.at(Semicolon) {
            PrattParser::parse(state, 0, self);
        }
        Ok(())
    }

    fn parse_import_stmt_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::PythonSyntaxKind::*;
        if state.eat(ImportKeyword) {
            self.advance_until(state, Newline);
        }
        else if state.eat(FromKeyword) {
            self.advance_until(state, Newline);
        }
        Ok(())
    }

    fn parse_suite<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::PythonSyntaxKind::*;
        let cp = state.checkpoint();
        if state.eat(Newline) {
            state.expect(Indent).ok();
            while state.not_at_end() && !state.at(Dedent) {
                self.parse_statement(state)?;
            }
            state.expect(Dedent).ok();
        }
        else {
            self.parse_statement(state)?;
        }
        state.finish_at(cp, Suite.into());
        Ok(())
    }
}

impl<'config> Pratt<PythonLanguage> for PythonParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, PythonLanguage> {
        use crate::kind::PythonSyntaxKind::*;
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(Identifier) => {
                state.bump();
                state.finish_at(cp, Name.into())
            }
            Some(Number) | Some(String) | Some(Bytes) | Some(FString) | Some(TrueKeyword) | Some(FalseKeyword) | Some(NoneKeyword) => {
                state.bump();
                state.finish_at(cp, Constant.into())
            }
            Some(LeftParen) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(RightParen).ok();
                state.finish_at(cp, Tuple.into())
            }
            Some(LeftBracket) => {
                state.bump();
                self.advance_until(state, RightBracket);
                state.expect(RightBracket).ok();
                state.finish_at(cp, List.into())
            }
            Some(LeftBrace) => {
                state.bump();
                self.advance_until(state, RightBrace);
                state.expect(RightBrace).ok();
                state.finish_at(cp, Dict.into())
            }
            _ => {
                state.bump();
                state.finish_at(cp, Error.into())
            }
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, PythonLanguage> {
        use crate::kind::PythonSyntaxKind::*;
        let kind = state.peek_kind().unwrap();
        match kind {
            Plus | Minus | Tilde | NotKeyword => unary(state, kind, 14, UnaryOp.into(), |s, p| PrattParser::parse(s, p, self)),
            _ => self.primary(state),
        }
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, PythonLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, PythonLanguage>> {
        use crate::kind::PythonSyntaxKind::*;
        let kind = state.peek_kind()?;

        let (prec, assoc) = match kind {
            Assign | PlusAssign | MinusAssign | StarAssign | SlashAssign | PercentAssign | AmpersandAssign | PipeAssign | CaretAssign | LeftShiftAssign | RightShiftAssign | DoubleStarAssign | DoubleSlashAssign => (1, Associativity::Right),
            OrKeyword => (2, Associativity::Left),
            AndKeyword => (3, Associativity::Left),
            NotKeyword => (4, Associativity::Left),
            Equal | NotEqual | Less | Greater | LessEqual | GreaterEqual | InKeyword | IsKeyword => (5, Associativity::Left),
            Pipe => (6, Associativity::Left),
            Caret => (7, Associativity::Left),
            Ampersand => (8, Associativity::Left),
            LeftShift | RightShift => (9, Associativity::Left),
            Plus | Minus => (10, Associativity::Left),
            Star | Slash | Percent | DoubleSlash | At => (11, Associativity::Left),
            DoubleStar => (13, Associativity::Right),
            LeftParen | LeftBracket | Dot => (15, Associativity::Left),
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
                self.advance_until(state, RightParen);
                state.expect(RightParen).ok();
                Some(state.finish_at(cp, Call.into()))
            }
            LeftBracket => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(LeftBracket).ok();
                while state.not_at_end() && !state.at(RightBracket) {
                    state.advance();
                }
                state.expect(RightBracket).ok();
                Some(state.finish_at(cp, Subscript.into()))
            }
            Dot => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(Dot).ok();
                state.expect(Identifier).ok();
                Some(state.finish_at(cp, Attribute.into()))
            }
            _ => {
                let result_kind = if prec == 1 {
                    AssignStmt
                }
                else if prec <= 3 {
                    BoolOp
                }
                else if prec == 5 {
                    Compare
                }
                else {
                    BinOp
                };
                Some(binary(state, left, kind, prec, assoc, result_kind.into(), |s, p| PrattParser::parse(s, p, self)))
            }
        }
    }
}

impl<'config> Parser<PythonLanguage> for PythonParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<PythonLanguage>) -> ParseOutput<'a, PythonLanguage> {
        let lexer = PythonLexer::new(self._config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                self.parse_statement(state)?;
            }

            Ok(state.finish_at(checkpoint, PythonSyntaxKind::ExpressionModule.into()))
        })
    }
}
