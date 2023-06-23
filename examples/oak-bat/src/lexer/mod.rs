#![doc = include_str!("readme.md")]
pub mod token_type;

pub use token_type::BatTokenType;

use crate::language::BatLanguage;
use oak_core::{Lexer, LexerCache, LexerState, OakError, lexer::LexOutput, source::Source};

type State<'a, S> = LexerState<'a, S, BatLanguage>;

#[derive(Clone)]
pub struct BatLexer<'config> {
    _config: &'config BatLanguage,
}

impl<'config> Lexer<BatLanguage> for BatLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<BatLanguage>) -> LexOutput<BatLanguage> {
        let mut state = LexerState::new_with_cache(source, 0, cache);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> BatLexer<'config> {
    pub fn new(config: &'config BatLanguage) -> Self {
        Self { _config: config }
    }

    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(BatTokenType::Text, start_pos, state.get_position())
            }
        }
        Ok(())
    }
}
