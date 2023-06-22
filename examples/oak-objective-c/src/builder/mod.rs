use crate::language::ObjectiveCLanguage;
use oak_core::{Builder, BuilderCache, OakDiagnostics, TextEdit, lexer::Lexer, parser::Parser, source::Source};

pub struct ObjectiveCBuilder<'config> {
    config: &'config ObjectiveCLanguage,
}

impl<'config> ObjectiveCBuilder<'config> {
    pub fn new(config: &'config ObjectiveCLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<ObjectiveCLanguage> for ObjectiveCBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[TextEdit], _cache: &'a mut impl BuilderCache<ObjectiveCLanguage>) -> oak_core::builder::BuildOutput<ObjectiveCLanguage> {
        // Simple implementation for now
        let mut session = oak_core::parser::session::ParseSession::<ObjectiveCLanguage>::default();
        let lexer = crate::lexer::ObjectiveCLexer::new(self.config);
        let parser = crate::parser::ObjectiveCParser::new(self.config);

        lexer.lex(source, &[], &mut session);
        let parse_result = parser.parse(source, &[], &mut session);

        let result = parse_result.result.map(|_| crate::ast::ObjectiveCRoot::default());

        OakDiagnostics { result, diagnostics: parse_result.diagnostics }
    }
}
