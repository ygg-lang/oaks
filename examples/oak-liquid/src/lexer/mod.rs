/// Liquid Lexer module
///
/// This module defines the lexer for Liquid templates, responsible for tokenizing the input.
use oak_core::{
    lexer::{LexOutput, Lexer, LexerCache, LexerState},
    source::{Source, TextEdit},
};

pub mod token_type;
use crate::language::LiquidLanguage;
use token_type::LiquidTokenType;

/// Lexer for Liquid templates
#[derive(Debug, Clone)]
pub struct LiquidLexer<'config> {
    /// Language configuration
    config: &'config LiquidLanguage,
}

pub(crate) type State<'a, S> = LexerState<'a, S, LiquidLanguage>;

impl<'config> LiquidLexer<'config> {
    /// Create a new Liquid lexer
    pub fn new(config: &'config LiquidLanguage) -> Self {
        Self { config }
    }

    fn run<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> Result<(), oak_core::OakError> {
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

            if self.lex_html_text(state) {
                continue;
            }

            state.advance_if_dead_lock(safe_point)
        }

        Ok(())
    }

    fn lex_html_text<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();
        while let Some(ch) = state.peek() {
            let rest = state.rest();
            if rest.starts_with(&self.config.variable_start) || rest.starts_with(&self.config.tag_start) || rest.starts_with(&self.config.comment_start) {
                break;
            }
            state.advance(ch.len_utf8());
        }
        if state.get_position() > start {
            state.add_token(LiquidTokenType::Identifier, start, state.get_position());
            return true;
        }
        false
    }

    fn skip_whitespace<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();
        let mut found = false;

        while let Some(ch) = state.peek() {
            if ch.is_whitespace() {
                state.advance(ch.len_utf8());
                found = true;
            }
            else {
                break;
            }
        }

        if found {
            state.add_token(LiquidTokenType::Whitespace, start, state.get_position());
        }

        found
    }

    fn skip_comment<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();
        if state.consume_if_starts_with(&self.config.comment_start) {
            while state.not_at_end() {
                if state.consume_if_starts_with(&self.config.comment_end) {
                    break;
                }
                if let Some(ch) = state.peek() {
                    state.advance(ch.len_utf8());
                }
            }
            state.add_token(LiquidTokenType::Comment, start, state.get_position());
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
                            state.advance(1);
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(LiquidTokenType::String, start, state.get_position());
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
                    if ch.is_ascii_digit() || ch == '.' {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                state.add_token(LiquidTokenType::Number, start, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_punctuation<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // Double-character operators
        if rest.starts_with(&self.config.variable_start) {
            state.advance(self.config.variable_start.len());
            state.add_token(LiquidTokenType::DoubleLeftBrace, start, state.get_position());
            return true;
        }
        if rest.starts_with(&self.config.variable_end) {
            state.advance(self.config.variable_end.len());
            state.add_token(LiquidTokenType::DoubleRightBrace, start, state.get_position());
            return true;
        }
        if rest.starts_with(&self.config.tag_start) {
            state.advance(self.config.tag_start.len());
            state.add_token(LiquidTokenType::LeftBracePercent, start, state.get_position());
            return true;
        }
        if rest.starts_with(&self.config.tag_end) {
            state.advance(self.config.tag_end.len());
            state.add_token(LiquidTokenType::PercentRightBrace, start, state.get_position());
            return true;
        }

        // Single-character operators
        if let Some(ch) = state.peek() {
            let kind = match ch {
                '{' => LiquidTokenType::LeftBrace,
                '}' => LiquidTokenType::RightBrace,
                '(' => LiquidTokenType::LeftParen,
                ')' => LiquidTokenType::RightParen,
                '[' => LiquidTokenType::LeftBracket,
                ']' => LiquidTokenType::RightBracket,
                ',' => LiquidTokenType::Comma,
                '.' => LiquidTokenType::Dot,
                ':' => LiquidTokenType::Colon,
                ';' => LiquidTokenType::Semicolon,
                '|' => LiquidTokenType::Pipe,
                '=' => LiquidTokenType::Eq,
                '+' => LiquidTokenType::Plus,
                '-' => LiquidTokenType::Minus,
                '*' => LiquidTokenType::Star,
                '/' => LiquidTokenType::Slash,
                '%' => LiquidTokenType::Percent,
                '!' => LiquidTokenType::Bang,
                '?' => LiquidTokenType::Question,
                '<' => LiquidTokenType::Lt,
                '>' => LiquidTokenType::Gt,
                '&' => LiquidTokenType::Amp,
                '^' => LiquidTokenType::Caret,
                '~' => LiquidTokenType::Tilde,
                _ => return false,
            };

            state.advance(ch.len_utf8());
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

                // Check if it is a boolean keyword
                let kind = match text.as_ref() {
                    "true" | "false" => LiquidTokenType::Boolean,
                    _ => LiquidTokenType::Identifier,
                };
                state.add_token(kind, start, end);
                return true;
            }
        }
        false
    }
}

impl<'config> Lexer<LiquidLanguage> for LiquidLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<LiquidLanguage>) -> LexOutput<LiquidLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}
