/// Token types and syntax kinds for PlantUML.
pub mod token_type;

use crate::{language::PlantUmlLanguage, lexer::token_type::PlantUmlTokenType};
use oak_core::{Lexer, LexerCache, LexerState, TextEdit, errors::OakError, lexer::LexOutput, source::Source};

pub(crate) type State<'a, S> = LexerState<'a, S, PlantUmlLanguage>;

/// Lexer for the PlantUML language.
pub struct PlantUmlLexer<'config> {
    config: &'config PlantUmlLanguage,
}

impl<'config> PlantUmlLexer<'config> {
    /// Creates a new lexer with the given configuration.
    pub fn new(config: &'config PlantUmlLanguage) -> Self {
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
                    '@' => {
                        if self.lex_directive(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    'c' => {
                        if state.starts_with("class") {
                            self.lex_keyword(state, "class", PlantUmlTokenType::Class);
                            continue;
                        }
                        self.lex_identifier(state);
                    }
                    'i' => {
                        if state.starts_with("interface") {
                            self.lex_keyword(state, "interface", PlantUmlTokenType::Interface);
                            continue;
                        }
                        self.lex_identifier(state);
                    }
                    'a'..='z' | 'A'..='Z' | '_' => {
                        self.lex_identifier(state);
                    }
                    '"' => {
                        self.lex_string(state);
                    }
                    _ => {
                        self.lex_text(state);
                    }
                }
            }

            if state.get_position() == safe_point {
                // Handle deadlock by advancing one character
                let start_pos = state.get_position();
                if let Some(ch) = state.peek() {
                    state.advance(ch.len_utf8());
                    state.add_token(PlantUmlTokenType::Error, start_pos, state.get_position());
                }
            }
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
            state.add_token(PlantUmlTokenType::Whitespace, start_pos, state.get_position());
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
            state.add_token(PlantUmlTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(PlantUmlTokenType::Newline, start_pos, state.get_position());
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
                state.add_token(PlantUmlTokenType::Comment, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// Handles directives like @startuml and @enduml
    fn lex_directive<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if state.peek() == Some('@') {
            state.advance(1);
            if state.starts_with("startuml") {
                state.advance(8);
                state.add_token(PlantUmlTokenType::StartUml, start_pos, state.get_position());
                return true;
            }
            else if state.starts_with("enduml") {
                state.advance(6);
                state.add_token(PlantUmlTokenType::EndUml, start_pos, state.get_position());
                return true;
            }
            state.set_position(start_pos);
        }
        false
    }

    /// Handles keywords
    fn lex_keyword<S: Source + ?Sized>(&self, state: &mut State<S>, keyword: &str, token_type: PlantUmlTokenType) {
        let start_pos = state.get_position();
        state.advance(keyword.len());
        state.add_token(token_type, start_pos, state.get_position());
    }

    /// Handles identifiers
    fn lex_identifier<S: Source + ?Sized>(&self, state: &mut State<S>) {
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

        state.add_token(PlantUmlTokenType::Id, start_pos, state.get_position());
    }

    /// Handles strings (labels)
    fn lex_string<S: Source + ?Sized>(&self, state: &mut State<S>) {
        let start_pos = state.get_position();

        if state.peek() == Some('"') {
            state.advance(1);
            while state.not_at_end() {
                if let Some(ch) = state.peek() {
                    if ch == '"' {
                        state.advance(1);
                        state.add_token(PlantUmlTokenType::Label, start_pos, state.get_position());
                        return;
                    }
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }
            // Unclosed string
            state.add_token(PlantUmlTokenType::Error, start_pos, state.get_position());
        }
    }

    /// Lexes plain text
    fn lex_text<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        while state.not_at_end() {
            if let Some(ch) = state.peek() {
                // Stop when encountering special characters
                match ch {
                    ' ' | '\t' | '\n' | '\r' | '/' | '@' | '"' => break,
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
            state.add_token(PlantUmlTokenType::Id, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<PlantUmlLanguage> for PlantUmlLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, text: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<PlantUmlLanguage>) -> LexOutput<PlantUmlLanguage> {
        let mut state = State::new(text);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> PlantUmlLexer<'config> {
    /// Runs the lexer on the given source and returns the output.
    pub fn lex_internal<'a, S: Source + ?Sized>(&self, source: &'a S) -> LexOutput<PlantUmlLanguage> {
        let mut state = State::new(source);
        let result = self.run(&mut state);
        state.finish(result)
    }
}
