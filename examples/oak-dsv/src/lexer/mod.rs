#![doc = include_str!("readme.md")]
/// Token types for the DSV language.
pub mod token_type;
use crate::language::{Dsv, DsvLanguage};
use oak_core::{Lexer, LexerState, OakError, lexer::LexOutput, source::Source};
pub use token_type::DsvTokenType;

/// DSV lexer state.
pub(crate) type State<'a, const LANG: DsvLanguage, S> = LexerState<'a, S, Dsv<LANG>>;

/// Lexer for the DSV language.
///
/// This lexer handles basic DSV tokenization, including quoted fields,
/// field separators, and newlines.
#[derive(Clone)]
pub struct DsvLexer<const LANG: DsvLanguage>;

impl<const LANG: DsvLanguage> Lexer<Dsv<LANG>> for DsvLexer<LANG> {
    fn lex<'a, S: Source + ?Sized>(&self, text: &'a S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl oak_core::LexerCache<Dsv<LANG>>) -> LexOutput<Dsv<LANG>> {
        let mut state = State::<LANG, S>::new(text);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}

impl<const LANG: DsvLanguage> DsvLexer<LANG> {
    /// Creates a new `DsvLexer`.
    pub fn new() -> Self {
        Self
    }

    /// Skips whitespace characters.
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, LANG, S>) -> bool {
        let start_pos = state.get_position();
        let mut found_whitespace = false;

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' {
                state.advance(ch.len_utf8());
                found_whitespace = true
            }
            else {
                break;
            }
        }

        if found_whitespace {
            state.add_token(DsvTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Lexes a newline.
    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, LANG, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch == '\r' {
                state.advance(1);
                // Check if it's CRLF
                if state.peek() == Some('\n') {
                    state.advance(1)
                }
                state.add_token(DsvTokenType::Newline, start_pos, state.get_position());
                true
            }
            else if ch == '\n' {
                state.advance(1);
                state.add_token(DsvTokenType::Newline, start_pos, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    /// Handles quoted fields.
    fn lex_quoted_field<'a, S: Source + ?Sized>(&self, state: &mut State<'a, LANG, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch == LANG.quote_char {
                state.advance(ch.len_utf8()); // Skip start quote
                while let Some(ch) = state.peek() {
                    if ch == LANG.quote_char {
                        state.advance(ch.len_utf8());
                        // Check if it's an escaped quote (double quote)
                        if state.peek() == Some(LANG.quote_char) {
                            state.advance(LANG.quote_char.len_utf8()); // Skip escaped quote
                        }
                        else {
                            // End quote
                            break;
                        }
                    }
                    else {
                        state.advance(ch.len_utf8())
                    }
                }
                state.add_token(DsvTokenType::Field, start_pos, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    /// Handles unquoted fields.
    fn lex_unquoted_field<'a, S: Source + ?Sized>(&self, state: &mut State<'a, LANG, S>) -> bool {
        let start_pos = state.get_position();
        let mut found_char = false;

        while let Some(ch) = state.peek() {
            if ch == LANG.field_separator || ch == '\n' || ch == '\r' {
                break;
            }
            else {
                state.advance(ch.len_utf8());
                found_char = true
            }
        }

        if found_char {
            state.add_token(DsvTokenType::Field, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles separators.
    fn lex_separator<'a, S: Source + ?Sized>(&self, state: &mut State<'a, LANG, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch == LANG.field_separator {
                state.advance(ch.len_utf8());
                state.add_token(DsvTokenType::Separator, start_pos, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, LANG, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            // Try various lexical rules
            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_newline(state) {
                continue;
            }

            if self.lex_separator(state) {
                continue;
            }

            if self.lex_quoted_field(state) {
                continue;
            }

            if self.lex_unquoted_field(state) {
                continue;
            }

            // If no rules match, report error and skip one character
            let start = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(DsvTokenType::Error, start, state.get_position());
            }
        }
        Ok(())
    }
}
