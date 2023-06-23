#![doc = include_str!("readme.md")]
pub mod token_type;

pub use token_type::CmdTokenType;

use crate::language::CmdLanguage;
use oak_core::{Lexer, LexerCache, LexerState, OakError, lexer::LexOutput, source::Source};

type State<'a, S> = LexerState<'a, S, CmdLanguage>;

#[derive(Clone)]
pub struct CmdLexer<'config> {
    _config: &'config CmdLanguage,
}

impl<'config> Lexer<CmdLanguage> for CmdLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<CmdLanguage>) -> LexOutput<CmdLanguage> {
        let mut state = LexerState::new_with_cache(source, 0, cache);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> CmdLexer<'config> {
    pub fn new(config: &'config CmdLanguage) -> Self {
        Self { _config: config }
    }

    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(CmdTokenType::Text, start_pos, state.get_position())
            }
        }
        Ok(())
    }
}
