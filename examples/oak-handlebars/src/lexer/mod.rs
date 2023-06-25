#![doc = include_str!("readme.md")]
use crate::language::HandlebarsLanguage;
pub mod token_type;
pub use token_type::HandlebarsTokenType;

use oak_core::{
    Lexer, LexerCache, LexerState, OakError, Range,
    lexer::{LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

pub(crate) type State<'a, S> = LexerState<'a, S, HandlebarsLanguage>;

// Scanner configurations
static HB_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static HB_STRING_DOUBLE: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });
static HB_STRING_SINGLE: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['\''], escape: Some('\\') });

/// Handlebars lexer.
#[derive(Clone)]
pub struct HandlebarsLexer<'config> {
    config: &'config HandlebarsLanguage,
}

impl<'config> Lexer<HandlebarsLanguage> for HandlebarsLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[oak_core::TextEdit], cache: &'a mut impl LexerCache<HandlebarsLanguage>) -> LexOutput<HandlebarsLanguage> {
        let mut state: State<'_, S> = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> HandlebarsLexer<'config> {
    /// Creates a new `HandlebarsLexer`.
    pub fn new(config: &'config HandlebarsLanguage) -> Self {
        Self { config }
    }

    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            if self.skip_newline(state) {
                continue;
            }

            if self.lex_comment(state) {
                continue;
            }

            if self.lex_handlebars_expression(state) {
                continue;
            }

            if self.lex_string_literal(state) {
                continue;
            }

            if self.lex_number_literal(state) {
                continue;
            }

            if self.lex_identifier(state) {
                continue;
            }

            if self.lex_single_char_tokens(state) {
                continue;
            }

            if self.lex_content(state) {
                continue;
            }

            state.advance_if_dead_lock(safe_point)
        }

        Ok(())
    }

    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        HB_WHITESPACE.scan(state, HandlebarsTokenType::Whitespace)
    }

    fn skip_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if state.current() == Some('\n') || state.current() == Some('\r') {
            let start = state.get_position();
            state.advance(1);
            if state.current() == Some('\n') && state.peek() == Some('\r') {
                state.advance(1)
            }
            let end = state.get_position();
            state.add_token(HandlebarsTokenType::Newline, start, end);
            true
        }
        else {
            false
        }
    }

    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let rest = state.rest();
        if rest.starts_with(&self.config.variable_start) {
            let comment_rest = &rest[self.config.variable_start.len()..];
            if comment_rest.starts_with("!--") {
                let start = state.get_position();
                state.advance(self.config.variable_start.len() + 3);
                while state.not_at_end() {
                    let current_rest = state.rest();
                    if current_rest.starts_with("--") && current_rest[2..].starts_with(&self.config.variable_end) {
                        state.advance(2 + self.config.variable_end.len());
                        let end = state.get_position();
                        state.add_token(HandlebarsTokenType::Comment, start, end);
                        return true;
                    }
                    state.advance(1);
                }
                return true;
            }
            else if comment_rest.starts_with('!') {
                let start = state.get_position();
                state.advance(self.config.variable_start.len() + 1);
                while state.not_at_end() {
                    if state.rest().starts_with(&self.config.variable_end) {
                        state.advance(self.config.variable_end.len());
                        let end = state.get_position();
                        state.add_token(HandlebarsTokenType::Comment, start, end);
                        return true;
                    }
                    state.advance(1);
                }
                return true;
            }
        }
        false
    }

    fn lex_handlebars_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        if rest.starts_with(&self.config.unescaped_start) {
            state.advance(self.config.unescaped_start.len());
            state.add_token(HandlebarsTokenType::OpenUnescaped, start, state.get_position());
            true
        }
        else if rest.starts_with(&self.config.variable_start) {
            state.advance(self.config.variable_start.len());
            state.add_token(HandlebarsTokenType::Open, start, state.get_position());
            true
        }
        else if rest.starts_with(&self.config.unescaped_end) {
            state.advance(self.config.unescaped_end.len());
            state.add_token(HandlebarsTokenType::CloseUnescaped, start, state.get_position());
            true
        }
        else if rest.starts_with(&self.config.variable_end) {
            state.advance(self.config.variable_end.len());
            state.add_token(HandlebarsTokenType::Close, start, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let config = if state.current() == Some('"') {
            &*HB_STRING_DOUBLE
        }
        else if state.current() == Some('\'') {
            &*HB_STRING_SINGLE
        }
        else {
            return false;
        };

        config.scan(state, HandlebarsTokenType::StringLiteral)
    }

    fn lex_number_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some(c) = state.current() {
            if c.is_ascii_digit() {
                let start = state.get_position();
                while let Some(c) = state.current() {
                    if c.is_ascii_digit() || c == '.' { state.advance(1) } else { break }
                }
                let end = state.get_position();
                state.add_token(HandlebarsTokenType::NumberLiteral, start, end);
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

    fn lex_identifier<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some(c) = state.current() {
            if c.is_alphabetic() || c == '_' || c == '@' {
                let start = state.get_position();
                while let Some(c) = state.current() {
                    if c.is_alphanumeric() || c == '_' || c == '-' || c == '.' { state.advance(1) } else { break }
                }
                let end = state.get_position();
                let text = state.get_text_in(Range { start, end });
                let kind = match text.as_ref() {
                    "else" => HandlebarsTokenType::Else,
                    "true" | "false" => HandlebarsTokenType::BooleanLiteral,
                    _ => HandlebarsTokenType::Identifier,
                };
                state.add_token(kind, start, end);
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

    fn lex_single_char_tokens<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some(c) = state.current() {
            let start = state.get_position();
            let kind = match c {
                '(' => HandlebarsTokenType::LeftParen,
                ')' => HandlebarsTokenType::RightParen,
                '[' => HandlebarsTokenType::LeftBracket,
                ']' => HandlebarsTokenType::RightBracket,
                '=' => HandlebarsTokenType::Equal,
                '|' => HandlebarsTokenType::Pipe,
                '#' => HandlebarsTokenType::Hash,
                '.' => HandlebarsTokenType::Dot,
                '/' => HandlebarsTokenType::Slash,
                '@' => HandlebarsTokenType::At,
                '^' => HandlebarsTokenType::Caret,
                _ => return false,
            };
            state.advance(1);
            let end = state.get_position();
            state.add_token(kind, start, end);
            true
        }
        else {
            false
        }
    }

    fn lex_content<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let mut count = 0;

        while let Some(c) = state.current() {
            let rest = state.rest();
            if rest.starts_with(&self.config.variable_start) || rest.starts_with(&self.config.unescaped_start) {
                break;
            }
            state.advance(c.len_utf8());
            count += 1
        }

        if count > 0 {
            let end = state.get_position();
            state.add_token(HandlebarsTokenType::Content, start, end);
            true
        }
        else {
            false
        }
    }
}
