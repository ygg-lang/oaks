use crate::{ast::*, language::SassLanguage, parser::SassParser};
use oak_core::{Builder, BuilderCache, OakDiagnostics, OakError, Parser, SourceText, TextEdit, builder::BuildOutput, source::Source, tree::GreenNode};

/// AST builder for the Sass language.
#[derive(Clone)]
pub struct SassBuilder<'config> {
    config: &'config SassLanguage,
}

impl<'config> SassBuilder<'config> {
    /// Creates a new Sass builder with the given configuration.
    pub fn new(config: &'config SassLanguage) -> Self {
        Self { config }
    }

    /// Builds the Sass AST root from a green tree.
    fn build_root<'a>(&self, _green_tree: &'a GreenNode<'a, SassLanguage>, _source: &SourceText) -> Result<SassRoot, OakError> {
        Ok(SassRoot { span: (0..0).into() })
    }
}

impl<'config> Builder<SassLanguage> for SassBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<SassLanguage>) -> BuildOutput<SassLanguage> {
        let parser = SassParser::new(self.config);
        let mut parse_cache = oak_core::parser::ParseSession::<SassLanguage>::default();
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
            Err(parse_error) => OakDiagnostics { result: Err(parse_error), diagnostics: parse_result.diagnostics },
        }
    }
}
