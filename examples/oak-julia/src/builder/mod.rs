use crate::{JuliaLanguage, ast::JuliaRoot, lexer::JuliaLexer, parser::JuliaParser};
use oak_core::{Builder, BuilderCache, OakDiagnostics, TextEdit, source::Source};

pub struct JuliaBuilder<'config> {
    config: &'config JuliaLanguage,
}

impl<'config> JuliaBuilder<'config> {
    pub fn new(config: &'config JuliaLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<JuliaLanguage> for JuliaBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<JuliaLanguage>) -> OakDiagnostics<JuliaRoot> {
        let parser = JuliaParser::new(self.config);
        let lexer = JuliaLexer::new(self.config);

        let mut cache = oak_core::parser::session::ParseSession::<JuliaLanguage>::default();
        let parse_result = oak_core::parser::parse(&parser, &lexer, source, edits, &mut cache);

        match parse_result.result {
            Ok(_green_tree) => {
                // TODO: Implement actual AST building
                OakDiagnostics { result: Ok(JuliaRoot { statements: Vec::new() }), diagnostics: parse_result.diagnostics }
            }
            Err(parse_error) => OakDiagnostics { result: Err(parse_error), diagnostics: parse_result.diagnostics },
        }
    }
}
