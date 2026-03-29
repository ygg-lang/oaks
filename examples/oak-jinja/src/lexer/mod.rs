/// Jinja Lexer module
///
/// This module defines the lexer for Jinja templates, responsible for tokenizing the input.
use oak_core::{
    lexer::{LexOutput, Lexer, LexerCache, LexerState},
    source::{Source, TextEdit},
};

pub mod token_type;
use crate::language::JinjaLanguage;
use token_type::JinjaTokenType;

/// Lexer for Jinja templates
#[derive(Debug, Clone)]
pub struct JinjaLexer<'config> {
    /// Language configuration
    config: &'config JinjaLanguage,
}

pub(crate) type State<'a, S> = LexerState<'a, S, JinjaLanguage>;

impl<'config> JinjaLexer<'config> {
    /// Create a new Jinja lexer
    pub fn new(config: &'config JinjaLanguage) -> Self {
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
            state.add_token(JinjaTokenType::Text, start, state.get_position());
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
            state.add_token(JinjaTokenType::Whitespace, start, state.get_position());
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
            state.add_token(JinjaTokenType::Comment, start, state.get_position());
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

                state.add_token(JinjaTokenType::String, start, state.get_position());
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

                state.add_token(JinjaTokenType::Number, start, state.get_position());
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
            state.add_token(JinjaTokenType::DoubleLeftBrace, start, state.get_position());
            return true;
        }
        if rest.starts_with(&self.config.variable_end) {
            state.advance(self.config.variable_end.len());
            state.add_token(JinjaTokenType::DoubleRightBrace, start, state.get_position());
            return true;
        }
        if rest.starts_with(&self.config.tag_start) {
            state.advance(self.config.tag_start.len());
            state.add_token(JinjaTokenType::LeftBracePercent, start, state.get_position());
            if state.peek() == Some('-') {
                let trim_start = state.get_position();
                state.advance(1);
                state.add_token(JinjaTokenType::TrimMark, trim_start, state.get_position());
            }
            return true;
        }
        let trim_tag_end = format!("-{}", &self.config.tag_end);
        if rest.starts_with(&trim_tag_end) {
            let trim_start = state.get_position();
            state.advance(1);
            state.add_token(JinjaTokenType::TrimMark, trim_start, state.get_position());
            state.advance(self.config.tag_end.len());
            state.add_token(JinjaTokenType::PercentRightBrace, start, state.get_position());
            return true;
        }
        if rest.starts_with(&self.config.tag_end) {
            state.advance(self.config.tag_end.len());
            state.add_token(JinjaTokenType::PercentRightBrace, start, state.get_position());
            return true;
        }

        if rest.starts_with("==") {
            state.advance(2);
            state.add_token(JinjaTokenType::EqEq, start, state.get_position());
            return true;
        }
        if rest.starts_with("!=") {
            state.advance(2);
            state.add_token(JinjaTokenType::Neq, start, state.get_position());
            return true;
        }
        if rest.starts_with("<=") {
            state.advance(2);
            state.add_token(JinjaTokenType::LtEq, start, state.get_position());
            return true;
        }
        if rest.starts_with(">=") {
            state.advance(2);
            state.add_token(JinjaTokenType::GtEq, start, state.get_position());
            return true;
        }

        // Single-character operators
        if let Some(ch) = state.peek() {
            let kind = match ch {
                '{' => JinjaTokenType::LeftBrace,
                '}' => JinjaTokenType::RightBrace,
                '(' => JinjaTokenType::LeftParen,
                ')' => JinjaTokenType::RightParen,
                '[' => JinjaTokenType::LeftBracket,
                ']' => JinjaTokenType::RightBracket,
                ',' => JinjaTokenType::Comma,
                '.' => JinjaTokenType::Dot,
                ':' => JinjaTokenType::Colon,
                ';' => JinjaTokenType::Semicolon,
                '|' => JinjaTokenType::Pipe,
                '=' => JinjaTokenType::Eq,
                '+' => JinjaTokenType::Plus,
                '-' => JinjaTokenType::Minus,
                '*' => JinjaTokenType::Star,
                '/' => JinjaTokenType::Slash,
                '%' => JinjaTokenType::Percent,
                '!' => JinjaTokenType::Bang,
                '?' => JinjaTokenType::Question,
                '<' => JinjaTokenType::Lt,
                '>' => JinjaTokenType::Gt,
                '&' => JinjaTokenType::Amp,
                '^' => JinjaTokenType::Caret,
                '~' => JinjaTokenType::Tilde,
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
                    "true" | "false" => JinjaTokenType::Boolean,
                    "and" => JinjaTokenType::And,
                    "or" => JinjaTokenType::Or,
                    "not" => JinjaTokenType::Not,
                    _ => JinjaTokenType::Identifier,
                };
                state.add_token(kind, start, end);
                return true;
            }
        }
        false
    }
}

impl<'config> Lexer<JinjaLanguage> for JinjaLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<JinjaLanguage>) -> LexOutput<JinjaLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}
