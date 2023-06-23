pub mod token_type;

use crate::{language::MermaidLanguage, lexer::token_type::MermaidTokenType};
use oak_core::{
    Lexer, LexerState,
    lexer::{LexOutput, LexerCache},
    source::{Source, TextEdit},
};

type State<'s, S> = LexerState<'s, S, MermaidLanguage>;

#[derive(Clone)]
pub struct MermaidLexer<'config> {
    _config: &'config MermaidLanguage,
}

impl<'config> Lexer<MermaidLanguage> for MermaidLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<MermaidLanguage>) -> LexOutput<MermaidLanguage> {
        let mut state: State<'_, S> = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> MermaidLexer<'config> {
    pub fn new(config: &'config MermaidLanguage) -> Self {
        Self { _config: config }
    }

    fn run<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> Result<(), oak_core::OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            // 基础 Lexer 逻辑示例
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(MermaidTokenType::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }

    fn skip_whitespace<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();
        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' || ch == '\r' || ch == '\n' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }
        if state.get_position() > start_pos {
            state.add_token(MermaidTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}
