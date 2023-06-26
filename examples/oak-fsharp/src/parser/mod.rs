/// Element type module
pub mod element_type;

use crate::{
    language::FSharpLanguage,
    lexer::{FSharpLexer, token_type::FSharpTokenType},
    parser::element_type::FSharpElementType,
};
use oak_core::{
    GreenNode, OakError,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, FSharpLanguage, S>;

/// F# parser
pub struct FSharpParser<'config> {
    pub(crate) config: &'config FSharpLanguage,
}

impl<'config> FSharpParser<'config> {
    /// Creates a new FSharpParser
    pub fn new(config: &'config FSharpLanguage) -> Self {
        Self { config }
    }

    fn parse_namespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(FSharpTokenType::Namespace)?;

        // Parse namespace name (e.g. System.Collections)
        while state.not_at_end() && state.at(FSharpTokenType::Identifier) {
            state.bump();
            if state.at(FSharpTokenType::Dot) { state.bump() } else { break }
        }

        state.finish_at(checkpoint, FSharpElementType::Namespace);
        Ok(())
    }

    fn parse_module<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(FSharpTokenType::Module)?;

        if state.at(FSharpTokenType::Identifier) {
            state.bump()
        }

        state.finish_at(checkpoint, FSharpElementType::Module);
        Ok(())
    }

    fn parse_open<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(FSharpTokenType::Open)?;

        // Parse namespace/module name to open
        while state.not_at_end() && state.at(FSharpTokenType::Identifier) {
            state.bump();
            if state.at(FSharpTokenType::Dot) { state.bump() } else { break }
        }

        state.finish_at(checkpoint, FSharpElementType::Open);
        Ok(())
    }

    fn parse_binding<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(FSharpTokenType::Let)?;

        if state.eat(FSharpTokenType::Rec) {
            // optional rec
        }

        // Name
        state.expect(FSharpTokenType::Identifier)?;

        // Parameters (optional)
        while state.not_at_end() && state.at(FSharpTokenType::Identifier) {
            state.bump()
        }

        // Equals
        state.expect(FSharpTokenType::Equal)?;

        // Expression
        self.parse_expression(state)?;

        state.finish_at(checkpoint, FSharpElementType::Let);
        Ok(())
    }

    fn parse_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        let kind = state.peek_kind();
        match kind {
            Some(FSharpTokenType::If) => {
                state.expect(FSharpTokenType::If)?;
                self.parse_expression(state)?;
                state.expect(FSharpTokenType::Then)?;
                self.parse_expression(state)?;
                if state.eat(FSharpTokenType::Else) {
                    self.parse_expression(state)?
                }
                state.finish_at(checkpoint, FSharpElementType::If);
            }
            Some(FSharpTokenType::Match) => {
                self.parse_match_expression(state, checkpoint)?;
            }
            Some(FSharpTokenType::Let) => {
                self.parse_let_expression(state, checkpoint)?;
            }
            Some(FSharpTokenType::Fun) => {
                self.parse_lambda_expression(state, checkpoint)?;
            }
            Some(FSharpTokenType::Function) => {
                self.parse_function_expression(state, checkpoint)?;
            }
            Some(FSharpTokenType::Open) => {
                self.parse_open(state)?;
            }
            Some(FSharpTokenType::LeftParen) => {
                self.parse_parenthesized_expression(state, checkpoint)?;
            }
            Some(FSharpTokenType::LeftBracket) => {
                self.parse_list_or_array_expression(state, checkpoint)?;
            }
            Some(FSharpTokenType::LeftBrace) => {
                self.parse_record_expression(state, checkpoint)?;
            }
            Some(FSharpTokenType::Identifier) => {
                self.parse_identifier_expression(state, checkpoint)?;
            }
            Some(FSharpTokenType::IntegerLiteral) | Some(FSharpTokenType::FloatLiteral) | Some(FSharpTokenType::StringLiteral) | Some(FSharpTokenType::CharLiteral) => {
                self.parse_literal_expression(state, checkpoint)?;
            }
            _ => {
                // Simple expression: consume until end of line or specific delimiters
                while state.not_at_end() {
                    let kind = state.peek_kind();
                    if matches!(kind, Some(FSharpTokenType::Newline | FSharpTokenType::Then | FSharpTokenType::Else | FSharpTokenType::In | FSharpTokenType::PipeRight | FSharpTokenType::PipeGreater)) {
                        break;
                    }
                    state.bump()
                }
                // If we didn't consume anything, just bump one to avoid infinite loop
                if state.checkpoint().0 == checkpoint.0 {
                    state.bump()
                }
                state.finish_at(checkpoint, FSharpElementType::Expression);
            }
        }
        Ok(())
    }

    fn parse_function_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, checkpoint: (usize, usize)) -> Result<(), OakError> {
        state.expect(FSharpTokenType::Function)?;

        // Parse function parameters with optional type annotations
        while state.not_at_end() && state.at(FSharpTokenType::Identifier) {
            state.bump();
            // Check for type annotation
            if state.eat(FSharpTokenType::Colon) {
                self.parse_type(state)?;
            }
        }

        // Parse match cases
        while state.not_at_end() && !state.at(FSharpTokenType::End) {
            self.parse_match_case(state)?;
        }

        state.eat(FSharpTokenType::End);
        state.finish_at(checkpoint, FSharpElementType::Function);
        Ok(())
    }

    fn parse_match_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, checkpoint: (usize, usize)) -> Result<(), OakError> {
        state.expect(FSharpTokenType::Match)?;
        self.parse_expression(state)?;
        state.expect(FSharpTokenType::With)?;

        while state.not_at_end() && !state.at(FSharpTokenType::End) {
            self.parse_match_case(state)?;
        }

        state.eat(FSharpTokenType::End);
        state.finish_at(checkpoint, FSharpElementType::Match);
        Ok(())
    }

    fn parse_match_case<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();

        // Parse pattern
        self.parse_pattern(state)?;

        // Optional guard
        if state.eat(FSharpTokenType::When) {
            self.parse_expression(state)?;
        }

        state.expect(FSharpTokenType::Arrow)?;
        self.parse_expression(state)?;

        state.finish_at(checkpoint, FSharpElementType::MatchCase);
        Ok(())
    }

    fn parse_pattern<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        let kind = state.peek_kind();

        match kind {
            Some(FSharpTokenType::Underscore) => {
                state.bump();
                state.finish_at(checkpoint, FSharpElementType::WildcardPattern);
            }
            Some(FSharpTokenType::Identifier) => {
                state.bump();
                // Check for active pattern
                if state.at(FSharpTokenType::LeftParen) {
                    state.bump();
                    // Parse active pattern parameters
                    while state.not_at_end() && !state.at(FSharpTokenType::RightParen) {
                        self.parse_expression(state)?;
                        if state.at(FSharpTokenType::Comma) {
                            state.bump();
                        }
                    }
                    state.expect(FSharpTokenType::RightParen)?;
                    state.finish_at(checkpoint, FSharpElementType::ActivePattern);
                }
                else if state.at(FSharpTokenType::Pipe) {
                    // Check for partial active pattern
                    state.bump();
                    if state.at(FSharpTokenType::Identifier) {
                        state.bump();
                    }
                    state.finish_at(checkpoint, FSharpElementType::ActivePattern);
                }
                else {
                    state.finish_at(checkpoint, FSharpElementType::IdentifierPattern);
                }
            }
            Some(FSharpTokenType::LeftParen) => {
                state.bump();
                while state.not_at_end() && !state.at(FSharpTokenType::RightParen) {
                    self.parse_pattern(state)?;
                    if state.at(FSharpTokenType::Comma) {
                        state.bump();
                    }
                }
                state.expect(FSharpTokenType::RightParen)?;
                state.finish_at(checkpoint, FSharpElementType::TuplePattern);
            }
            Some(FSharpTokenType::LeftBracket) => {
                state.bump();
                while state.not_at_end() && !state.at(FSharpTokenType::RightBracket) {
                    self.parse_pattern(state)?;
                    if state.at(FSharpTokenType::Comma) {
                        state.bump();
                    }
                }
                state.expect(FSharpTokenType::RightBracket)?;
                state.finish_at(checkpoint, FSharpElementType::ListPattern);
            }
            Some(FSharpTokenType::Pipe) => {
                // Parse partial active pattern case
                state.bump();
                if state.at(FSharpTokenType::Identifier) {
                    state.bump();
                }
                state.finish_at(checkpoint, FSharpElementType::ActivePattern);
            }
            _ => {
                state.bump();
                state.finish_at(checkpoint, FSharpElementType::Pattern);
            }
        }
        Ok(())
    }

    fn parse_let_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, checkpoint: (usize, usize)) -> Result<(), OakError> {
        state.expect(FSharpTokenType::Let)?;

        let is_rec = state.eat(FSharpTokenType::Rec);

        // Parse binding name
        state.expect(FSharpTokenType::Identifier)?;

        // Parse parameters
        while state.not_at_end() && state.at(FSharpTokenType::Identifier) {
            state.bump();
        }

        // Optional type annotation
        if state.eat(FSharpTokenType::Colon) {
            self.parse_type(state)?;
        }

        state.expect(FSharpTokenType::Equal)?;
        self.parse_expression(state)?;

        if state.eat(FSharpTokenType::In) {
            self.parse_expression(state)?;
        }

        state.finish_at(checkpoint, FSharpElementType::Let);
        Ok(())
    }

    fn parse_lambda_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, checkpoint: (usize, usize)) -> Result<(), OakError> {
        state.expect(FSharpTokenType::Fun)?;

        // Parse parameters
        while state.not_at_end() && state.at(FSharpTokenType::Identifier) {
            state.bump();
        }

        state.expect(FSharpTokenType::Arrow)?;
        self.parse_expression(state)?;

        state.finish_at(checkpoint, FSharpElementType::Lambda);
        Ok(())
    }

    fn parse_parenthesized_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, checkpoint: (usize, usize)) -> Result<(), OakError> {
        state.bump(); // LeftParen
        self.parse_expression(state)?;
        state.expect(FSharpTokenType::RightParen)?;
        state.finish_at(checkpoint, FSharpElementType::Parenthesized);
        Ok(())
    }

    fn parse_list_or_array_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, checkpoint: (usize, usize)) -> Result<(), OakError> {
        state.bump(); // LeftBracket

        while state.not_at_end() && !state.at(FSharpTokenType::RightBracket) {
            self.parse_expression(state)?;
            if state.at(FSharpTokenType::Comma) {
                state.bump();
            }
        }

        state.expect(FSharpTokenType::RightBracket)?;
        state.finish_at(checkpoint, FSharpElementType::List);
        Ok(())
    }

    fn parse_record_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, checkpoint: (usize, usize)) -> Result<(), OakError> {
        state.bump(); // LeftBrace

        while state.not_at_end() && !state.at(FSharpTokenType::RightBrace) {
            state.expect(FSharpTokenType::Identifier)?;
            state.expect(FSharpTokenType::Equal)?;
            self.parse_expression(state)?;
            if state.at(FSharpTokenType::Comma) {
                state.bump();
            }
        }

        state.expect(FSharpTokenType::RightBrace)?;
        state.finish_at(checkpoint, FSharpElementType::Record);
        Ok(())
    }

    fn parse_identifier_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, checkpoint: (usize, usize)) -> Result<(), OakError> {
        state.bump(); // Identifier

        // Check for function application
        if state.not_at_end() && !matches!(state.peek_kind(), Some(FSharpTokenType::Newline | FSharpTokenType::Then | FSharpTokenType::Else | FSharpTokenType::In | FSharpTokenType::PipeRight | FSharpTokenType::PipeGreater)) {
            self.parse_expression(state)?;
            state.finish_at(checkpoint, FSharpElementType::Application);
        }
        else {
            state.finish_at(checkpoint, FSharpElementType::Identifier);
        }
        Ok(())
    }

    fn parse_literal_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, checkpoint: (usize, usize)) -> Result<(), OakError> {
        state.bump(); // Literal
        state.finish_at(checkpoint, FSharpElementType::Literal);
        Ok(())
    }

    fn parse_type<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        // Simple type parsing
        while state.not_at_end() && !matches!(state.peek_kind(), Some(FSharpTokenType::Equal | FSharpTokenType::Newline)) {
            state.bump();
        }
        Ok(())
    }
}

impl<'config> Parser<FSharpLanguage> for FSharpParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<FSharpLanguage>) -> ParseOutput<'a, FSharpLanguage> {
        let lexer = FSharpLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let cp = (0, 0); // Ensure the root node includes initial trivia skipped during state initialization
            while state.not_at_end() {
                let kind = state.peek_kind();
                match kind {
                    Some(FSharpTokenType::Namespace) => self.parse_namespace(state)?,
                    Some(FSharpTokenType::Module) => self.parse_module(state)?,
                    Some(FSharpTokenType::Open) => self.parse_open(state)?,
                    Some(FSharpTokenType::Let) => self.parse_binding(state)?,
                    _ => state.bump(),
                }
            }

            Ok(state.finish_at(cp, FSharpElementType::Root))
        })
    }
}
