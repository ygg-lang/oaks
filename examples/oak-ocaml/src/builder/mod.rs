use crate::{ast::OCamlRoot, language::OCamlLanguage, parser::OCamlParser};
use oak_core::{Builder, BuilderCache, Lexer, OakDiagnostics, Parser, TextEdit, source::Source};

#[derive(Clone)]
pub struct OCamlBuilder<'config> {
    config: &'config OCamlLanguage,
}

impl<'config> OCamlBuilder<'config> {
    pub fn new(config: &'config OCamlLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<OCamlLanguage> for OCamlBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<OCamlLanguage>) -> OakDiagnostics<OCamlRoot> {
        let parser = OCamlParser::new(self.config);
        let lexer = crate::lexer::OCamlLexer::new(self.config);

        let mut session = oak_core::parser::session::ParseSession::<OCamlLanguage>::default();
        lexer.lex(source, edits, &mut session);
        let parse_result = parser.parse(source, edits, &mut session);

        match parse_result.result {
            Ok(_green_tree) => {
                let ast_root = OCamlRoot { items: vec![] };
                OakDiagnostics { result: Ok(ast_root), diagnostics: parse_result.diagnostics }
            }
            Err(parse_error) => OakDiagnostics { result: Err(parse_error), diagnostics: parse_result.diagnostics },
        }
    }
}
