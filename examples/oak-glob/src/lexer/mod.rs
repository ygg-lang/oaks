pub mod token_type;

use oak_core::{
    lexer::{LexOutput, Lexer, LexerCache, LexerState},
    source::{Source, TextEdit},
};

use crate::{language::GlobLanguage, lexer::token_type::GlobTokenType};

/// Lexer for glob pattern syntax.
pub struct GlobLexer;

impl Lexer<GlobLanguage> for GlobLexer {
    fn lex<'a, S: Source + ?Sized>(&self, text: &S, edits: &[TextEdit], cache: &'a mut impl LexerCache<GlobLanguage>) -> LexOutput<GlobLanguage> {
        let mut state = LexerState::new_with_cache(text, text.length(), cache);

        while state.not_at_end() {
            let safe_point = state.get_position();
            let current = state.get_position();

            match state.current() {
                Some('#') => {
                    // Comment
                    let start = current;
                    while state.not_at_end() && state.current() != Some('\n') {
                        let _ = state.bump();
                    }
                    let end = state.get_position();
                    state.add_token(GlobTokenType::Comment, start, end);
                }
                Some(' ') | Some('\t') | Some('\r') | Some('\n') => {
                    // Whitespace
                    let start = current;
                    while state.not_at_end() {
                        match state.current() {
                            Some(' ') | Some('\t') | Some('\r') | Some('\n') => {
                                let _ = state.bump();
                            }
                            _ => break,
                        }
                    }
                    let end = state.get_position();
                    state.add_token(GlobTokenType::Whitespace, start, end);
                }
                Some(_) => {
                    // Rule
                    let start = current;
                    while state.not_at_end() && state.current() != Some('\n') {
                        let _ = state.bump();
                    }
                    let end = state.get_position();
                    state.add_token(GlobTokenType::Rule, start, end);
                }
                None => break,
            }

            state.advance_if_dead_lock(safe_point);
        }

        state.add_eof();
        state.finish_with_cache(Ok(()), cache)
    }
}

impl Default for GlobLexer {
    fn default() -> Self {
        Self
    }
}
