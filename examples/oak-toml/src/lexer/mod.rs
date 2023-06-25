#![doc = include_str!("readme.md")]
/// Token types for the TOML language.
pub mod token_type;
pub use crate::lexer::token_type::TomlTokenType;

use crate::{language::TomlLanguage, lexer::token_type::TomlTokenKind as TomlSyntaxKind};
use oak_core::{
    Lexer, LexerState, OakError, TextEdit,
    lexer::{LexOutput, LexerCache},
    source::Source,
};

pub(crate) type State<'a, S> = LexerState<'a, S, TomlLanguage>;

/// TOML lexer implementation.
///
/// This struct implements the `Lexer` trait for the TOML language,
/// converting source text into a stream of tokens.
#[derive(Clone, Debug)]
pub struct TomlLexer<'config> {
    config: &'config TomlLanguage,
}

impl<'config> Lexer<TomlLanguage> for TomlLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<TomlLanguage>) -> LexOutput<TomlLanguage> {
        let mut state = State::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> TomlLexer<'config> {
    /// Creates a new `TomlLexer` with the given language configuration.
    pub fn new(config: &'config TomlLanguage) -> Self {
        Self { config }
    }

    /// Main lexing loop that iterates through the source text.
    fn run<S: Source + ?Sized>(&self, state: &mut State<S>) -> Result<(), OakError> {
        while state.not_at_end() {
            if let Some(ch) = state.peek() {
                match ch {
                    ' ' | '\t' | '\n' | '\r' => {
                        self.skip_whitespace(state);
                    }
                    '#' => {
                        self.skip_comment(state);
                    }
                    '"' | '\'' => {
                        self.lex_string(state);
                    }
                    '0'..='9' | '+' | '-' => {
                        self.lex_number(state);
                    }
                    '[' | ']' | '{' | '}' | ',' | '.' | '=' => {
                        self.lex_punctuation(state);
                    }
                    'a'..='z' | 'A'..='Z' | '_' => {
                        self.lex_identifier(state);
                    }
                    _ => {
                        // Fallback for any other punctuation or unknown characters
                        if self.lex_punctuation(state) {
                            continue;
                        }
                        // Skip character if no pattern matches
                        state.advance(1);
                    }
                }
            }
            else {
                break;
            }
        }
        Ok(())
    }

    /// Lexes and skips whitespace characters.
    fn skip_whitespace<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.current() {
            if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(TomlSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Lexes and skips TOML comments.
    fn skip_comment<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        if state.current() == Some('#') {
            let start_pos = state.get_position();
            state.advance(1);

            // Read until end of line
            while let Some(ch) = state.current() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(TomlSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Lexes strings.
    fn lex_string<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        match state.current() {
            Some('"') => {
                let start = state.get_position();
                state.advance(1);

                // Simple string parsing
                while let Some(ch) = state.current() {
                    if ch == '"' {
                        state.advance(1);
                        break;
                    }
                    if ch == '\\' {
                        state.advance(1); // Skip escape character
                        if state.current().is_some() {
                            state.advance(1);
                        }
                    }
                    else {
                        state.advance(1);
                    }
                }

                let end = state.get_position();
                state.add_token(TomlSyntaxKind::BasicString, start, end);
                true
            }
            Some('\'') => {
                let start = state.get_position();
                state.advance(1);

                // Literal string parsing
                while let Some(ch) = state.current() {
                    if ch == '\'' {
                        state.advance(1);
                        break;
                    }
                    state.advance(1);
                }

                let end = state.get_position();
                state.add_token(TomlSyntaxKind::LiteralString, start, end);
                true
            }
            _ => false,
        }
    }

    /// Lexes numbers.
    fn lex_number<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        if !state.current().map_or(false, |c| c.is_ascii_digit() || c == '-' || c == '+') {
            return false;
        }

        let start = state.get_position();

        // Skip sign
        if matches!(state.current(), Some('-') | Some('+')) {
            state.advance(1);
        }

        // Lex digits
        while state.current().map_or(false, |c| c.is_ascii_digit()) {
            state.advance(1);
        }

        // Check if it's a float
        let mut is_float = false;
        if state.current() == Some('.') {
            is_float = true;
            state.advance(1);
            while state.current().map_or(false, |c| c.is_ascii_digit()) {
                state.advance(1);
            }
        }

        let end = state.get_position();
        let kind = if is_float { TomlSyntaxKind::Float } else { TomlSyntaxKind::Integer };
        state.add_token(kind, start, end);
        true
    }

    /// Lexes punctuation.
    fn lex_punctuation<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        match state.current() {
            Some('[') => {
                state.advance(1);
                if state.current() == Some('[') {
                    state.advance(1);
                    let end = state.get_position();
                    state.add_token(TomlSyntaxKind::DoubleLeftBracket, start, end);
                }
                else {
                    let end = state.get_position();
                    state.add_token(TomlSyntaxKind::LeftBracket, start, end);
                }
                true
            }
            Some(']') => {
                state.advance(1);
                if state.current() == Some(']') {
                    state.advance(1);
                    let end = state.get_position();
                    state.add_token(TomlSyntaxKind::DoubleRightBracket, start, end);
                }
                else {
                    let end = state.get_position();
                    state.add_token(TomlSyntaxKind::RightBracket, start, end);
                }
                true
            }
            Some('{') => {
                state.advance(1);
                let end = state.get_position();
                state.add_token(TomlSyntaxKind::LeftBrace, start, end);
                true
            }
            Some('}') => {
                state.advance(1);
                let end = state.get_position();
                state.add_token(TomlSyntaxKind::RightBrace, start, end);
                true
            }
            Some(',') => {
                state.advance(1);
                let end = state.get_position();
                state.add_token(TomlSyntaxKind::Comma, start, end);
                true
            }
            Some('.') => {
                state.advance(1);
                let end = state.get_position();
                state.add_token(TomlSyntaxKind::Dot, start, end);
                true
            }
            Some('=') => {
                state.advance(1);
                let end = state.get_position();
                state.add_token(TomlSyntaxKind::Equal, start, end);
                true
            }
            _ => false,
        }
    }

    /// Lexes identifiers and keys.
    fn lex_identifier<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        if !state.current().map_or(false, |c| c.is_ascii_alphabetic() || c == '_') {
            return false;
        }

        let start = state.get_position();

        while state.current().map_or(false, |c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
            state.advance(1);
        }

        let end = state.get_position();

        // Check for keywords
        let text = state.get_text_in((start..end).into());
        let kind = match text.as_ref() {
            "true" | "false" => TomlSyntaxKind::Boolean,
            _ => TomlSyntaxKind::BareKey,
        };

        state.add_token(kind, start, end);
        true
    }
}
