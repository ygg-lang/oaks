pub mod element_type;

use crate::{
    language::PythonLanguage,
    lexer::{PythonLexer, token_type::PythonTokenType},
};
use oak_core::{
    OakError,
    parser::{
        ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer,
        pratt::{Associativity, Pratt, PrattParser},
    },
    source::{Source, TextEdit},
    tree::GreenNode,
};

pub(crate) type State<'a, S> = ParserState<'a, PythonLanguage, S>;

/// Python parser implementation.
pub struct PythonParser<'config> {
    /// Reference to the language configuration.
    pub(crate) config: &'config PythonLanguage,
}

impl<'config> PythonParser<'config> {
    /// Creates a new Python parser.
    pub fn new(config: &'config PythonLanguage) -> Self {
        Self { config }
    }

    /// Advances the parser until it reaches a token of the specified kind.
    fn advance_until<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, kind: PythonTokenType) {
        while state.not_at_end() && !state.at(kind) {
            state.advance()
        }
    }

    /// Skips trivia tokens (whitespace, comments, etc.).
    fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        while state.not_at_end() {
            if let Some(kind) = state.peek_kind() {
                if kind.is_trivia() {
                    state.bump();
                    continue;
                }
            }
            break;
        }
    }

    /// Parses an expression with the given minimum precedence.
    fn parse_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, min_precedence: u8) -> &'a GreenNode<'a, PythonLanguage> {
        PrattParser::parse(state, min_precedence, self)
    }

    /// Parses a single statement.
    pub(crate) fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::{lexer::PythonTokenType::*, parser::element_type::PythonElementType as ET};
        let _cp = state.checkpoint();
        self.skip_trivia(state);

        // Skip leading newlines at top level
        while state.eat(Newline) {
            self.skip_trivia(state)
        }

        if !state.not_at_end() || state.at(Dedent) || state.at(Eof) {
            return Ok(());
        }

        if state.at(At) {
            let cp = state.checkpoint();
            state.bump();
            self.parse_expression(state, 0);
            state.finish_at(cp, crate::parser::element_type::PythonElementType::Decorator);
            self.skip_trivia(state);
            state.eat(Newline);
            return self.parse_statement(state);
        }

        if state.at(DefKeyword) {
            state.incremental_node(ET::FunctionDef.into(), |state| self.parse_function_def_body(state))
        }
        else if state.at(AsyncKeyword) {
            state.bump(); // Consume async
            self.skip_trivia(state);
            if state.at(DefKeyword) {
                state.incremental_node(ET::AsyncFunctionDef.into(), |state| self.parse_function_def_body(state))
            }
            else if state.at(ForKeyword) {
                state.incremental_node(ET::AsyncFor.into(), |state| self.parse_for_stmt_body(state))
            }
            else if state.at(WithKeyword) {
                state.incremental_node(ET::AsyncWith.into(), |state| self.parse_with_stmt_body(state))
            }
            else {
                state.incremental_node(ET::Error.into(), |state| {
                    state.bump();
                    Ok(())
                })
            }
        }
        else if state.at(ClassKeyword) {
            state.incremental_node(ET::ClassDef.into(), |state| self.parse_class_def_body(state))
        }
        else if state.at(IfKeyword) {
            state.incremental_node(ET::If.into(), |state| self.parse_if_stmt_body(state))
        }
        else if state.at(WhileKeyword) {
            state.incremental_node(ET::While.into(), |state| self.parse_while_stmt_body(state))
        }
        else if state.at(ForKeyword) {
            state.incremental_node(ET::For.into(), |state| self.parse_for_stmt_body(state))
        }
        else if state.at(TryKeyword) {
            state.incremental_node(ET::Try.into(), |state| self.parse_try_stmt_body(state))
        }
        else if state.at(WithKeyword) {
            state.incremental_node(ET::With.into(), |state| self.parse_with_stmt_body(state))
        }
        else if state.at(RaiseKeyword) {
            state.incremental_node(ET::Raise.into(), |state| self.parse_raise_stmt_body(state))
        }
        else if state.at(AssertKeyword) {
            state.incremental_node(ET::Assert.into(), |state| self.parse_assert_stmt_body(state))
        }
        else if state.at(DelKeyword) {
            state.incremental_node(ET::Delete.into(), |state| self.parse_del_stmt_body(state))
        }
        else if state.at(GlobalKeyword) {
            state.incremental_node(ET::Global.into(), |state| self.parse_global_stmt_body(state))
        }
        else if state.at(NonlocalKeyword) {
            state.incremental_node(ET::Nonlocal.into(), |state| self.parse_nonlocal_stmt_body(state))
        }
        else if state.eat(ReturnKeyword) {
            let cp = state.checkpoint();
            self.parse_return_stmt_body(state)?;
            state.finish_at(cp, ET::Return);
            state.eat(Newline);
            Ok(())
        }
        else if state.at(ImportKeyword) || state.at(FromKeyword) {
            state.incremental_node(ET::Import.into(), |state| self.parse_import_stmt_body(state))
        }
        else if state.eat(PassKeyword) {
            state.incremental_node(ET::Pass.into(), |state| {
                self.skip_trivia(state);
                state.eat(Newline);
                Ok(())
            })
        }
        else if state.eat(BreakKeyword) {
            state.incremental_node(ET::Break.into(), |state| {
                self.skip_trivia(state);
                state.eat(Newline);
                Ok(())
            })
        }
        else if state.eat(ContinueKeyword) {
            state.incremental_node(ET::Continue.into(), |state| {
                self.skip_trivia(state);
                state.eat(Newline);
                Ok(())
            })
        }
        else {
            let cp = state.checkpoint();
            self.parse_expression(state, 0);
            state.finish_at(cp, ET::Expr);
            self.skip_trivia(state);
            state.eat(Newline);
            Ok(())
        }
    }

    /// Parses the body of a function definition.
    fn parse_function_def_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::{lexer::PythonTokenType::*, parser::element_type::PythonElementType as ET};
        state.expect(DefKeyword).ok();
        self.skip_trivia(state);
        if !state.expect(Identifier).is_ok() {
            // If identifier is missing, we might want to advance to avoid infinite loop
            // but for now let's just let it be.
        }
        self.skip_trivia(state);
        state.expect(LeftParen).ok();
        state.incremental_node(ET::Arguments.into(), |state| {
            while state.not_at_end() && !state.at(RightParen) {
                self.skip_trivia(state);
                if state.at(RightParen) {
                    break;
                }
                state.incremental_node(ET::Arg.into(), |state| {
                    if state.at(Star) {
                        state.bump();
                        self.skip_trivia(state);
                    }
                    else if state.at(DoubleStar) {
                        state.bump();
                        self.skip_trivia(state);
                    }
                    state.expect(Identifier).ok();
                    self.skip_trivia(state);
                    if state.eat(Colon) {
                        self.skip_trivia(state);
                        // Consume until comma or right paren for simple type annotation
                        while state.not_at_end() && !state.at(Comma) && !state.at(RightParen) && !state.at(Assign) {
                            state.advance();
                        }
                    }
                    self.skip_trivia(state);
                    if state.eat(Assign) {
                        self.skip_trivia(state);
                        self.parse_expression(state, 0);
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

    /// Parses the body of a class definition.
    fn parse_class_def_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::PythonTokenType::*;
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

    /// Parses the body of an if statement.
    fn parse_if_stmt_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::{lexer::PythonTokenType::*, parser::element_type::PythonElementType as ET};
        state.expect(IfKeyword).ok();
        self.skip_trivia(state);
        PrattParser::parse(state, 0, self);
        self.skip_trivia(state);
        state.expect(Colon).ok();
        self.parse_suite(state)?;
        self.skip_trivia(state);

        // Handle newlines before elif/else
        let mut lookahead = 0;
        while let Some(kind) = state.peek_kind_at(lookahead) {
            if kind == Newline || kind.is_trivia() {
                lookahead += 1;
                continue;
            }
            if kind == ElifKeyword || kind == ElseKeyword {
                // Consume the newlines and trivia
                for _ in 0..lookahead {
                    state.bump()
                }
                break;
            }
            break;
        }

        while state.at(ElifKeyword) {
            state.incremental_node(ET::If.into(), |state| {
                state.expect(ElifKeyword).ok();
                self.skip_trivia(state);
                self.parse_expression(state, 0);
                self.skip_trivia(state);
                state.expect(Colon).ok();
                self.parse_suite(state)?;
                self.skip_trivia(state);

                // Peek for more elif/else after newlines
                let mut lookahead = 0;
                while let Some(kind) = state.peek_kind_at(lookahead) {
                    if kind == Newline || kind.is_trivia() {
                        lookahead += 1;
                        continue;
                    }
                    if kind == ElifKeyword || kind == ElseKeyword {
                        for _ in 0..lookahead {
                            state.bump()
                        }
                        break;
                    }
                    break;
                }
                Ok(())
            })?
        }

        if state.eat(ElseKeyword) {
            self.skip_trivia(state);
            state.expect(Colon).ok();
            self.parse_suite(state)?
        }
        Ok(())
    }

    /// Parses the body of a while statement.
    fn parse_while_stmt_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::PythonTokenType::*;
        state.expect(WhileKeyword).ok();
        self.skip_trivia(state);
        PrattParser::parse(state, 0, self);
        self.skip_trivia(state);
        state.expect(Colon).ok();
        self.parse_suite(state)?;
        self.skip_trivia(state);

        // Handle newlines before else
        let mut lookahead = 0;
        while let Some(kind) = state.peek_kind_at(lookahead) {
            if kind == Newline || kind.is_trivia() {
                lookahead += 1;
                continue;
            }
            if kind == ElseKeyword {
                for _ in 0..lookahead {
                    state.bump()
                }
                break;
            }
            break;
        }

        if state.eat(ElseKeyword) {
            self.skip_trivia(state);
            state.expect(Colon).ok();
            self.parse_suite(state)?
        }
        Ok(())
    }

    /// Parses the body of a for statement.
    fn parse_for_stmt_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::PythonTokenType::*;
        state.expect(ForKeyword).ok();
        self.skip_trivia(state);

        // Use a higher precedence to stop at 'in'
        let target_cp = state.checkpoint();
        self.parse_expression(state, 6);

        if state.at(Comma) {
            while state.eat(Comma) {
                self.skip_trivia(state);
                if state.at(InKeyword) {
                    break;
                }
                self.parse_expression(state, 6);
                self.skip_trivia(state);
            }
            state.finish_at(target_cp, crate::parser::element_type::PythonElementType::Tuple);
        }

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
            self.parse_suite(state)?
        }
        Ok(())
    }

    /// Parses the body of a try statement.
    fn parse_try_stmt_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::{lexer::PythonTokenType::*, parser::element_type::PythonElementType as ET};
        state.expect(TryKeyword).ok();
        self.skip_trivia(state);
        state.expect(Colon).ok();
        self.parse_suite(state)?;
        self.skip_trivia(state);

        // Handle newlines before except/else/finally
        let mut lookahead = 0;
        while let Some(kind) = state.peek_kind_at(lookahead) {
            if kind == Newline || kind.is_trivia() {
                lookahead += 1;
                continue;
            }
            if kind == ExceptKeyword || kind == ElseKeyword || kind == FinallyKeyword {
                for _ in 0..lookahead {
                    state.bump()
                }
                break;
            }
            break;
        }

        while state.at(ExceptKeyword) {
            state.incremental_node(ET::ExceptHandler.into(), |state| {
                state.expect(ExceptKeyword).ok();
                self.skip_trivia(state);
                if !state.at(Colon) {
                    self.parse_expression(state, 0); // Exception type
                    self.skip_trivia(state);
                    if state.eat(AsKeyword) {
                        self.skip_trivia(state);
                        state.expect(Identifier).ok();
                        self.skip_trivia(state)
                    }
                }
                self.skip_trivia(state);
                state.expect(Colon).ok();
                self.parse_suite(state)?;
                self.skip_trivia(state);

                // Peek for more except/else/finally after newlines
                let mut lookahead = 0;
                while let Some(kind) = state.peek_kind_at(lookahead) {
                    if kind == Newline || kind.is_trivia() {
                        lookahead += 1;
                        continue;
                    }
                    if kind == ExceptKeyword || kind == ElseKeyword || kind == FinallyKeyword {
                        for _ in 0..lookahead {
                            state.bump()
                        }
                        break;
                    }
                    break;
                }
                Ok(())
            })?
        }

        if state.eat(ElseKeyword) {
            self.skip_trivia(state);
            state.expect(Colon).ok();
            self.parse_suite(state)?;
            self.skip_trivia(state);

            // Peek for finally after newlines
            let mut lookahead = 0;
            while let Some(kind) = state.peek_kind_at(lookahead) {
                if kind == Newline || kind.is_trivia() {
                    lookahead += 1;
                    continue;
                }
                if kind == FinallyKeyword {
                    for _ in 0..lookahead {
                        state.bump()
                    }
                    break;
                }
                break;
            }
        }

        if state.eat(FinallyKeyword) {
            self.skip_trivia(state);
            state.expect(Colon).ok();
            self.parse_suite(state)?
        }
        Ok(())
    }

    /// Parses the body of a with statement.
    fn parse_with_stmt_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::{lexer::PythonTokenType::*, parser::element_type::PythonElementType as ET};
        state.expect(WithKeyword).ok();
        self.skip_trivia(state);
        loop {
            state.incremental_node(ET::WithItem.into(), |state| {
                PrattParser::parse(state, 0, self);
                self.skip_trivia(state);
                if state.eat(AsKeyword) {
                    self.skip_trivia(state);
                    PrattParser::parse(state, 0, self);
                    self.skip_trivia(state)
                }
                Ok(())
            })?;
            if !state.eat(Comma) {
                break;
            }
            self.skip_trivia(state)
        }
        state.expect(Colon).ok();
        self.parse_suite(state)?;
        Ok(())
    }

    /// Parses the body of a raise statement.
    fn parse_raise_stmt_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::PythonTokenType::*;
        state.expect(RaiseKeyword).ok();
        self.skip_trivia(state);
        if !state.at(Newline) && !state.at(Semicolon) && state.not_at_end() {
            PrattParser::parse(state, 0, self);
            self.skip_trivia(state);
            if state.eat(FromKeyword) {
                self.skip_trivia(state);
                PrattParser::parse(state, 0, self);
            }
        }
        self.skip_trivia(state);
        state.eat(Newline);
        Ok(())
    }

    /// Parses the body of an assert statement.
    fn parse_assert_stmt_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::PythonTokenType::*;
        state.expect(AssertKeyword).ok();
        self.skip_trivia(state);
        PrattParser::parse(state, 0, self);
        self.skip_trivia(state);
        if state.eat(Comma) {
            self.skip_trivia(state);
            PrattParser::parse(state, 0, self);
        }
        self.skip_trivia(state);
        state.eat(Newline);
        Ok(())
    }

    /// Parses the body of a delete statement.
    fn parse_del_stmt_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::PythonTokenType::*;
        state.expect(DelKeyword).ok();
        self.skip_trivia(state);
        PrattParser::parse(state, 0, self);
        self.skip_trivia(state);
        state.eat(Newline);
        Ok(())
    }

    /// Parses the body of a global statement.
    fn parse_global_stmt_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::PythonTokenType::*;
        state.expect(GlobalKeyword).ok();
        self.skip_trivia(state);
        loop {
            state.expect(Identifier).ok();
            self.skip_trivia(state);
            if !state.eat(Comma) {
                break;
            }
            self.skip_trivia(state)
        }
        state.eat(Newline);
        Ok(())
    }

    /// Parses the body of a nonlocal statement.
    fn parse_nonlocal_stmt_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::PythonTokenType::*;
        state.expect(NonlocalKeyword).ok();
        self.skip_trivia(state);
        loop {
            state.expect(Identifier).ok();
            self.skip_trivia(state);
            if !state.eat(Comma) {
                break;
            }
            self.skip_trivia(state)
        }
        state.eat(Newline);
        Ok(())
    }

    /// Parses the body of a return statement.
    fn parse_return_stmt_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::PythonTokenType::*;
        self.skip_trivia(state);
        if state.not_at_end() && !state.at(Newline) && !state.at(Semicolon) {
            self.parse_expression(state, 0);
        }
        Ok(())
    }

    /// Parses the body of an import statement.
    fn parse_import_stmt_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::PythonTokenType::*;
        if state.at(ImportKeyword) {
            state.bump();
            self.skip_trivia(state);
            loop {
                state
                    .incremental_node(crate::parser::element_type::PythonElementType::Alias.into(), |state| {
                        state.expect(Identifier).ok();
                        self.skip_trivia(state);
                        if state.eat(AsKeyword) {
                            self.skip_trivia(state);
                            state.expect(Identifier).ok();
                            self.skip_trivia(state)
                        }
                        Ok(())
                    })
                    .ok();
                if !state.eat(Comma) {
                    break;
                }
                self.skip_trivia(state)
            }
        }
        else if state.at(FromKeyword) {
            state.bump();
            self.skip_trivia(state);
            // Module name (can be dotted)
            while state.at(Identifier) || state.at(Dot) {
                state.bump();
                self.skip_trivia(state)
            }
            state.expect(ImportKeyword).ok();
            self.skip_trivia(state);
            if state.eat(Star) {
                // from module import *
            }
            else {
                loop {
                    state
                        .incremental_node(crate::parser::element_type::PythonElementType::Alias.into(), |state| {
                            state.expect(Identifier).ok();
                            self.skip_trivia(state);
                            if state.eat(AsKeyword) {
                                self.skip_trivia(state);
                                state.expect(Identifier).ok();
                                self.skip_trivia(state)
                            }
                            Ok(())
                        })
                        .ok();
                    if !state.eat(Comma) {
                        break;
                    }
                    self.skip_trivia(state)
                }
            }
        }
        state.eat(Newline);
        Ok(())
    }

    /// Parses a suite (a block of statements).
    fn parse_suite<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::PythonTokenType::*;
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
            ()
        }
        else {
            self.parse_statement(state)?
        }
        state.finish_at(cp, crate::parser::element_type::PythonElementType::Suite);
        Ok(())
    }

    /// Internal method to parse the root module.
    fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, PythonLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() && !state.at(PythonTokenType::Eof) {
            let start_index = state.tokens.index();
            self.parse_statement(state)?;

            // Safety: if parse_statement didn't advance, we must advance to avoid infinite loop
            if state.tokens.index() == start_index && state.not_at_end() {
                state.bump()
            }
        }

        if state.at(PythonTokenType::Eof) {
            state.bump()
        }

        Ok(state.finish_at(checkpoint, crate::parser::element_type::PythonElementType::ExpressionModule))
    }

    /// Parses a comprehension (list, set, dict, or generator).
    fn parse_comprehension<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        use crate::{lexer::PythonTokenType::*, parser::element_type::PythonElementType as ET};
        while state.at(ForKeyword) || state.at(AsyncKeyword) {
            state
                .incremental_node(ET::Comprehension.into(), |state| {
                    state.eat(AsyncKeyword);
                    self.skip_trivia(state);
                    state.expect(ForKeyword).ok();
                    self.skip_trivia(state);
                    self.parse_expression(state, 6); // target
                    self.skip_trivia(state);
                    state.expect(InKeyword).ok();
                    self.skip_trivia(state);
                    self.parse_expression(state, 6); // iter
                    self.skip_trivia(state);
                    while state.at(IfKeyword) {
                        state.bump();
                        self.skip_trivia(state);
                        self.parse_expression(state, 6); // if condition
                        self.skip_trivia(state)
                    }
                    Ok(())
                })
                .ok();
            self.skip_trivia(state)
        }
    }
}

