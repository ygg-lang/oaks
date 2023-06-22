use crate::{ast::PhpRoot, language::PhpLanguage, lexer::PhpLexer, parser::PhpParser};
use oak_core::{Builder, BuilderCache, Lexer, OakDiagnostics, Parser, TextEdit, source::Source};

pub struct PhpBuilder<'config> {
    config: &'config PhpLanguage,
}

impl<'config> PhpBuilder<'config> {
    pub fn new(config: &'config PhpLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<PhpLanguage> for PhpBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<PhpLanguage>) -> oak_core::builder::BuildOutput<PhpLanguage> {
        let parser = PhpParser::new(self.config);
        let lexer = PhpLexer::new(self.config);
        let mut session = oak_core::parser::session::ParseSession::<PhpLanguage>::default();
        lexer.lex(source, edits, &mut session);
        let parse_result = parser.parse(source, edits, &mut session);

        match parse_result.result {
            Ok(_) => {
                // Placeholder for AST building
                OakDiagnostics { result: Ok(PhpRoot { items: vec![] }), diagnostics: parse_result.diagnostics }
            }
            Err(e) => OakDiagnostics { result: Err(e), diagnostics: parse_result.diagnostics },
        }
    }
}
