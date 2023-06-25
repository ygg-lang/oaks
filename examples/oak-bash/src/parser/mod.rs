#![doc = include_str!("readme.md")]
/// Bash element types and role definitions.
pub mod element_type;

pub use element_type::BashElementType;

use crate::{
    language::BashLanguage,
    lexer::{BashLexer, BashTokenType},
};
use oak_core::{
    OakError, TextEdit,
    parser::{ParseCache, Parser, ParserState},
    source::Source,
};

pub(crate) type State<'a, S> = ParserState<'a, BashLanguage, S>;

/// Parser for the Bash language.
pub struct BashParser<'config> {
    pub(crate) config: &'config BashLanguage,
}

impl<'config> BashParser<'config> {
    /// Creates a new `BashParser` instance.
    pub fn new(config: &'config BashLanguage) -> Self {
        Self { config }
    }

    pub(crate) fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        if state.at(BashTokenType::Keyword) {
            let text = state.peek_text();
            match text.as_deref() {
                Some("if") => self.parse_if(state),
                Some("while") => self.parse_while(state),
                Some("for") => self.parse_for(state),
                Some("function") => self.parse_function(state),
                _ => self.parse_command_or_pipeline(state),
            }
        }
        else if state.at(BashTokenType::Identifier) && state.peek_kind_at(1) == Some(BashTokenType::Delimiter) && state.peek_at(1).map(|t| state.source.get_text_in(t.span)).as_deref() == Some("(") {
            self.parse_function(state)
        }
        else if state.at(BashTokenType::Identifier) && state.peek_kind_at(1) == Some(BashTokenType::Operator) && state.peek_at(1).map(|t| state.source.get_text_in(t.span)).as_deref() == Some("=") {
            self.parse_variable_assignment(state)
        }
        else {
            self.parse_command_or_pipeline(state)
        }
    }

    fn parse_variable_assignment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(BashTokenType::Identifier)?;
        state.expect(BashTokenType::Operator)?; // =

        // Parse the value
        while state.not_at_end() && !state.at(BashTokenType::Newline) && !state.at(BashTokenType::Delimiter) {
            state.bump();
        }

        state.finish_at(checkpoint, BashElementType::VariableAssignment);
        Ok(())
    }

    fn parse_command_or_pipeline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        self.parse_command(state)?;

        if state.peek_text().as_deref() == Some("|") {
            while state.peek_text().as_deref() == Some("|") {
                state.bump();
                self.parse_command(state)?;
            }
            state.finish_at(checkpoint, BashElementType::Pipeline);
        }

        Ok(())
    }

    fn parse_if<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(BashTokenType::Keyword)?; // if

        // Parse condition
        while state.not_at_end() && state.peek_text().as_deref() != Some("then") {
            state.bump();
        }

        if state.peek_text().as_deref() == Some("then") {
            state.bump();
        }

        // Parse body
        while state.not_at_end() {
            let text = state.peek_text();
            if matches!(text.as_deref(), Some("elif" | "else" | "fi")) {
                break;
            }
            self.parse_statement(state).ok();
        }

        while state.peek_text().as_deref() == Some("elif") {
            state.bump();
            while state.not_at_end() && state.peek_text().as_deref() != Some("then") {
                state.bump();
            }
            if state.peek_text().as_deref() == Some("then") {
                state.bump();
            }
            while state.not_at_end() {
                let text = state.peek_text();
                if matches!(text.as_deref(), Some("elif" | "else" | "fi")) {
                    break;
                }
                self.parse_statement(state).ok();
            }
        }

        if state.peek_text().as_deref() == Some("else") {
            state.bump();
            while state.not_at_end() && state.peek_text().as_deref() != Some("fi") {
                self.parse_statement(state).ok();
            }
        }

        if state.peek_text().as_deref() == Some("fi") {
            state.bump();
        }

        state.finish_at(checkpoint, BashElementType::IfStatement);
        Ok(())
    }

    fn parse_while<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(BashTokenType::Keyword)?; // while

        while state.not_at_end() && state.peek_text().as_deref() != Some("do") {
            state.bump();
        }

        if state.peek_text().as_deref() == Some("do") {
            state.bump();
        }

        while state.not_at_end() && state.peek_text().as_deref() != Some("done") {
            self.parse_statement(state).ok();
        }

        if state.peek_text().as_deref() == Some("done") {
            state.bump();
        }

        state.finish_at(checkpoint, BashElementType::WhileStatement);
        Ok(())
    }

    fn parse_for<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(BashTokenType::Keyword)?; // for

        while state.not_at_end() && state.peek_text().as_deref() != Some("do") {
            state.bump();
        }

        if state.peek_text().as_deref() == Some("do") {
            state.bump();
        }

        while state.not_at_end() && state.peek_text().as_deref() != Some("done") {
            self.parse_statement(state).ok();
        }

        if state.peek_text().as_deref() == Some("done") {
            state.bump();
        }

        state.finish_at(checkpoint, BashElementType::ForStatement);
        Ok(())
    }

    fn parse_function<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        if state.peek_text().as_deref() == Some("function") {
            state.bump();
        }

        state.expect(BashTokenType::Identifier).ok();

        if state.peek_text().as_deref() == Some("(") {
            state.bump();
            if state.peek_text().as_deref() == Some(")") {
                state.bump();
            }
        }

        // Bash functions usually followed by a compound command, often a brace group
        if state.peek_text().as_deref() == Some("{") {
            state.bump();
            while state.not_at_end() && state.peek_text().as_deref() != Some("}") {
                self.parse_statement(state).ok();
            }
            if state.peek_text().as_deref() == Some("}") {
                state.bump();
            }
        }
        else {
            self.parse_statement(state).ok();
        }

        state.finish_at(checkpoint, BashElementType::FunctionDefinition);
        Ok(())
    }

    pub(crate) fn parse_command<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        while state.not_at_end() && !state.at(BashTokenType::Newline) && !state.at(BashTokenType::Delimiter) && state.peek_text().as_deref() != Some("|") {
            if matches!(state.peek_text().as_deref(), Some(">" | ">>" | "<" | "<<")) {
                self.parse_redirection(state)?;
            }
            else {
                state.bump();
            }
        }
        state.finish_at(checkpoint, BashElementType::CommandStatement);
        Ok(())
    }

    fn parse_redirection<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.bump(); // The redirection operator

        // Skip whitespace if any
        while state.at(BashTokenType::Whitespace) {
            state.bump();
        }

        // The target file or descriptor
        if state.not_at_end() && !state.at(BashTokenType::Newline) && !state.at(BashTokenType::Delimiter) {
            state.bump();
        }

        state.finish_at(checkpoint, BashElementType::Redirection);
        Ok(())
    }
}

impl<'config> Parser<BashLanguage> for BashParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<BashLanguage>) -> oak_core::ParseOutput<'a, BashLanguage> {
        let lexer = BashLexer::new(self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() && !state.at(BashTokenType::Eof) {
                if state.at(BashTokenType::Newline) || state.at(BashTokenType::Delimiter) {
                    state.bump()
                }
                else {
                    self.parse_statement(state).ok();
                }
            }

            Ok(state.finish_at(checkpoint, BashElementType::Root))
        })
    }
}
