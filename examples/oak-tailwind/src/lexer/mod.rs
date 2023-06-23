#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::TailwindLanguage, lexer::token_type::TailwindTokenType};
use oak_core::{Lexer, LexerCache, LexerState, OakError, lexer::LexOutput, source::Source};

/// Lexer for the Tailwind language.
#[derive(Clone, Debug)]
pub struct TailwindLexer<'config> {
    /// Language configuration
    _config: &'config TailwindLanguage,
}

type State<'a, S> = LexerState<'a, S, TailwindLanguage>;

impl<'config> TailwindLexer<'config> {
    /// Creates a new `TailwindLexer` with the given configuration.
    pub fn new(config: &'config TailwindLanguage) -> Self {
        Self { _config: config }
    }
}

impl<'config> Lexer<TailwindLanguage> for TailwindLexer<'config> {
    /// Tokenizes the source text into a sequence of Tailwind tokens.
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[oak_core::TextEdit], cache: &'a mut impl LexerCache<TailwindLanguage>) -> LexOutput<TailwindLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> TailwindLexer<'config> {
    fn run<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            if self.skip_comment(state) {
                continue;
            }

            if self.lex_string(state) {
                continue;
            }

            if self.lex_number(state) {
                continue;
            }

            if self.lex_punctuation(state) {
                continue;
            }

            if self.lex_identifier(state) {
                continue;
            }

            state.advance_if_dead_lock(safe_point)
        }

        Ok(())
    }

    fn skip_whitespace<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();
        let mut found = false;

        while let Some(ch) = state.peek() {
            if ch.is_whitespace() {
                state.advance(ch.len_utf8());
                found = true
            }
            else {
                break;
            }
        }

        if found {
            state.add_token(TailwindTokenType::Whitespace, start, state.get_position())
        }

        found
    }

    fn skip_comment<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();
        if state.consume_if_starts_with("{#") {
            while state.not_at_end() {
                if state.consume_if_starts_with("#}") {
                    break;
                }
                if let Some(ch) = state.peek() {
                    state.advance(ch.len_utf8())
                }
            }
            state.add_token(TailwindTokenType::Comment, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch == quote {
                        state.advance(1);
                        break;
                    }
                    else if ch == '\\' {
                        state.advance(1);
                        if let Some(_) = state.peek() {
                            state.advance(1)
                        }
                    }
                    else {
                        state.advance(ch.len_utf8())
                    }
                }

                state.add_token(TailwindTokenType::String, start, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() || ch == '.' { state.advance(1) } else { break }
                }

                state.add_token(TailwindTokenType::Number, start, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_punctuation<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // Two-character operators
        if rest.starts_with("{{") {
            state.advance(2);
            state.add_token(TailwindTokenType::DoubleLeftBrace, start, state.get_position());
            return true;
        }
        if rest.starts_with("}}") {
            state.advance(2);
            state.add_token(TailwindTokenType::DoubleRightBrace, start, state.get_position());
            return true;
        }
        if rest.starts_with("{%") {
            state.advance(2);
            state.add_token(TailwindTokenType::LeftBracePercent, start, state.get_position());
            return true;
        }
        if rest.starts_with("%}") {
            state.advance(2);
            state.add_token(TailwindTokenType::PercentRightBrace, start, state.get_position());
            return true;
        }

        // Single-character operators
        if let Some(ch) = state.peek() {
            let kind = match ch {
                '{' => TailwindTokenType::LeftBrace,
                '}' => TailwindTokenType::RightBrace,
                '(' => TailwindTokenType::LeftParen,
                ')' => TailwindTokenType::RightParen,
                '[' => TailwindTokenType::LeftBracket,
                ']' => TailwindTokenType::RightBracket,
                ',' => TailwindTokenType::Comma,
                '.' => TailwindTokenType::Dot,
                ':' => TailwindTokenType::Colon,
                ';' => TailwindTokenType::Semicolon,
                '|' => TailwindTokenType::Pipe,
                '=' => TailwindTokenType::Eq,
                '+' => TailwindTokenType::Plus,
                '-' => TailwindTokenType::Minus,
                '*' => TailwindTokenType::Star,
                '/' => TailwindTokenType::Slash,
                '%' => TailwindTokenType::Percent,
                '!' => TailwindTokenType::Bang,
                '?' => TailwindTokenType::Question,
                '<' => TailwindTokenType::Lt,
                '>' => TailwindTokenType::Gt,
                '&' => TailwindTokenType::Amp,
                '^' => TailwindTokenType::Caret,
                '~' => TailwindTokenType::Tilde,
                _ => return false,
            };

            state.advance(1);
            state.add_token(kind, start, state.get_position());
            return true;
        }

        false
    }

    fn lex_identifier<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let end = state.get_position();
                let text = state.get_text_in((start..end).into());

                // Check if it's a boolean keyword
                let kind = match text.as_ref() {
                    "true" | "false" => TailwindTokenType::Boolean,
                    _ => TailwindTokenType::Identifier,
                };
                state.add_token(kind, start, end);
                return true;
            }
        }
        false
    }
}
