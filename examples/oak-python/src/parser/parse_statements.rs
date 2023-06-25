use crate::{language::PythonLanguage, lexer::PythonTokenType, parser::PythonElementType as ET};
use oak_core::{Source, parser::ParserState};

/// Python parser state.
pub(crate) type State<'a, S> = ParserState<'a, PythonLanguage, S>;

impl<'config> super::PythonParser<'config> {
    pub(crate) fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        use PythonTokenType::*;
        self.skip_trivia(state);

        let cp = state.checkpoint();
        let kind = state.peek_kind();

        match kind {
            Some(At) => {
                self.parse_decorator(state);
            }
            Some(AsyncKeyword) => {
                state.bump();
                self.skip_trivia(state);
                let inner_kind = state.peek_kind();
                match inner_kind {
                    Some(DefKeyword) => {
                        state.bump();
                        self.parse_function_def_body(state, cp);
                    }
                    Some(ForKeyword) => {
                        state.bump();
                        self.parse_for_stmt_body(state, cp);
                    }
                    Some(WithKeyword) => {
                        state.bump();
                        self.parse_with_stmt_body(state, cp);
                    }
                    _ => {
                        // Error or just an expression statement
                        self.parse_expression(state, 0);
                        state.finish_at(cp, ET::ExprStmt);
                    }
                }
            }
            Some(DefKeyword) => {
                state.bump();
                self.parse_function_def_body(state, cp);
            }
            Some(ClassKeyword) => {
                state.bump();
                self.parse_class_def_body(state, cp);
            }
            Some(IfKeyword) => {
                state.bump();
                self.parse_if_stmt_body(state, cp);
            }
            Some(ForKeyword) => {
                state.bump();
                self.parse_for_stmt_body(state, cp);
            }
            Some(WhileKeyword) => {
                state.bump();
                self.parse_while_stmt_body(state, cp);
            }
            Some(TryKeyword) => {
                state.bump();
                self.parse_try_stmt_body(state, cp);
            }
            Some(WithKeyword) => {
                state.bump();
                self.parse_with_stmt_body(state, cp);
            }
            Some(ReturnKeyword) => {
                state.bump();
                self.skip_trivia(state);
                if !state.at(Newline) && !state.at(Eof) {
                    self.parse_expression(state, 0);
                }
                state.finish_at(cp, ET::ReturnStmt);
            }
            Some(ImportKeyword) | Some(FromKeyword) => {
                self.parse_import_stmt(state, cp);
            }
            Some(GlobalKeyword) => {
                state.bump();
                self.parse_global_stmt_body(state, cp);
            }
            Some(NonlocalKeyword) => {
                state.bump();
                self.parse_nonlocal_stmt_body(state, cp);
            }
            Some(_) => {
                // Expression statement or assignment
                self.parse_expression(state, 0);
                state.finish_at(cp, ET::ExprStmt);
            }
            None => {}
        }

