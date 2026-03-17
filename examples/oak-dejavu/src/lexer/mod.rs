use crate::DejavuLanguage;
use oak_core::{
    Source, TextEdit,
    lexer::{LexOutput, Lexer, LexerCache, LexerState},
};

pub use token_type::DejavuTokenType;

/// Dejavu lexer.
pub struct DejavuLexer<'config> {
    config: &'config DejavuLanguage,
}

impl<'config> DejavuLexer<'config> {
    /// Create a new Dejavu lexer.
    pub fn new(config: &'config DejavuLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Lexer<DejavuLanguage> for DejavuLexer<'config> {
    fn lex<'b, S: Source + ?Sized>(&self, text: &'b S, edits: &[TextEdit], cache: &'b mut impl LexerCache<DejavuLanguage>) -> LexOutput<DejavuLanguage> {
        let relex_from = if edits.is_empty() { text.length() } else { edits.iter().map(|e| e.span.start).min().unwrap_or(0) };

        let mut state = LexerState::new_with_cache(text, relex_from, cache);
        let result = self.run(&mut state);

        state.finish_with_cache(result, cache)
    }
}

/// Keyword definitions for Dejavu.
pub mod keywords;
/// Lexer implementation.
pub mod lex;
/// Token type definitions.
pub mod token_type;

pub use keywords::DejavuKeywords;
