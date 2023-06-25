#![doc = include_str!("readme.md")]
/// Token types for the Nginx lexer.
pub mod token_type;

use crate::{language::NginxLanguage, lexer::token_type::NginxTokenType};
use oak_core::{Lexer, LexerCache, LexerState, lexer::LexOutput, source::Source};

pub(crate) type State<'a, S> = LexerState<'a, S, NginxLanguage>;

/// Lexer for Nginx configuration files.
#[derive(Clone, Debug)]
pub struct NginxLexer<'config> {
    config: &'config NginxLanguage,
}

impl<'config> NginxLexer<'config> {
    /// Creates a new Nginx lexer with the given configuration.
    pub fn new(config: &'config NginxLanguage) -> Self {
        Self { config }
    }

    /// Skips whitespace characters.
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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
            state.add_token(NginxTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles newline characters.
    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(NginxTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(NginxTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles comments.
    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('#') = state.peek() {
            state.advance(1);

            // Read until the end of the line
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(NginxTokenType::CommentToken, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles strings.
    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote != '"' && quote != '\'' {
                return false;
            }

            state.advance(1); // Skip start quote
            while let Some(ch) = state.peek() {
                if ch == quote {
                    state.advance(1); // Skip end quote
                    break;
                }
                else if ch == '\\' {
                    state.advance(1); // Skip escape character
                    if let Some(c) = state.peek() {
                        state.advance(c.len_utf8());
                    }
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            state.add_token(NginxTokenType::String, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles numbers.
    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if !ch.is_ascii_digit() {
                return false;
            }

            // Handle integer part
            while let Some(ch) = state.peek() {
                if ch.is_ascii_digit() {
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }

            // Handle decimal part
            if let Some('.') = state.peek() {
                if let Some(next_ch) = state.peek_next_n(1) {
                    if next_ch.is_ascii_digit() {
                        state.advance(1); // Skip decimal point
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() {
                                state.advance(ch.len_utf8());
                            }
                            else {
                                break;
                            }
                        }
                    }
                }
            }

            // Handle unit suffixes (k, m, g, s, ms, etc.)
            if let Some(ch) = state.peek() {
                if ch.is_ascii_alphabetic() {
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_alphabetic() {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }
                }
            }

            state.add_token(NginxTokenType::Number, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles paths.
    fn lex_path<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch.is_ascii_alphanumeric() || ch == '/' || ch == '.' || ch == '-' || ch == '_' || ch == '*' {
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }

            state.add_token(NginxTokenType::Path, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles URLs.
    fn lex_url<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        // Check if starts with http:// or https://
        if state.starts_with("http://") || state.starts_with("https://") {
            let scheme_len = if state.starts_with("https://") { 8 } else { 7 };
            state.advance(scheme_len);

            while let Some(ch) = state.peek() {
                if ch.is_ascii_alphanumeric() || ch == '.' || ch == '/' || ch == ':' || ch == '-' || ch == '_' || ch == '?' || ch == '&' || ch == '=' {
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }

            state.add_token(NginxTokenType::Url, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles identifiers and keywords.
    fn lex_identifier<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if !ch.is_ascii_alphanumeric() && ch != '_' && ch != '$' {
                return false;
            }

            // Collect identifier characters
            while let Some(ch) = state.peek() {
                if ch.is_ascii_alphanumeric() || ch == '_' || ch == '$' {
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }

            // Check if it's a keyword
            let end_pos = state.get_position();
            let text = state.source().get_text_in(oak_core::Range { start: start_pos, end: end_pos });
            let token_kind = match text.as_ref() {
                "server" => NginxTokenType::ServerKeyword,
                "location" => NginxTokenType::LocationKeyword,
                "upstream" => NginxTokenType::UpstreamKeyword,
                "http" => NginxTokenType::HttpKeyword,
                "events" => NginxTokenType::EventsKeyword,
                "listen" => NginxTokenType::ListenKeyword,
                "server_name" => NginxTokenType::ServerNameKeyword,
                "root" => NginxTokenType::RootKeyword,
                "index" => NginxTokenType::IndexKeyword,
                "proxy_pass" => NginxTokenType::ProxyPassKeyword,
                _ => NginxTokenType::Identifier,
            };

            state.add_token(token_kind, start_pos, end_pos);
            true
        }
        else {
            false
        }
    }

    /// Handles delimiters.
    fn lex_delimiter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '{' => NginxTokenType::LeftBrace,
                '}' => NginxTokenType::RightBrace,
                ';' => NginxTokenType::Semicolon,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        while state.not_at_end() {
            let start_pos = state.get_position();

            // Try various lexical rules
            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_newline(state) {
                continue;
            }

            if self.lex_comment(state) {
                continue;
            }

            if self.lex_string(state) {
                continue;
            }

            if self.lex_url(state) {
                continue;
            }

            if self.lex_path(state) {
                continue;
            }

            if self.lex_number(state) {
                continue;
            }

            if self.lex_identifier(state) {
                continue;
            }

            if self.lex_delimiter(state) {
                continue;
            }

            // If no rules match, skip current character and mark as error
            state.advance_if_dead_lock(start_pos);
            if state.get_position() > start_pos {
                state.add_token(NginxTokenType::Error, start_pos, state.get_position())
            }
        }
        Ok(())
    }
}

impl<'config> Lexer<NginxLanguage> for NginxLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<NginxLanguage>) -> LexOutput<NginxLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}
