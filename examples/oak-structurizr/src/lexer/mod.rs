/// Token types for the Structurizr language.
pub mod token_type;

use crate::lexer::token_type::StructurizrTokenType;
use core::range::Range;
use oak_core::{Lexer, LexerCache, LexerState, TextEdit, errors::OakError, lexer::LexOutput, source::Source};

/// A token in the Structurizr language.
pub type StructurizrToken = oak_core::Token<StructurizrTokenType>;

pub(crate) type State<'a, S> = LexerState<'a, S, crate::language::StructurizrLanguage>;

/// A lexer for the Structurizr language.
pub struct StructurizrLexer<'config> {
    config: &'config crate::language::StructurizrLanguage,
}

impl<'config> StructurizrLexer<'config> {
    /// Creates a new Structurizr lexer with the given configuration.
    pub fn new(config: &'config crate::language::StructurizrLanguage) -> Self {
        Self { config }
    }

    fn run<S: Source + ?Sized>(&self, state: &mut State<S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if let Some(ch) = state.peek() {
                match ch {
                    ' ' | '\t' => {
                        self.skip_whitespace(state);
                    }
                    '\n' | '\r' => {
                        self.lex_newline(state);
                    }
                    '/' => {
                        if self.lex_comment(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    '{' => {
                        state.advance(1);
                        state.add_token(StructurizrTokenType::LeftBrace, safe_point, state.get_position());
                    }
                    '}' => {
                        state.advance(1);
                        state.add_token(StructurizrTokenType::RightBrace, safe_point, state.get_position());
                    }
                    '"' => {
                        self.lex_string(state);
                    }
                    'a'..='z' | 'A'..='Z' => {
                        self.lex_identifier(state);
                    }
                    _ => {
                        self.lex_text(state);
                    }
                }
            }

            state.advance_if_dead_lock(safe_point)
        }
        Ok(())
    }

    /// Skips whitespace
    fn skip_whitespace<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(StructurizrTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles newlines
    fn lex_newline<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(StructurizrTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(StructurizrTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles comments
    fn lex_comment<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if state.peek() == Some('/') {
            // Check if it's a comment (starts with //)
            if state.source().get_char_at(start_pos + 1) == Some('/') {
                state.advance(2);
                while state.not_at_end() {
                    if let Some(ch) = state.peek() {
                        if ch == '\n' || ch == '\r' {
                            break;
                        }
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }
                state.add_token(StructurizrTokenType::Comment, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// Handles strings
    fn lex_string<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if state.peek() == Some('"') {
            state.advance(1);
            let mut found_end = false;

            while state.not_at_end() {
                if let Some(ch) = state.peek() {
                    if ch == '"' {
                        state.advance(1);
                        found_end = true;
                        break;
                    }
                    else if ch == '\\' {
                        // Handle escape sequences
                        state.advance(1);
                        if state.not_at_end() {
                            state.advance(1);
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }
                else {
                    break;
                }
            }

            if found_end {
                state.add_token(StructurizrTokenType::Label, start_pos, state.get_position());
                return true;
            }
            state.set_position(start_pos);
        }
        false
    }

    /// Handles identifiers and keywords
    fn lex_identifier<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        while state.not_at_end() {
            if let Some(ch) = state.peek() {
                if ch.is_alphanumeric() || ch == '_' {
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }
            else {
                break;
            }
        }

        let range = oak_core::Range { start: start_pos, end: state.get_position() };
        let identifier = state.source().get_text_in(range).to_string();
        match identifier.as_str() {
            "workspace" => state.add_token(StructurizrTokenType::Workspace, start_pos, state.get_position()),
            "model" => state.add_token(StructurizrTokenType::Model, start_pos, state.get_position()),
            "person" => state.add_token(StructurizrTokenType::Person, start_pos, state.get_position()),
            "softwareSystem" => state.add_token(StructurizrTokenType::SoftwareSystem, start_pos, state.get_position()),
            "container" => state.add_token(StructurizrTokenType::Container, start_pos, state.get_position()),
            "component" => state.add_token(StructurizrTokenType::Component, start_pos, state.get_position()),
            _ => state.add_token(StructurizrTokenType::Id, start_pos, state.get_position()),
        }
        true
    }

    /// Lexes plain text
    fn lex_text<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        while state.not_at_end() {
            if let Some(ch) = state.peek() {
                // Stop when encountering special characters
                match ch {
                    ' ' | '\t' | '\n' | '\r' | '/' | '{' | '}' | '"' => break,
                    _ => {
                        state.advance(ch.len_utf8());
                    }
                }
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(StructurizrTokenType::Error, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<crate::language::StructurizrLanguage> for StructurizrLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, text: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<crate::language::StructurizrLanguage>) -> LexOutput<crate::language::StructurizrLanguage> {
        let mut state = State::new(text);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> StructurizrLexer<'config> {
    /// Runs the lexer on the given source and returns the output.
    pub fn lex_internal<'a, S: Source + ?Sized>(&self, source: &'a S) -> LexOutput<crate::language::StructurizrLanguage> {
        let mut state = State::new(source);
        let result = self.run(&mut state);
        state.finish(result)
    }
}
