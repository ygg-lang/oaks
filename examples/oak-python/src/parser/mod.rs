use crate::{kind::PythonSyntaxKind, language::PythonLanguage, lexer::PythonLexer};
use oak_core::{
    OakError, TokenType,
    parser::{
        ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer,
        pratt::{Associativity, Pratt, PrattParser},
    },
    source::{Source, TextEdit},
    tree::GreenNode,
};

pub(crate) type State<'a, S> = ParserState<'a, PythonLanguage, S>;

pub struct PythonParser<'config> {
    pub(crate) config: &'config PythonLanguage,
}

impl<'config> PythonParser<'config> {
    pub fn new(config: &'config PythonLanguage) -> Self {
        Self { config }
    }

    fn advance_until<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, kind: PythonSyntaxKind) {
        while state.not_at_end() && !state.at(kind) {
            state.advance();
        }
    }

    fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        while state.not_at_end() {
            if let Some(kind) = state.peek_kind() {
                if kind.is_ignored() {
                    state.bump();
                    continue;
                }
            }
            break;
        }
    }

    fn parse_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, min_precedence: u8) -> &'a GreenNode<'a, PythonLanguage> {
        let node = PrattParser::parse(state, min_precedence, self);
        state.push_child(node);
        node
    }

    pub(crate) fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::PythonSyntaxKind::*;
        self.skip_trivia(state);

        // Skip leading newlines at top level
        while state.eat(Newline) {
            self.skip_trivia(state);
        }

        if !state.not_at_end() || state.at(Dedent) {
            return Ok(());
        }

        if state.at(DefKeyword) {
            state.incremental_node(FunctionDef.into(), |state| self.parse_function_def_body(state))
        }
        else if state.at(ClassKeyword) {
            state.incremental_node(ClassDef.into(), |state| self.parse_class_def_body(state))
        }
        else if state.at(IfKeyword) {
            state.incremental_node(If.into(), |state| self.parse_if_stmt_body(state))
        }
        else if state.at(WhileKeyword) {
            state.incremental_node(While.into(), |state| self.parse_while_stmt_body(state))
        }
        else if state.at(ForKeyword) {
            state.incremental_node(For.into(), |state| self.parse_for_stmt_body(state))
        }
        else if state.eat(ReturnKeyword) {
            let cp = state.checkpoint();
            self.parse_return_stmt_body(state)?;
            state.finish_at(cp, Return.into());
            state.eat(Newline);
            Ok(())
        }
        else if state.at(ImportKeyword) || state.at(FromKeyword) {
            state.incremental_node(Import.into(), |state| self.parse_import_stmt_body(state))
        }
        else if state.eat(PassKeyword) {
            state.incremental_node(Pass.into(), |state| {
                self.skip_trivia(state);
                state.eat(Newline);
                Ok(())
            })
        }
        else if state.eat(BreakKeyword) {
            state.incremental_node(Break.into(), |state| {
                self.skip_trivia(state);
                state.eat(Newline);
                Ok(())
            })
        }
        else if state.eat(ContinueKeyword) {
            state.incremental_node(Continue.into(), |state| {
                self.skip_trivia(state);
                state.eat(Newline);
                Ok(())
            })
        }
        else {
            let cp = state.checkpoint();
            self.parse_expression(state, 0);
            state.finish_at(cp, Expr.into());
            self.skip_trivia(state);
            state.eat(Newline);
            Ok(())
        }
    }

    fn parse_function_def_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::PythonSyntaxKind::*;
        state.expect(DefKeyword).ok();
        self.skip_trivia(state);
        if !state.expect(Identifier).is_ok() {
            // If identifier is missing, we might want to advance to avoid infinite loop
            // but for now let's just let it be.
        }
        self.skip_trivia(state);
        state.expect(LeftParen).ok();
        state.incremental_node(Arguments.into(), |state| {
            while state.not_at_end() && !state.at(RightParen) {
                self.skip_trivia(state);
                if state.at(RightParen) {
                    break;
                }
                state.incremental_node(Arg.into(), |state| {
                    state.expect(Identifier).ok();
                    self.skip_trivia(state);
                    if state.eat(Colon) {
                        self.skip_trivia(state);
                        // Consume until comma or right paren for simple type annotation
                        while state.not_at_end() && !state.at(Comma) && !state.at(RightParen) {
                            state.advance();
                        }
                    }
                    Ok(())
                })?;
                self.skip_trivia(state);
                if !state.eat(Comma) {
                    break;
                }
            }
            Ok(())
        })?;
        self.skip_trivia(state);
        state.expect(RightParen).ok();
        self.skip_trivia(state);
        if state.eat(Arrow) {
            self.skip_trivia(state);
            self.advance_until(state, Colon);
        }
        state.expect(Colon).ok();
        self.parse_suite(state)?;
        Ok(())
    }

    fn parse_class_def_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::PythonSyntaxKind::*;
        state.expect(ClassKeyword).ok();
        self.skip_trivia(state);
        state.expect(Identifier).ok();
        self.skip_trivia(state);
        if state.eat(LeftParen) {
            self.skip_trivia(state);
            self.advance_until(state, RightParen);
            state.expect(RightParen).ok();
        }
        self.skip_trivia(state);
        state.expect(Colon).ok();
        self.parse_suite(state)?;
        Ok(())
    }

    fn parse_if_stmt_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::PythonSyntaxKind::*;
        state.expect(IfKeyword).ok();
        self.skip_trivia(state);
        PrattParser::parse(state, 0, self);
        self.skip_trivia(state);
        state.expect(Colon).ok();
        self.parse_suite(state)?;
        self.skip_trivia(state);
        while state.eat(ElifKeyword) {
            self.skip_trivia(state);
            PrattParser::parse(state, 0, self);
            self.skip_trivia(state);
            state.expect(Colon).ok();
            self.parse_suite(state)?;
            self.skip_trivia(state);
        }
        if state.eat(ElseKeyword) {
            self.skip_trivia(state);
            state.expect(Colon).ok();
            self.parse_suite(state)?;
        }
        Ok(())
    }

    fn parse_while_stmt_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::PythonSyntaxKind::*;
        state.expect(WhileKeyword).ok();
        self.skip_trivia(state);
        PrattParser::parse(state, 0, self);
        self.skip_trivia(state);
        state.expect(Colon).ok();
        self.parse_suite(state)?;
        self.skip_trivia(state);
        if state.eat(ElseKeyword) {
            self.skip_trivia(state);
            state.expect(Colon).ok();
            self.parse_suite(state)?;
        }
        Ok(())
    }

    fn parse_for_stmt_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::PythonSyntaxKind::*;
        state.expect(ForKeyword).ok();
        self.skip_trivia(state);
        PrattParser::parse(state, 0, self);
        self.skip_trivia(state);
        state.expect(InKeyword).ok();
        self.skip_trivia(state);
        PrattParser::parse(state, 0, self);
        self.skip_trivia(state);
        state.expect(Colon).ok();
        self.parse_suite(state)?;
        self.skip_trivia(state);
        if state.eat(ElseKeyword) {
            self.skip_trivia(state);
            state.expect(Colon).ok();
            self.parse_suite(state)?;
        }
        Ok(())
    }

    fn parse_return_stmt_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::PythonSyntaxKind::*;
        self.skip_trivia(state);
        if state.not_at_end() && !state.at(Newline) && !state.at(Semicolon) {
            self.parse_expression(state, 0);
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
        self.skip_trivia(state);
        if state.eat(Newline) {
            self.skip_trivia(state);
            state.expect(Indent).ok();
            while state.not_at_end() && !state.at(Dedent) {
                self.parse_statement(state)?;
                self.skip_trivia(state);
            }
            state.expect(Dedent).ok();
        }
        else {
            self.parse_statement(state)?;
        }
        state.finish_at(cp, Suite.into());
        Ok(())
    }

    fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, PythonLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            self.parse_statement(state)?;
        }
        self.skip_trivia(state);

        Ok(state.finish_at(checkpoint, PythonSyntaxKind::ExpressionModule.into()))
    }
}

