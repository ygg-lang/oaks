use crate::{CoqParser, ast::*, language::CoqLanguage};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, SourceText, TextEdit, source::Source};

/// AST builder for the Coq language.
#[derive(Clone)]
pub struct CoqBuilder<'config> {
    /// Language configuration.
    config: &'config CoqLanguage,
}

impl<'config> CoqBuilder<'config> {
    /// Creates a new Coq builder.
    pub fn new(config: &'config CoqLanguage) -> Self {
        Self { config }
    }

    /// Builds the AST root from the green tree.
    pub fn build_root(&self, _green: &GreenNode<CoqLanguage>, _source: &SourceText) -> Result<CoqRoot, oak_core::OakError> {
        // Simplified AST building logic
        Ok(CoqRoot::new())
    }
}

impl<'config> Builder<CoqLanguage> for CoqBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<CoqLanguage>) -> oak_core::builder::BuildOutput<CoqLanguage> {
        let parser = CoqParser::new(self.config);
        let lexer = crate::lexer::CoqLexer::new(&self.config);

        let mut cache = oak_core::parser::session::ParseSession::<CoqLanguage>::default();
        let parse_result = oak_core::parser::parse(&parser, &lexer, source, edits, &mut cache);

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
