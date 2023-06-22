use crate::{ast::PrologRoot, language::PrologLanguage, parser::PrologParser};
use oak_core::{Builder, BuilderCache, OakDiagnostics, TextEdit, parser::Parser, source::Source};

#[derive(Clone)]
pub struct PrologBuilder<'config> {
    config: &'config PrologLanguage,
}

impl<'config> PrologBuilder<'config> {
    pub fn new(config: &'config PrologLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<PrologLanguage> for PrologBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<PrologLanguage>) -> oak_core::builder::BuildOutput<PrologLanguage> {
        let parser = PrologParser::new(self.config);
        let mut cache = oak_core::parser::session::ParseSession::<PrologLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut cache);

        match parse_result.result {
            Ok(_green_tree) => {
                // Placeholder AST building
                OakDiagnostics { result: Ok(PrologRoot {}), diagnostics: parse_result.diagnostics }
            }
            Err(parse_error) => OakDiagnostics { result: Err(parse_error), diagnostics: parse_result.diagnostics },
        }
    }
}
