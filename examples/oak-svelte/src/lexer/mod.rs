#![doc = include_str!("readme.md")]
use oak_core::{
    Source,
    lexer::{LexOutput, Lexer, LexerCache, LexerState},
};
/// Token types for Svelte.
pub mod token_type;

use crate::lexer::token_type::{SvelteLanguage, SvelteTokenType};

/// Svelte Lexer
#[derive(Clone, Debug)]
pub struct SvelteLexer<'config> {
    _config: &'config SvelteLanguage,
}

type State<'a, S> = LexerState<'a, S, SvelteLanguage>;

impl<'config> SvelteLexer<'config> {
    /// Create a new Svelte lexer
    pub fn new(config: &'config SvelteLanguage) -> Self {
        Self { _config: config }
    }

    fn lex_token<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        if self.lex_whitespace(state) {
            return;
        }
        if self.lex_comment(state) {
            return;
        }

        let start_pos = state.get_position();
        let ch = match state.peek() {
            Some(c) => c,
            None => {
                state.add_token(SvelteTokenType::Error, start_pos, start_pos);
                return;
            }
        };

        match ch {
            '{' => {
                state.advance(1);
                state.add_token(SvelteTokenType::OpenBrace, start_pos, start_pos + 1);
            }
            '}' => {
                state.advance(1);
                state.add_token(SvelteTokenType::CloseBrace, start_pos, start_pos + 1);
            }
            '#' => {
                state.advance(1);
                state.add_token(SvelteTokenType::Hash, start_pos, start_pos + 1);
            }
            '/' => {
                if state.peek_next_n(1) == Some('>') {
                    state.advance(2);
                    state.add_token(SvelteTokenType::TagSelfClose, start_pos, start_pos + 2);
                }
                else {
                    state.advance(1);
                    state.add_token(SvelteTokenType::Slash, start_pos, start_pos + 1);
                }
            }
            ':' => {
                state.advance(1);
                state.add_token(SvelteTokenType::Colon, start_pos, start_pos + 1);
            }
            '@' => {
                state.advance(1);
                state.add_token(SvelteTokenType::At, start_pos, start_pos + 1);
            }
            '<' => {
                if state.peek_next_n(1) == Some('/') {
                    state.advance(2);
                    state.add_token(SvelteTokenType::TagEndOpen, start_pos, start_pos + 2);
                }
                else {
                    state.advance(1);
                    state.add_token(SvelteTokenType::TagOpen, start_pos, start_pos + 1);
                }
            }
            '>' => {
                state.advance(1);
                state.add_token(SvelteTokenType::TagClose, start_pos, start_pos + 1);
            }
            '=' => {
                state.advance(1);
                state.add_token(SvelteTokenType::Eq, start_pos, start_pos + 1);
            }
            '"' | '\'' => {
                self.lex_string(state);
            }
            _ if ch.is_ascii_alphabetic() || ch == '_' => {
                self.lex_identifier(state);
            }
            _ => {
                state.advance(ch.len_utf8());
                state.add_token(SvelteTokenType::Text, start_pos, state.get_position());
            }
        }
    }

    fn lex_whitespace<'a, S: Source + ?Sized>(&self, state: &mut LexerState<'a, S, SvelteLanguage>) -> bool {
        let range = state.skip_ascii_whitespace();
        if range.start < range.end {
            state.add_token(SvelteTokenType::Whitespace, range.start, range.end);
            true
        }
        else {
            false
        }
    }

    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();
        if state.rest().starts_with("<!--") {
            state.advance(4);
            while state.not_at_end() && !state.rest().starts_with("-->") {
                state.advance(1);
            }
            if state.rest().starts_with("-->") {
                state.advance(3);
            }
            state.add_token(SvelteTokenType::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let start_pos = state.get_position();
        let quote = state.peek().unwrap();
        state.advance(1);
        while let Some(ch) = state.peek() {
            if ch == quote {
                state.advance(1);
                break;
            }
            state.advance(ch.len_utf8());
        }
        state.add_token(SvelteTokenType::StringLiteral, start_pos, state.get_position());
    }

    fn lex_identifier<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let start_pos = state.get_position();
        while let Some(ch) = state.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' || ch == ':' {
                state.advance(1);
            }
            else {
                break;
            }
        }
        state.add_token(SvelteTokenType::Identifier, start_pos, state.get_position());
    }
}

impl<'config> Lexer<SvelteLanguage> for SvelteLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<SvelteLanguage>) -> LexOutput<SvelteLanguage> {
        let mut state = LexerState::new_with_cache(source, 0, cache);
        while state.not_at_end() {
            self.lex_token(&mut state);
        }
        state.add_eof();
        state.finish_with_cache(Ok(()), cache)
    }
}
