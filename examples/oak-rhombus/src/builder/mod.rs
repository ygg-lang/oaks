//! Builder implementation for the Rhombus language.

use crate::{language::RhombusLanguage, parser::RhombusParser};
use oak_core::{Builder, BuilderCache, Lexer, OakDiagnostics, Parser, TextEdit, source::Source};

/// Builder for the Rhombus language.
#[derive(Clone)]
pub struct RhombusBuilder<'config> {
    config: &'config RhombusLanguage,
}

impl<'config> RhombusBuilder<'config> {
    /// Creates a new `RhombusBuilder` with the given configuration.
    pub fn new(config: &'config RhombusLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<RhombusLanguage> for RhombusBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<RhombusLanguage>) -> OakDiagnostics<()> {
        let parser = RhombusParser::new(self.config);
        let lexer = crate::lexer::RhombusLexer::new(&self.config);

        lexer.lex(source, edits, cache);
        let parse_result = parser.parse(source, edits, cache);

        match parse_result.result {
            Ok(_) => OakDiagnostics { result: Ok(()), diagnostics: parse_result.diagnostics },
            Err(e) => OakDiagnostics { result: Err(e), diagnostics: parse_result.diagnostics },
        }
    }
}
