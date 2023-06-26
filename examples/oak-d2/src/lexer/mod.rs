/// Token type definitions for D2.
pub mod token_type;

use crate::{D2TokenType, language::D2Language};
use oak_core::{
    Lexer, LexerCache, LexerState,
    lexer::LexOutput,
    source::{Source, TextEdit},
};

/// Lexer for D2 diagram language.
pub struct D2Lexer<'config> {
    config: &'config D2Language,
}

impl<'config> D2Lexer<'config> {
    /// Creates a new D2Lexer with the given language configuration.
    pub fn new(config: &'config D2Language) -> Self {
        Self { config }
    }
}

impl<'config> Lexer<D2Language> for D2Lexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<D2Language>) -> LexOutput<D2Language> {
        let mut state = LexerState::new(source);

        while state.not_at_end() {
            let start = state.get_position();

            match state.peek() {
                Some(' ') | Some('\t') => {
                    // Whitespace
                    while state.peek().map_or(false, |c| c == ' ' || c == '\t') {
                        state.advance(1);
                    }
                    let end = state.get_position();
                    state.add_token(D2TokenType::Whitespace, start, end);
                }
                Some('\n') | Some('\r') => {
                    // Newline
                    if state.peek() == Some('\r') {
                        state.advance(1);
                    }
                    if state.peek() == Some('\n') {
                        state.advance(1);
                    }
                    let end = state.get_position();
                    state.add_token(D2TokenType::Newline, start, end);
                }
                Some('#') => {
                    // Comment
                    while state.peek().map_or(false, |c| c != '\n' && c != '\r') {
                        state.advance(1);
                    }
                    let end = state.get_position();
                    state.add_token(D2TokenType::Comment, start, end);
                }
                Some(':') => {
                    // Colon
                    state.advance(1);
                    let end = state.get_position();
                    state.add_token(D2TokenType::Colon, start, end);
                }
                Some('-') if state.peek_next_n(1) == Some('>') => {
                    // Arrow
                    state.advance(2);
                    let end = state.get_position();
                    state.add_token(D2TokenType::Arrow, start, end);
                }
                Some('{') => {
                    // Left brace
                    state.advance(1);
                    let end = state.get_position();
                    state.add_token(D2TokenType::LeftBrace, start, end);
                }
                Some('}') => {
                    // Right brace
                    state.advance(1);
                    let end = state.get_position();
                    state.add_token(D2TokenType::RightBrace, start, end);
                }
                Some(c) if c.is_alphabetic() || c == '_' => {
                    // Identifier
                    while state.peek().map_or(false, |c| c.is_alphanumeric() || c == '_') {
                        state.advance(1);
                    }
                    let end = state.get_position();
                    state.add_token(D2TokenType::Id, start, end);
                }
                Some('"') => {
                    // Label
                    state.advance(1); // Skip opening quote
                    while state.peek().map_or(false, |c| c != '"') {
                        state.advance(1);
                    }
                    if state.peek() == Some('"') {
                        state.advance(1); // Skip closing quote
                    }
                    let end = state.get_position();
                    state.add_token(D2TokenType::Label, start, end);
                }
                _ => {
                    // Error or unknown character
                    state.advance(1);
                    let end = state.get_position();
                    state.add_token(D2TokenType::Error, start, end);
                }
            }
        }

        state.add_eof();
        state.finish_with_cache(Ok(()), cache)
    }
}
