//! Svelte lexer implementation.

pub mod token_type;
use crate::{language::SvelteLanguage, lexer::token_type::SvelteTokenType};
use oak_core::{LexOutput, Lexer, LexerCache, LexerState, OakError, Source};

pub(crate) type State<'a, S> = LexerState<'a, S, SvelteLanguage>;

/// Svelte lexer.
#[derive(Clone, Debug)]
pub struct SvelteLexer<'config> {
    config: &'config SvelteLanguage,
}

impl<'config> SvelteLexer<'config> {
    /// Creates a new `SvelteLexer`.
    pub fn new(config: &'config SvelteLanguage) -> Self {
        Self { config }
    }

    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let start_pos = state.get_position();

            // 1. Handle Whitespace
            if let Some(ch) = state.peek() {
                if ch.is_whitespace() {
                    while let Some(c) = state.peek() {
                        if c.is_whitespace() {
                            state.advance(c.len_utf8());
                        }
                        else {
                            break;
                        }
                    }
                    state.add_token(SvelteTokenType::Whitespace, start_pos, state.get_position());
                    continue;
                }
            }

            let rest = state.rest();

            // 2. Handle HTML-like tags
            if rest.starts_with("</") {
                state.advance(2);
                state.add_token(SvelteTokenType::LtSlash, start_pos, state.get_position());
                continue;
            }
            if rest.starts_with("/>") {
                state.advance(2);
                state.add_token(SvelteTokenType::SlashGt, start_pos, state.get_position());
                continue;
            }
            if rest.starts_with('<') {
                state.advance(1);
                state.add_token(SvelteTokenType::Lt, start_pos, state.get_position());
                continue;
            }
            if rest.starts_with('>') {
                state.advance(1);
                state.add_token(SvelteTokenType::Gt, start_pos, state.get_position());
                continue;
            }

            // 3. Handle Svelte Blocks and Expressions
            if rest.starts_with(&self.config.tag_start) {
                state.advance(self.config.tag_start.len());
                // Check for special blocks like {#, {/, {:
                if let Some(next) = state.peek() {
                    match next {
                        '#' => {
                            state.advance(1);
                            state.add_token(SvelteTokenType::HashBrace, start_pos, state.get_position());
                        }
                        '/' => {
                            state.advance(1);
                            state.add_token(SvelteTokenType::SlashBrace, start_pos, state.get_position());
                        }
                        ':' => {
                            state.advance(1);
                            state.add_token(SvelteTokenType::ColonBrace, start_pos, state.get_position());
                        }
                        '@' => {
                            state.advance(1);
                            state.add_token(SvelteTokenType::At, start_pos, state.get_position());
                        }
                        _ => {
                            state.add_token(SvelteTokenType::LeftBrace, start_pos, state.get_position());
                        }
                    }
                }
                else {
                    state.add_token(SvelteTokenType::LeftBrace, start_pos, state.get_position());
                }
                continue;
            }

            if rest.starts_with(&self.config.tag_end) {
                state.advance(self.config.tag_end.len());
                state.add_token(SvelteTokenType::RightBrace, start_pos, state.get_position());
                continue;
            }

            // 4. Handle Attributes and Identifiers
            if rest.starts_with('=') {
                state.advance(1);
                state.add_token(SvelteTokenType::Eq, start_pos, state.get_position());
                continue;
            }
            if rest.starts_with(':') {
                state.advance(1);
                state.add_token(SvelteTokenType::Colon, start_pos, state.get_position());
                continue;
            }

            // 5. Handle Identifiers and Text
            if let Some(ch) = state.peek() {
                if ch.is_ascii_alphabetic() || ch == '_' {
                    while let Some(c) = state.peek() {
                        if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                            state.advance(c.len_utf8());
                        }
                        else {
                            break;
                        }
                    }
                    state.add_token(SvelteTokenType::Identifier, start_pos, state.get_position());
                    continue;
                }

                if ch == '"' || ch == '\'' {
                    let quote = ch;
                    state.advance(1);
                    while let Some(c) = state.peek() {
                        if c == quote {
                            state.advance(1);
                            break;
                        }
                        state.advance(c.len_utf8());
                    }
                    state.add_token(SvelteTokenType::StringLiteral, start_pos, state.get_position());
                    continue;
                }

                // Everything else is text
                state.advance(ch.len_utf8());
                while let Some(c) = state.peek() {
                    if c.is_whitespace() || c == '<' || c == '>' || c == '{' || c == '}' || c == '=' {
                        break;
                    }
                    state.advance(c.len_utf8());
                }
                state.add_token(SvelteTokenType::Text, start_pos, state.get_position());
            }
        }
        Ok(())
    }
}

impl<'config> Lexer<SvelteLanguage> for SvelteLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<SvelteLanguage>) -> LexOutput<SvelteLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}
