#![doc = include_str!("readme.md")]
use crate::{language::MatlabLanguage, parser::MatlabParser};
use oak_core::{Builder, BuilderCache, Lexer, OakDiagnostics, Parser, TextEdit, source::Source};

/// Matlab language AST builder (parse-only; typed root is unused).
#[derive(Clone)]
pub struct MatlabBuilder<'config> {
    /// The Matlab language configuration.
    config: &'config MatlabLanguage,
}

impl<'config> MatlabBuilder<'config> {
    /// Creates a new `MatlabBuilder` with the given configuration.
    pub fn new(config: &'config MatlabLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<MatlabLanguage> for MatlabBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<MatlabLanguage>) -> OakDiagnostics<()> {
        let parser = MatlabParser::new(self.config);
        let lexer = crate::lexer::MatlabLexer::new(self.config);

        lexer.lex(source, edits, cache);
        let parse_result = parser.parse(source, edits, cache);

        match parse_result.result {
            Ok(_) => OakDiagnostics { result: Ok(()), diagnostics: parse_result.diagnostics },
            Err(e) => OakDiagnostics { result: Err(e), diagnostics: parse_result.diagnostics },
        }
    }
}