        self.skip_trivia(state);
        if state.at(Newline) {
            state.bump();
        }
    }

    fn parse_decorator<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        use PythonTokenType::*;
        let cp = state.checkpoint();
        state.expect(At).ok();
        self.parse_expression(state, 0);
        self.skip_trivia(state);
        state.expect(Newline).ok();
        state.finish_at(cp, ET::Decorator);
    }

    fn parse_function_def_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) {
        use PythonTokenType::*;
        self.skip_trivia(state);
        state.expect(Identifier).ok();

        self.skip_trivia(state);
        state.expect(LeftParen).ok();
        // parameters
        while !state.at(RightParen) && !state.at(Eof) {
            self.skip_trivia(state);
            if state.eat(Star) || state.eat(DoubleStar) {
                self.skip_trivia(state);
            }
            state.expect(Identifier).ok();
            self.skip_trivia(state);

            // Type hint
            if state.eat(Colon) {
                self.parse_expression(state, 0);
                self.skip_trivia(state);
            }

            // Default value
            if state.eat(Assign) {
                self.parse_expression(state, 0);
                self.skip_trivia(state);
            }

            if !state.eat(Comma) {
                break;
            }
        }
        state.expect(RightParen).ok();

        self.skip_trivia(state);
        if state.eat(Arrow) {
            self.parse_expression(state, 0);
        }

        self.skip_trivia(state);
        state.expect(Colon).ok();

        self.parse_suite(state);
        state.finish_at(cp, ET::FunctionDef);
    }

    fn parse_class_def_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) {
        use PythonTokenType::*;
        self.skip_trivia(state);
        state.expect(Identifier).ok();

        self.skip_trivia(state);
        if state.eat(LeftParen) {
            while !state.at(RightParen) && !state.at(Eof) {
                self.parse_expression(state, 0);
                self.skip_trivia(state);
                if !state.eat(Comma) {
                    break;
                }
            }
            state.expect(RightParen).ok();
        }

        self.skip_trivia(state);
        state.expect(Colon).ok();

        self.parse_suite(state);
        state.finish_at(cp, ET::ClassDef);
    }

    fn parse_if_stmt_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) {
        use PythonTokenType::*;
        self.parse_expression(state, 0);
        self.skip_trivia(state);
        state.expect(Colon).ok();
        self.parse_suite(state);

        loop {
            self.skip_trivia(state);
            if state.eat(ElifKeyword) {
                self.parse_expression(state, 0);
                self.skip_trivia(state);
                state.expect(Colon).ok();
                self.parse_suite(state);
            }
            else {
                break;
            }
        }

        self.skip_trivia(state);
        if state.eat(ElseKeyword) {
            self.skip_trivia(state);
            state.expect(Colon).ok();
            self.parse_suite(state);
        }

        state.finish_at(cp, ET::IfStmt);
    }

    fn parse_for_stmt_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) {
        use PythonTokenType::*;
        self.parse_expression(state, 0); // target
        self.skip_trivia(state);
        state.expect(InKeyword).ok();
        self.parse_expression(state, 0); // iter
        self.skip_trivia(state);
        state.expect(Colon).ok();
        self.parse_suite(state);

        self.skip_trivia(state);
        if state.eat(ElseKeyword) {
            self.skip_trivia(state);
            state.expect(Colon).ok();
            self.parse_suite(state);
        }

        state.finish_at(cp, ET::ForStmt);
    }

    fn parse_while_stmt_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) {
        use PythonTokenType::*;
        self.parse_expression(state, 0);
        self.skip_trivia(state);
        state.expect(Colon).ok();
        self.parse_suite(state);

        self.skip_trivia(state);
        if state.eat(ElseKeyword) {
            self.skip_trivia(state);
            state.expect(Colon).ok();
            self.parse_suite(state);
        }

        state.finish_at(cp, ET::WhileStmt);
    }

    fn parse_try_stmt_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) {
        use PythonTokenType::*;
        self.skip_trivia(state);
        state.expect(Colon).ok();
        self.parse_suite(state);

        loop {
            self.skip_trivia(state);
            if state.eat(ExceptKeyword) {
                self.skip_trivia(state);
                if !state.at(Colon) {
                    self.parse_expression(state, 0);
                    self.skip_trivia(state);
                    if state.eat(AsKeyword) {
                        self.skip_trivia(state);
                        state.expect(Identifier).ok();
                    }
                }
                self.skip_trivia(state);
                state.expect(Colon).ok();
                self.parse_suite(state);
            }
            else {
                break;
            }
        }

        self.skip_trivia(state);
        if state.eat(ElseKeyword) {
            self.skip_trivia(state);
            state.expect(Colon).ok();
            self.parse_suite(state);
        }

        self.skip_trivia(state);
        if state.eat(FinallyKeyword) {
            self.skip_trivia(state);
            state.expect(Colon).ok();
            self.parse_suite(state);
        }

        state.finish_at(cp, ET::TryStmt);
    }

    fn parse_with_stmt_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) {
        use PythonTokenType::*;
        loop {
            self.parse_expression(state, 0);
            self.skip_trivia(state);
            if state.eat(AsKeyword) {
                self.skip_trivia(state);
                self.parse_expression(state, 0);
            }
            self.skip_trivia(state);
            if !state.eat(Comma) {
                break;
            }
        }
        state.expect(Colon).ok();
        self.parse_suite(state);
        state.finish_at(cp, ET::WithStmt);
    }

    fn parse_import_stmt<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) {
        use PythonTokenType::*;
        if state.eat(ImportKeyword) {
            loop {
                self.parse_expression(state, 0);
                self.skip_trivia(state);
                if state.eat(AsKeyword) {
                    self.skip_trivia(state);
                    state.expect(Identifier).ok();
                }
                self.skip_trivia(state);
                if !state.eat(Comma) {
                    break;
                }
            }
            state.finish_at(cp, ET::ImportStmt);
        }
        else if state.eat(FromKeyword) {
            // dots
            while state.eat(Dot) {}
            // name
            if !state.at(ImportKeyword) {
                self.parse_expression(state, 0);
            }
            self.skip_trivia(state);
            state.expect(ImportKeyword).ok();
            self.skip_trivia(state);
            if state.eat(Star) {
                // from ... import *
            }
            else {
                loop {
                    state.expect(Identifier).ok();
                    self.skip_trivia(state);
                    if state.eat(AsKeyword) {
                        self.skip_trivia(state);
                        state.expect(Identifier).ok();
                    }
                    self.skip_trivia(state);
                    if !state.eat(Comma) {
                        break;
                    }
                }
            }
            state.finish_at(cp, ET::ImportFromStmt);
        }
    }

    fn parse_global_stmt_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) {
        use PythonTokenType::*;
        loop {
            self.skip_trivia(state);
            state.expect(Identifier).ok();
            self.skip_trivia(state);
            if !state.eat(Comma) {
                break;
            }
        }
        state.finish_at(cp, ET::GlobalStmt);
    }

    fn parse_nonlocal_stmt_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) {
        use PythonTokenType::*;
        loop {
            self.skip_trivia(state);
            state.expect(Identifier).ok();
            self.skip_trivia(state);
            if !state.eat(Comma) {
                break;
            }
        }
        state.finish_at(cp, ET::NonlocalStmt);
    }

    pub(crate) fn parse_suite<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        use PythonTokenType::*;
        self.skip_trivia(state);
        let cp = state.checkpoint();

        if state.eat(Newline) {
            state.expect(Indent).ok();
            while !state.at(Dedent) && !state.at(Eof) {
                self.parse_statement(state);
                self.skip_trivia(state);
            }
            state.expect(Dedent).ok();
        }
        else {
            self.parse_statement(state);
        }

        state.finish_at(cp, ET::Suite);
    }
}
