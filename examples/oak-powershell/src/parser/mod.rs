/// Element type definitions.
pub mod element_type;

use crate::{language::PowerShellLanguage, lexer::token_type::PowerShellTokenType, parser::element_type::PowerShellElementType};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

/// Parser for the PowerShell language.
#[derive(Debug)]
pub struct PowerShellParser<'a> {
    /// The language configuration.
    pub config: &'a PowerShellLanguage,
}

impl<'a> PowerShellParser<'a> {
    /// Creates a new `PowerShellParser`.
    pub fn new(config: &'a PowerShellLanguage) -> Self {
        Self { config }
    }

    fn parse_program<'b, S: Source + ?Sized>(&self, state: &mut ParserState<'b, PowerShellLanguage, S>) {
        while state.not_at_end() {
            let kind = state.peek_kind();
            match kind {
                Some(PowerShellTokenType::Function) => self.parse_function(state),
                Some(PowerShellTokenType::Class) => self.parse_class(state),
                Some(PowerShellTokenType::If) => self.parse_if(state),
                Some(PowerShellTokenType::While) => self.parse_while(state),
                Some(PowerShellTokenType::For) => self.parse_for(state),
                Some(PowerShellTokenType::ForEach) => self.parse_foreach(state),
                Some(PowerShellTokenType::Try) => self.parse_try(state),
                Some(PowerShellTokenType::Newline) | Some(PowerShellTokenType::Semicolon) | Some(PowerShellTokenType::Whitespace) => {
                    state.bump();
                }
                _ => self.parse_statement(state),
            }
        }
    }

    fn parse_function<'b, S: Source + ?Sized>(&self, state: &mut ParserState<'b, PowerShellLanguage, S>) {
        let checkpoint = state.checkpoint();
        state.expect(PowerShellTokenType::Function).ok();
        state.expect(PowerShellTokenType::Identifier).ok();
        self.skip_trivia(state);
        if state.at(PowerShellTokenType::LeftBrace) {
            self.parse_block(state);
        }
        state.finish_at(checkpoint, PowerShellElementType::FunctionDef);
    }

    fn parse_class<'b, S: Source + ?Sized>(&self, state: &mut ParserState<'b, PowerShellLanguage, S>) {
        let checkpoint = state.checkpoint();
        state.expect(PowerShellTokenType::Class).ok();
        state.expect(PowerShellTokenType::Identifier).ok();
        self.skip_trivia(state);
        if state.at(PowerShellTokenType::LeftBrace) {
            self.parse_block(state);
        }
        state.finish_at(checkpoint, PowerShellElementType::ClassDef);
    }

    fn parse_if<'b, S: Source + ?Sized>(&self, state: &mut ParserState<'b, PowerShellLanguage, S>) {
        let checkpoint = state.checkpoint();
        state.expect(PowerShellTokenType::If).ok();
        self.skip_trivia(state);
        state.expect(PowerShellTokenType::LeftParen).ok();
        while !state.at(PowerShellTokenType::RightParen) && state.not_at_end() {
            state.bump();
        }
        state.expect(PowerShellTokenType::RightParen).ok();
        self.skip_trivia(state);
        self.parse_block(state);

        loop {
            self.skip_trivia(state);
            if state.at(PowerShellTokenType::ElseIf) {
                state.bump();
                self.skip_trivia(state);
                state.expect(PowerShellTokenType::LeftParen).ok();
                while !state.at(PowerShellTokenType::RightParen) && state.not_at_end() {
                    state.bump();
                }
                state.expect(PowerShellTokenType::RightParen).ok();
                self.skip_trivia(state);
                self.parse_block(state);
            }
            else {
                break;
            }
        }

        self.skip_trivia(state);
        if state.at(PowerShellTokenType::Else) {
            state.bump();
            self.skip_trivia(state);
            self.parse_block(state);
        }

        state.finish_at(checkpoint, PowerShellElementType::IfStatement);
    }

    fn parse_while<'b, S: Source + ?Sized>(&self, state: &mut ParserState<'b, PowerShellLanguage, S>) {
        let checkpoint = state.checkpoint();
        state.expect(PowerShellTokenType::While).ok();
        self.skip_trivia(state);
        state.expect(PowerShellTokenType::LeftParen).ok();
        while !state.at(PowerShellTokenType::RightParen) && state.not_at_end() {
            state.bump();
        }
        state.expect(PowerShellTokenType::RightParen).ok();
        self.skip_trivia(state);
        self.parse_block(state);
        state.finish_at(checkpoint, PowerShellElementType::WhileStatement);
    }

    fn parse_for<'b, S: Source + ?Sized>(&self, state: &mut ParserState<'b, PowerShellLanguage, S>) {
        let checkpoint = state.checkpoint();
        state.expect(PowerShellTokenType::For).ok();
        self.skip_trivia(state);
        state.expect(PowerShellTokenType::LeftParen).ok();
        while !state.at(PowerShellTokenType::RightParen) && state.not_at_end() {
            state.bump();
        }
        state.expect(PowerShellTokenType::RightParen).ok();
        self.skip_trivia(state);
        self.parse_block(state);
        state.finish_at(checkpoint, PowerShellElementType::ForStatement);
    }

    fn parse_foreach<'b, S: Source + ?Sized>(&self, state: &mut ParserState<'b, PowerShellLanguage, S>) {
        let checkpoint = state.checkpoint();
        state.expect(PowerShellTokenType::ForEach).ok();
        self.skip_trivia(state);
        state.expect(PowerShellTokenType::LeftParen).ok();
        while !state.at(PowerShellTokenType::RightParen) && state.not_at_end() {
            state.bump();
        }
        state.expect(PowerShellTokenType::RightParen).ok();
        self.skip_trivia(state);
        self.parse_block(state);
        state.finish_at(checkpoint, PowerShellElementType::ForEachStatement);
    }

    fn parse_try<'b, S: Source + ?Sized>(&self, state: &mut ParserState<'b, PowerShellLanguage, S>) {
        let checkpoint = state.checkpoint();
        state.expect(PowerShellTokenType::Try).ok();
        self.skip_trivia(state);
        self.parse_block(state);

        while state.at(PowerShellTokenType::Catch) {
            let catch_cp = state.checkpoint();
            state.bump();
            self.skip_trivia(state);
            if state.at(PowerShellTokenType::LeftBracket) {
                // Type list
                while !state.at(PowerShellTokenType::RightBracket) && state.not_at_end() {
                    state.bump();
                }
                state.expect(PowerShellTokenType::RightBracket).ok();
                self.skip_trivia(state);
            }
            self.parse_block(state);
            state.finish_at(catch_cp, PowerShellElementType::CatchBlock);
            self.skip_trivia(state);
        }

        if state.at(PowerShellTokenType::Finally) {
            let finally_cp = state.checkpoint();
            state.bump();
            self.skip_trivia(state);
            self.parse_block(state);
            state.finish_at(finally_cp, PowerShellElementType::FinallyBlock);
        }

        state.finish_at(checkpoint, PowerShellElementType::TryStatement);
    }

    fn parse_block<'b, S: Source + ?Sized>(&self, state: &mut ParserState<'b, PowerShellLanguage, S>) {
        state.expect(PowerShellTokenType::LeftBrace).ok();
        while !state.at(PowerShellTokenType::RightBrace) && state.not_at_end() {
            let kind = state.peek_kind();
            match kind {
                Some(PowerShellTokenType::Newline) | Some(PowerShellTokenType::Semicolon) | Some(PowerShellTokenType::Whitespace) => {
                    state.bump();
                }
                _ => self.parse_statement(state),
            }
        }
        state.expect(PowerShellTokenType::RightBrace).ok();
    }

    fn parse_statement<'b, S: Source + ?Sized>(&self, state: &mut ParserState<'b, PowerShellLanguage, S>) {
        let checkpoint = state.checkpoint();

        self.parse_pipeline(state);

        if state.at(PowerShellTokenType::Semicolon) || state.at(PowerShellTokenType::Newline) {
            state.bump();
        }
        state.finish_at(checkpoint, PowerShellElementType::ExpressionStatement);
    }

    fn parse_pipeline<'b, S: Source + ?Sized>(&self, state: &mut ParserState<'b, PowerShellLanguage, S>) {
        let checkpoint = state.checkpoint();
        self.parse_command(state);

        while state.at(PowerShellTokenType::Pipe) {
            state.bump();
            self.skip_trivia(state);
            self.parse_command(state);
        }

        if state.at(PowerShellTokenType::Pipe) {
            state.finish_at(checkpoint, PowerShellElementType::Pipeline);
        }
    }

    fn parse_command<'b, S: Source + ?Sized>(&self, state: &mut ParserState<'b, PowerShellLanguage, S>) {
        let checkpoint = state.checkpoint();

        // Command name or variable
        if state.not_at_end() && !matches!(state.peek_kind(), Some(PowerShellTokenType::Newline | PowerShellTokenType::Semicolon | PowerShellTokenType::Pipe | PowerShellTokenType::RightBrace)) {
            state.bump();
        }

        // Arguments and parameters
        while state.not_at_end() {
            self.skip_trivia(state);
            let kind = state.peek_kind();
            if matches!(kind, Some(PowerShellTokenType::Newline | PowerShellTokenType::Semicolon | PowerShellTokenType::Pipe | PowerShellTokenType::RightBrace)) {
                break;
            }
            state.bump();
        }

        state.finish_at(checkpoint, PowerShellElementType::Command);
    }

    fn skip_trivia<'b, S: Source + ?Sized>(&self, state: &mut ParserState<'b, PowerShellLanguage, S>) {
        while state.at(PowerShellTokenType::Whitespace) || state.at(PowerShellTokenType::Comment) {
            state.bump();
        }
    }
}

impl<'a> Parser<PowerShellLanguage> for PowerShellParser<'a> {
    fn parse<'b, S: Source + ?Sized>(&self, text: &'b S, edits: &[TextEdit], cache: &'b mut impl ParseCache<PowerShellLanguage>) -> ParseOutput<'b, PowerShellLanguage> {
        let lexer = crate::lexer::PowerShellLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            self.parse_program(state);
            Ok(state.finish_at(checkpoint, PowerShellElementType::Root))
        })
    }
}
