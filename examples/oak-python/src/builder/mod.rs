mod build_expressions;
mod build_root;
mod build_statements;
mod build_utils;

use crate::{language::PythonLanguage, parser::PythonParser};
use oak_core::{Builder, BuilderCache, OakDiagnostics, Parser, SourceText, TextEdit, builder::BuildOutput, source::Source};

/// AST builder for the Python language.
pub struct PythonBuilder<'config> {
    /// Reference to the language configuration
    config: &'config PythonLanguage,
}

impl<'config> PythonBuilder<'config> {
    /// Creates a new Python builder.
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
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
                match self.build_root(green_tree, &source_text) {
                    Ok(ast_root) => OakDiagnostics { result: Ok(ast_root), diagnostics: parse_result.diagnostics },
                    Err(build_error) => {
                        let mut diagnostics = parse_result.diagnostics;
                        diagnostics.push(build_error.clone());
                        OakDiagnostics { result: Err(build_error), diagnostics }
                    }
                }
            }
            Err(e) => OakDiagnostics { result: Err(e), diagnostics: parse_result.diagnostics },
        }
    }
}
