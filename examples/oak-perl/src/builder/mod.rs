use crate::{ast::PerlRoot, language::PerlLanguage, parser::PerlParser};
use oak_core::{Builder, BuilderCache, OakDiagnostics, Parser, Source, SourceText, TextEdit, parser::session::ParseSession};

pub struct PerlBuilder<'config> {
    config: &'config PerlLanguage,
}

impl<'config> PerlBuilder<'config> {
    pub fn new(config: &'config PerlLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<PerlLanguage> for PerlBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<PerlLanguage>) -> OakDiagnostics<PerlRoot> {
        let parser = PerlParser::new(self.config);

        let mut parse_cache = ParseSession::<PerlLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut parse_cache);

        match parse_result.result {
            Ok(_green_tree) => {
                let _source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
                // Placeholder implementation
                let ast_root = PerlRoot { items: Vec::new() };
                OakDiagnostics { result: Ok(ast_root), diagnostics: parse_result.diagnostics }
            }
            Err(e) => OakDiagnostics { result: Err(e), diagnostics: parse_result.diagnostics },
        }
    }
}
