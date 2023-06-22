use crate::{ast::*, language::JasminLanguage, parser::JasminParser};
use oak_core::{Builder, BuilderCache, GreenNode, Lexer, OakError, Parser, SourceText, TextEdit, source::Source};

pub struct JasminBuilder<'config> {
    config: &'config JasminLanguage,
}

impl<'config> JasminBuilder<'config> {
    pub fn new(config: &'config JasminLanguage) -> Self {
        Self { config }
    }

    fn build_root(&self, _green_tree: &GreenNode<JasminLanguage>, _source: &SourceText) -> Result<JasminRoot, OakError> {
        // Simple implementation for now
        Ok(JasminRoot { class: JasminClass { modifiers: vec![], name: "Unknown".to_string(), version: None, methods: vec![], fields: vec![], source_file: None } })
    }
}

impl<'config> Builder<JasminLanguage> for JasminBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<JasminLanguage>) -> oak_core::builder::BuildOutput<JasminLanguage> {
        let parser = JasminParser::new(self.config);
        let lexer = crate::lexer::JasminLexer::new(&self.config);
        let mut parse_cache = oak_core::parser::session::ParseSession::<JasminLanguage>::default();
        lexer.lex(source, edits, &mut parse_cache);
        let parse_result = parser.parse(source, edits, &mut parse_cache);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
                match self.build_root(green_tree, &source_text) {
                    Ok(ast_root) => oak_core::errors::OakDiagnostics { result: Ok(ast_root), diagnostics: parse_result.diagnostics },
                    Err(e) => oak_core::errors::OakDiagnostics { result: Err(e.clone()), diagnostics: vec![e] },
                }
            }
            Err(e) => oak_core::errors::OakDiagnostics { result: Err(e.clone()), diagnostics: vec![e] },
        }
    }
}
