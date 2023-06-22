use crate::{ast::PythonRoot, language::PythonLanguage, parser::PythonParser};
use oak_core::{Builder, BuilderCache, OakDiagnostics, Parser, TextEdit, builder::BuildOutput, source::Source};

#[derive(Clone)]
pub struct PythonBuilder<'config> {
    config: &'config PythonLanguage,
}

impl<'config> PythonBuilder<'config> {
    pub fn new(config: &'config PythonLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<PythonLanguage> for PythonBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<PythonLanguage>) -> BuildOutput<PythonLanguage> {
        let parser = PythonParser::new(self.config);

        let mut parse_cache = oak_core::parser::session::ParseSession::<PythonLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut parse_cache);

        match parse_result.result {
            Ok(_) => {
                // Placeholder: need to implement build_root
                OakDiagnostics { result: Ok(PythonRoot { items: vec![] }), diagnostics: parse_result.diagnostics }
            }
            Err(e) => OakDiagnostics { result: Err(e), diagnostics: parse_result.diagnostics },
        }
    }
}