impl<'config> Pratt<PythonLanguage> for PythonParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, PythonLanguage> {
        use crate::kind::PythonSyntaxKind::*;
        self.skip_trivia(state);
        let cp = state.checkpoint();
        let kind = state.peek_kind();
        match kind {
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
                let cp_inner = state.checkpoint();
                let inner = PrattParser::parse(state, 0, self);
                state.push_child(inner);
                state.finish_at(cp_inner, Expr.into());
                self.skip_trivia(state);
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
        self.skip_trivia(state);
        let kind = state.peek_kind().expect("Expected token in prefix");
        match kind {
            Plus | Minus | Tilde | NotKeyword => {
                let cp = state.checkpoint();
                state.expect(kind).ok();
                let right = PrattParser::parse(state, 14, self);
                state.push_child(right);
                state.finish_at(cp, UnaryOp.into())
            }
            _ => self.primary(state),
        }
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, _left: &'a GreenNode<'a, PythonLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, PythonLanguage>> {
        use crate::kind::PythonSyntaxKind::*;

        // Peek kind without consuming trivia yet
        let mut lookahead = 0;
        let mut kind = None;
        while let Some(k) = state.peek_kind_at(lookahead) {
            if k.is_ignored() {
                lookahead += 1;
                continue;
            }
            kind = Some(k);
            break;
        }

        let kind = kind?;

        let (prec, assoc) = match kind {
            Assign | PlusAssign | MinusAssign | StarAssign | DoubleStarAssign | SlashAssign | DoubleSlashAssign | PercentAssign | AtAssign | AmpersandAssign | PipeAssign | CaretAssign | LeftShiftAssign | RightShiftAssign => (1, Associativity::Right),
            OrKeyword => (2, Associativity::Left),
            AndKeyword => (3, Associativity::Left),
            NotKeyword => (4, Associativity::Left),
            Less | Greater | Equal | NotEqual | LessEqual | GreaterEqual | InKeyword | IsKeyword => (5, Associativity::Left),
            Pipe => (6, Associativity::Left),
            Caret => (7, Associativity::Left),
            Ampersand => (8, Associativity::Left),
            LeftShift | RightShift => (9, Associativity::Left),
            Plus | Minus => (10, Associativity::Left),
            Star | Slash | DoubleSlash | Percent | At => (11, Associativity::Left),
            DoubleStar => (13, Associativity::Right),
            Dot | LeftParen | LeftBracket => (15, Associativity::Left),
            _ => return None,
        };

        if prec < min_precedence {
            return None;
        }

        match kind {
            LeftParen => {
                let cp = (0, state.sink.checkpoint() - 1);
                self.skip_trivia(state);
                state.expect(LeftParen).ok();
                self.advance_until(state, RightParen);
                state.expect(RightParen).ok();
                Some(state.finish_at(cp, Call.into()))
            }
            LeftBracket => {
                let cp = (0, state.sink.checkpoint() - 1);
                self.skip_trivia(state);
                state.expect(LeftBracket).ok();
                self.advance_until(state, RightBracket);
                state.expect(RightBracket).ok();
                Some(state.finish_at(cp, Subscript.into()))
            }
            Dot => {
                let cp = (0, state.sink.checkpoint() - 1);
                self.skip_trivia(state);
                state.expect(Dot).ok();
                self.skip_trivia(state);
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

                let cp = (0, state.sink.checkpoint() - 1);
                self.skip_trivia(state);
                state.expect(kind).ok();

                let next_prec = match assoc {
                    Associativity::Left => prec + 1,
                    Associativity::Right => prec,
                    Associativity::None => prec + 1,
                };

                PrattParser::parse(state, next_prec, self);
                Some(state.finish_at(cp, result_kind.into()))
            }
        }
    }
}

impl<'config> Parser<PythonLanguage> for PythonParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<PythonLanguage>) -> ParseOutput<'a, PythonLanguage> {
        let lexer = PythonLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
