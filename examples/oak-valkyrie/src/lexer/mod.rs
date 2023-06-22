pub use self::keywords::ValkyrieKeywords;
use crate::language::ValkyrieLanguage;
use oak_core::{Lexer, LexerCache, LexerState, lexer::LexOutput, source::Source};
mod keywords;
mod lex;

/// The lexer for the Valkyrie programming language.
#[derive(Clone)]
pub struct ValkyrieLexer<'config> {
    _config: &'config ValkyrieLanguage,
}

impl<'config> Lexer<ValkyrieLanguage> for ValkyrieLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[oak_core::TextEdit], cache: &'a mut impl LexerCache<ValkyrieLanguage>) -> LexOutput<ValkyrieLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> ValkyrieLexer<'config> {
    /// Tokenize the given source code.
    pub fn tokenize<S: Source + ?Sized>(&self, source: &S) -> impl Iterator<Item = oak_core::lexer::Token<crate::kind::ValkyrieSyntaxKind>> {
        let mut cache = oak_core::parser::session::ParseSession::<ValkyrieLanguage>::default();
        let output = self.lex(source, &[], &mut cache);
        output.result.unwrap_or_else(|_| oak_core::Arc::from_iter(Vec::new())).to_vec().into_iter()
    }
}
