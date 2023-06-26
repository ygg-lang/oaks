#![doc = include_str!("readme.md")]

/// Token type definitions for COBOL lexical analysis.
///
/// This module provides [`CobolTokenType`] which defines all token types
/// recognized by the COBOL lexer, including keywords, literals, identifiers,
/// and structural tokens.
pub mod token_type;

pub use token_type::CobolTokenType;

use crate::language::CobolLanguage;
use oak_core::{Lexer, LexerCache, LexerState, OakError, lexer::LexOutput, source::Source};

/// COBOL lexer.
pub struct CobolLexer;

impl CobolLexer {
    /// Create a new COBOL lexer.
    pub fn new() -> Self {
        Self
    }
}

impl Lexer<CobolLanguage> for CobolLexer {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<CobolLanguage>) -> LexOutput<CobolLanguage> {
        let mut state = LexerState::new_with_cache(source, 0, cache);
        // Minimal implementation: just return EOF
        state.add_eof();
        state.finish_with_cache(Ok(()), cache)
    }
}
