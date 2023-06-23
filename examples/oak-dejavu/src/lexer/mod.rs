#![doc = include_str!("readme.md")]
pub mod token_type;
use crate::language::DejavuLanguage;
pub use crate::lexer::token_type::DejavuSyntaxKind as DejavuTokenType;

use oak_core::{Lexer, LexerCache, LexerState, lexer::LexOutput, source::Source};
pub mod keywords;
pub use keywords::DejavuKeywords;
mod lex;

/// The lexer for the Dejavu programming language.
#[derive(Clone, Debug)]
pub struct DejavuLexer<'config> {
    _config: &'config DejavuLanguage,
}

impl<'config> Lexer<DejavuLanguage> for DejavuLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[oak_core::TextEdit], cache: &'a mut impl LexerCache<DejavuLanguage>) -> LexOutput<DejavuLanguage> {
        let mut state = LexerState::new_with_cache(source, 0, cache);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> DejavuLexer<'config> {
    /// Create a new lexer with the given configuration.
    pub fn new(config: &'config DejavuLanguage) -> Self {
        Self { _config: config }
    }

    /// Tokenize the given source code.
    pub fn tokenize<S: Source + ?Sized>(&self, source: &S) -> impl Iterator<Item = oak_core::lexer::Token<crate::lexer::token_type::DejavuSyntaxKind>> {
        let mut cache = oak_core::parser::session::ParseSession::<DejavuLanguage>::default();
        let output = self.lex(source, &[], &mut cache);
        output.result.unwrap_or_else(|_| oak_core::Arc::from_iter(Vec::new())).to_vec().into_iter()
    }
}
