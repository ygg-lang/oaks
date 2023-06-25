#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::DockerfileLanguage, lexer::token_type::DockerfileTokenType};
use oak_core::{
    Lexer, LexerCache, LexerState, OakError, Source, TextEdit,
    lexer::{LexOutput, WhitespaceConfig},
};
use std::sync::LazyLock;

pub(crate) type State<'a, S> = LexerState<'a, S, DockerfileLanguage>;

static DOCKERFILE_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });

/// Lexer for Dockerfile files.
#[derive(Clone)]
pub struct DockerfileLexer<'config> {
    config: &'config DockerfileLanguage,
}

impl<'config> Lexer<DockerfileLanguage> for DockerfileLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, text: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<DockerfileLanguage>) -> LexOutput<DockerfileLanguage> {
        let mut state = State::new(text);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> DockerfileLexer<'config> {
    /// Creates a new `DockerfileLexer` with the given configuration.
    pub fn new(config: &'config DockerfileLanguage) -> Self {
        Self { config }
    }

    fn run<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_newline(state) {
                continue;
            }

            if self.lex_comment(state) {
                continue;
            }

            if self.lex_identifier_or_instruction(state) {
                continue;
            }

            if self.lex_number(state) {
                continue;
            }

            if self.lex_string(state) {
                continue;
            }

            if self.lex_path(state) {
                continue;
            }

            if self.lex_operators_and_delimiters(state) {
                continue;
            }

            if self.lex_other(state) {
                continue;
            }

            state.advance_if_dead_lock(safe_point)
        }

        Ok(())
    }

    /// Skips whitespace characters.
    fn skip_whitespace<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        DOCKERFILE_WHITESPACE.scan(state, DockerfileTokenType::Whitespace)
    }

    /// Handles newline characters.
    fn lex_newline<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.peek() {
            if ch == '\n' {
                state.advance(1);
                state.add_token(DockerfileTokenType::Newline, start, state.get_position());
                return true;
            }
            else if ch == '\r' {
                state.advance(1);
                if state.peek() == Some('\n') {
                    state.advance(1)
                }
                state.add_token(DockerfileTokenType::Newline, start, state.get_position());
                return true;
            }
        }
        false
    }

    /// Handles comments
    fn lex_comment<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        if state.peek() == Some('#') {
            state.advance(1);
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8())
            }
            state.add_token(DockerfileTokenType::Comment, start, state.get_position());
            return true;
        }
        false
    }

    /// Handles identifiers or instructions
    fn lex_identifier_or_instruction<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' { state.advance(ch.len_utf8()) } else { break }
                }

                let end_pos = state.get_position();
                let text = state.get_source().get_text_in((start..end_pos).into());

                // Check if it's a Dockerfile instruction
                let kind = match text.to_uppercase().as_str() {
                    "FROM" => DockerfileTokenType::From,
                    "RUN" => DockerfileTokenType::Run,
                    "CMD" => DockerfileTokenType::Cmd,
                    "LABEL" => DockerfileTokenType::Label,
                    "EXPOSE" => DockerfileTokenType::Expose,
                    "ENV" => DockerfileTokenType::Env,
                    "ADD" => DockerfileTokenType::Add,
                    "COPY" => DockerfileTokenType::Copy,
                    "ENTRYPOINT" => DockerfileTokenType::Entrypoint,
                    "VOLUME" => DockerfileTokenType::Volume,
                    "USER" => DockerfileTokenType::User,
                    "WORKDIR" => DockerfileTokenType::Workdir,
                    "ARG" => DockerfileTokenType::Arg,
                    "ONBUILD" => DockerfileTokenType::Onbuild,
                    "STOPSIGNAL" => DockerfileTokenType::Stopsignal,
                    "HEALTHCHECK" => DockerfileTokenType::Healthcheck,
                    "SHELL" => DockerfileTokenType::Shell,
                    "MAINTAINER" => DockerfileTokenType::Maintainer,
                    "AS" => DockerfileTokenType::As,
                    "NONE" => DockerfileTokenType::None,
                    "INTERVAL" => DockerfileTokenType::Interval,
                    "TIMEOUT" => DockerfileTokenType::Timeout,
                    "START_PERIOD" => DockerfileTokenType::StartPeriod,
                    "RETRIES" => DockerfileTokenType::Retries,
                    _ => DockerfileTokenType::Identifier,
                };

                state.add_token(kind, start, end_pos);
                return true;
            }
        }
        false
    }

    /// Handles numbers
    fn lex_number<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() || ch == '.' { state.advance(1) } else { break }
                }

                state.add_token(DockerfileTokenType::Number, start, state.get_position());
                return true;
            }
        }
        false
    }

    /// Handles strings
    fn lex_string<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
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
                        if state.peek().is_some() {
                            state.advance(1)
                        }
                    }
                    else {
                        state.advance(ch.len_utf8())
                    }
                }

                state.add_token(DockerfileTokenType::String, start, state.get_position());
                return true;
            }
        }
        false
    }

    /// Lexes paths.
    fn lex_path<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.peek() {
            if ch == '/' || ch == '.' {
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '/' || ch == '.' || ch == '-' || ch == '_' { state.advance(1) } else { break }
                }

                state.add_token(DockerfileTokenType::Path, start, state.get_position());
                return true;
            }
        }
        false
    }

    /// Lexes operators and delimiters.
    fn lex_operators_and_delimiters<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.peek() {
            let kind = match ch {
                '=' => DockerfileTokenType::Equal,
                ':' => DockerfileTokenType::Colon,
                '{' => DockerfileTokenType::LeftBrace,
                '}' => DockerfileTokenType::RightBrace,
                '[' => DockerfileTokenType::LeftBracket,
                ']' => DockerfileTokenType::RightBracket,
                '(' => DockerfileTokenType::LeftParen,
                ')' => DockerfileTokenType::RightParen,
                ',' => DockerfileTokenType::Comma,
                ';' => DockerfileTokenType::Semicolon,
                '$' => DockerfileTokenType::Dollar,
                _ => return false,
            };

            state.advance(1);
            state.add_token(kind, start, state.get_position());
            return true;
        }
        false
    }

    /// Lexes other characters.
    fn lex_other<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.peek() {
            state.advance(ch.len_utf8());
            state.add_token(DockerfileTokenType::Error, start, state.get_position());
            return true;
        }
        false
    }
}
