//! Builder implementation for the Racket language.

use crate::{language::RacketLanguage, parser::RacketParser};
use oak_core::{Builder, BuilderCache, Lexer, OakDiagnostics, Parser, TextEdit, source::Source};

/// Builder for the Racket language.
#[derive(Clone)]
pub struct RacketBuilder<'config> {
    config: &'config RacketLanguage,
}

impl<'config> RacketBuilder<'config> {
    /// Creates a new `RacketBuilder` with the given configuration.
    pub fn new(config: &'config RacketLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<RacketLanguage> for RacketBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<RacketLanguage>) -> OakDiagnostics<()> {
        let parser = RacketParser::new(self.config);
        let lexer = crate::lexer::RacketLexer::new(&self.config);

        lexer.lex(source, edits, cache);
        let parse_result = parser.parse(source, edits, cache);

        match parse_result.result {
            Ok(_) => OakDiagnostics { result: Ok(()), diagnostics: parse_result.diagnostics },
            Err(e) => OakDiagnostics { result: Err(e), diagnostics: parse_result.diagnostics },
        }
    }
}