impl<'config> Pratt<PythonLanguage> for PythonParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, PythonLanguage> {
        use crate::{lexer::PythonTokenType::*, parser::element_type::PythonElementType as ET};
        self.skip_trivia(state);

        println!("primary at {}: {:?}", state.current_offset(), state.peek_kind());

        let cp = state.checkpoint();
        let kind = state.peek_kind();
        match kind {
            Some(Identifier) => {
                state.bump();
                state.finish_at(cp, ET::Name)
            }
            Some(Number) | Some(String) | Some(Bytes) | Some(TrueKeyword) | Some(FalseKeyword) | Some(NoneKeyword) => {
                state.bump();
                state.finish_at(cp, ET::Constant)
            }
            Some(FString) => {
                state.bump();
                state.finish_at(cp, ET::JoinedStr)
            }
            Some(LeftParen) => {
                state.bump();
                self.skip_trivia(state);
                if state.eat(RightParen) {
                    state.finish_at(cp, ET::Tuple)
                }
                else {
                    self.parse_expression(state, 0);
                    self.skip_trivia(state);
                    if state.at(Comma) {
                        while state.eat(Comma) {
                            self.skip_trivia(state);
                            if state.at(RightParen) {
                                break;
                            }
                            self.parse_expression(state, 0);
                            self.skip_trivia(state);
                        }
                        state.expect(RightParen).ok();
                        state.finish_at(cp, ET::Tuple)
                    }
                    else {
                        state.expect(RightParen).ok();
                        state.finish_at(cp, ET::Expr)
                    }
                }
            }
            Some(LeftBracket) => {
                state.bump();
                self.skip_trivia(state);
                if state.eat(RightBracket) {
                    state.finish_at(cp, ET::List)
                }
                else {
                    self.parse_expression(state, 0);
                    self.skip_trivia(state);
                    if state.at(ForKeyword) || state.at(AsyncKeyword) {
                        self.parse_comprehension(state);
                        state.expect(RightBracket).ok();
                        state.finish_at(cp, ET::ListComp)
                    }
                    else {
                        while state.eat(Comma) {
                            self.skip_trivia(state);
                            if state.at(RightBracket) {
                                break;
                            }
                            self.parse_expression(state, 0);
                            self.skip_trivia(state);
                        }
                        state.expect(RightBracket).ok();
                        state.finish_at(cp, ET::List)
                    }
                }
            }
            Some(LeftBrace) => {
                state.bump();
                self.skip_trivia(state);
                if state.eat(RightBrace) {
                    state.finish_at(cp, ET::Dict)
                }
                else {
                    self.parse_expression(state, 0);
                    self.skip_trivia(state);
                    if state.eat(Colon) {
                        self.skip_trivia(state);
                        self.parse_expression(state, 0);
                        self.skip_trivia(state);
                        if state.at(ForKeyword) || state.at(AsyncKeyword) {
                            self.parse_comprehension(state);
                            state.expect(RightBrace).ok();
                            state.finish_at(cp, ET::DictComp)
                        }
                        else {
                            while state.eat(Comma) {
                                self.skip_trivia(state);
                                if state.at(RightBrace) {
                                    break;
                                }
                                self.parse_expression(state, 0);
                                self.skip_trivia(state);
                                state.expect(Colon).ok();
                                self.skip_trivia(state);
                                self.parse_expression(state, 0);
                                self.skip_trivia(state);
                            }
                            state.expect(RightBrace).ok();
                            state.finish_at(cp, ET::Dict)
                        }
                    }
                    else if state.at(ForKeyword) || state.at(AsyncKeyword) {
                        self.parse_comprehension(state);
                        state.expect(RightBrace).ok();
                        state.finish_at(cp, ET::SetComp)
                    }
                    else {
                        while state.eat(Comma) {
                            self.skip_trivia(state);
                            if state.at(RightBrace) {
                                break;
                            }
                            self.parse_expression(state, 0);
                            self.skip_trivia(state);
                        }
                        state.expect(RightBrace).ok();
                        state.finish_at(cp, ET::Set)
                    }
                }
            }
            Some(Plus) | Some(Minus) | Some(Tilde) | Some(NotKeyword) => {
                state.bump();
                self.skip_trivia(state);
                self.parse_expression(state, 12);
                state.finish_at(cp, ET::UnaryOp)
            }
            Some(LambdaKeyword) => {
                state.bump();
                self.skip_trivia(state);
                state
                    .incremental_node(ET::Arguments.into(), |state| {
                        while state.not_at_end() && !state.at(Colon) {
                            state.incremental_node(ET::Arg.into(), |state| {
                                state.expect(Identifier).ok();
                                self.skip_trivia(state);
                                if state.eat(Assign) {
                                    self.skip_trivia(state);
                                    self.parse_expression(state, 0);
                                }
                                Ok(())
                            })?;
                            self.skip_trivia(state);
                            if !state.eat(Comma) {
                                break;
                            }
                            self.skip_trivia(state);
                        }
                        Ok(())
                    })
                    .ok();
                state.expect(Colon).ok();
                self.skip_trivia(state);
                self.parse_expression(state, 0);
                state.finish_at(cp, ET::Lambda)
            }
            Some(YieldKeyword) => {
                state.bump();
                self.skip_trivia(state);
                if state.eat(FromKeyword) {
                    self.skip_trivia(state);
                    self.parse_expression(state, 0);
                    state.finish_at(cp, ET::YieldFrom)
                }
                else {
                    if !state.at(Newline) && !state.at(RightParen) && !state.at(RightBracket) && !state.at(RightBrace) && !state.at(Comma) && !state.at(Colon) {
                        self.parse_expression(state, 0);
                    }
                    state.finish_at(cp, ET::Yield)
                }
            }
            _ => {
                state.bump();
                state.finish_at(cp, ET::Error)
            }
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, PythonLanguage> {
        use crate::lexer::token_type::PythonTokenType;
        self.skip_trivia(state);
        let kind = state.peek_kind().expect("Expected token in prefix");
        match kind {
            PythonTokenType::Plus | PythonTokenType::Minus | PythonTokenType::Tilde | PythonTokenType::NotKeyword => {
                let cp = state.checkpoint();
                state.expect(kind).ok();
                PrattParser::parse(state, 14, self);
                state.finish_at(cp, crate::parser::element_type::PythonElementType::UnaryOp)
            }
            _ => self.primary(state),
        }
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, _left: &'a GreenNode<'a, PythonLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, PythonLanguage>> {
        use crate::{lexer::PythonTokenType::*, parser::element_type::PythonElementType as ET};

        // Peek kind without consuming trivia yet
        let mut lookahead = 0;
        let mut kind = None;
        while let Some(k) = state.peek_kind_at(lookahead) {
            if k.is_trivia() {
                lookahead += 1;
                continue;
            }
            kind = Some(k);
            break;
        }

        let kind = kind?;
        let (prec, assoc) = match kind {
            PythonTokenType::Assign
            | PythonTokenType::PlusAssign
            | PythonTokenType::MinusAssign
            | PythonTokenType::StarAssign
            | PythonTokenType::DoubleStarAssign
            | PythonTokenType::SlashAssign
            | PythonTokenType::DoubleSlashAssign
            | PythonTokenType::PercentAssign
            | PythonTokenType::AtAssign
            | PythonTokenType::AmpersandAssign
            | PythonTokenType::PipeAssign
            | PythonTokenType::CaretAssign
            | PythonTokenType::LeftShiftAssign
            | PythonTokenType::RightShiftAssign => (1, Associativity::Right),
            PythonTokenType::OrKeyword => (2, Associativity::Left),
            PythonTokenType::AndKeyword => (3, Associativity::Left),
            PythonTokenType::NotKeyword => (4, Associativity::Left),
            PythonTokenType::Less | PythonTokenType::Greater | PythonTokenType::Equal | PythonTokenType::NotEqual | PythonTokenType::LessEqual | PythonTokenType::GreaterEqual | PythonTokenType::InKeyword | PythonTokenType::IsKeyword => {
                (5, Associativity::Left)
            }
            PythonTokenType::Pipe => (6, Associativity::Left),
            PythonTokenType::Caret => (7, Associativity::Left),
            PythonTokenType::Ampersand => (8, Associativity::Left),
            PythonTokenType::LeftShift | PythonTokenType::RightShift => (9, Associativity::Left),
            PythonTokenType::Plus | PythonTokenType::Minus => (10, Associativity::Left),
            PythonTokenType::Star | PythonTokenType::Slash | PythonTokenType::DoubleSlash | PythonTokenType::Percent | PythonTokenType::At => (11, Associativity::Left),
            PythonTokenType::DoubleStar => (13, Associativity::Right),
            PythonTokenType::Dot | PythonTokenType::LeftParen | PythonTokenType::LeftBracket => (15, Associativity::Left),
            _ => return None,
        };

        if prec < min_precedence {
            return None;
        }

        let cp = state.checkpoint_before(_left);
        for _ in 0..lookahead {
            state.bump();
        }

        match kind {
            LeftParen => {
                state.bump(); // Consume LeftParen

                while state.not_at_end() && !state.at(RightParen) {
                    self.skip_trivia(state);
                    if state.at(RightParen) {
                        break;
                    }

                    let mut lookahead = 0;
                    let mut is_keyword = false;
                    if state.at(Identifier) {
                        lookahead += 1;
                        while let Some(k) = state.peek_kind_at(lookahead) {
                            if k.is_trivia() {
                                lookahead += 1;
                                continue;
                            }
                            if k == Assign {
                                is_keyword = true;
                            }
                            break;
                        }
                    }

                    if is_keyword {
                        state
                            .incremental_node(ET::Keyword.into(), |state| {
                                state.expect(Identifier).ok();
                                self.skip_trivia(state);
                                state.expect(Assign).ok();
                                self.skip_trivia(state);
                                self.parse_expression(state, 0);
                                Ok(())
                            })
                            .ok()?;
                    }
                    else if state.at(DoubleStar) {
                        state
                            .incremental_node(ET::Keyword.into(), |state| {
                                state.bump(); // DoubleStar
                                self.skip_trivia(state);
                                self.parse_expression(state, 0);
                                Ok(())
                            })
                            .ok()?;
                    }
                    else if state.at(Star) {
                        state
                            .incremental_node(ET::Starred.into(), |state| {
                                state.bump(); // Star
                                self.skip_trivia(state);
                                self.parse_expression(state, 0);
                                Ok(())
                            })
                            .ok()?;
                    }
                    else {
                        self.parse_expression(state, 0);
                    }

                    self.skip_trivia(state);
                    if !state.eat(Comma) {
                        break;
                    }
                }

                self.skip_trivia(state);
                state.expect(RightParen).ok();
                Some(state.finish_at(cp, ET::Call))
            }
            LeftBracket => {
                state.bump(); // Consume LeftBracket

                let cp_slice = state.checkpoint();
                let mut is_slice = false;

                if state.at(Colon) {
                    is_slice = true;
                }
                else {
                    self.parse_expression(state, 0);
                    self.skip_trivia(state);
                    if state.at(Colon) {
                        is_slice = true;
                    }
                }

                if is_slice {
                    // It's a slice
                    while state.at(Colon) {
                        state.bump();
                        self.skip_trivia(state);
                        if !state.at(Colon) && !state.at(RightBracket) {
                            self.parse_expression(state, 0);
                            self.skip_trivia(state);
                        }
                    }
                    state.finish_at(cp_slice, ET::Slice);
                }

                state.expect(RightBracket).ok();
                Some(state.finish_at(cp, ET::Subscript))
            }
            Dot => {
                state.bump(); // Consume Dot
                self.skip_trivia(state);
                state.expect(Identifier).ok();
                Some(state.finish_at(cp, ET::Attribute))
            }
            _ => {
                let result_kind = if prec == 1 {
                    ET::AssignStmt
                }
                else if prec <= 3 {
                    ET::BoolOp
                }
                else if prec == 5 {
                    ET::Compare
                }
                else {
                    ET::BinOp
                };

                state.bump(); // Consume operator

                let next_prec = match assoc {
                    Associativity::Left => prec + 1,
                    Associativity::Right => prec,
                    Associativity::None => prec + 1,
                };

                self.parse_expression(state, next_prec);
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
