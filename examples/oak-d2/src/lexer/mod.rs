pub mod token_type;

use crate::language::D2Language;
use oak_core::{
    Lexer, LexerCache, LexerState,
    lexer::LexOutput,
    source::{Source, TextEdit},
};

pub struct D2Lexer<'config> {
    config: &'config D2Language,
}

impl<'config> D2Lexer<'config> {
    pub fn new(config: &'config D2Language) -> Self {
        Self { config }
    }
}

impl<'config> Lexer<D2Language> for D2Lexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<D2Language>) -> LexOutput<D2Language> {
        let mut state = LexerState::new(source);
        // Minimal lexing implementation
        while state.not_at_end() {
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
            }
        }
        state.add_eof();
        state.finish_with_cache(Ok(()), cache)
    }
}
