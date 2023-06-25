#![doc = include_str!("readme.md")]
use crate::{ast::*, language::ElmLanguage, parser::ElmParser};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, Parser, SourceText, TextEdit, source::Source};

/// AST builder for the Elm language
#[derive(Clone)]
pub struct ElmBuilder<'config> {
    /// Language configuration
    config: &'config ElmLanguage,
}

impl<'config> ElmBuilder<'config> {
    /// Creates a new Elm builder
    pub fn new(config: &'config ElmLanguage) -> Self {
        Self { config }
    }

    /// Builds the AST root node from the syntax tree
    pub fn build_root(&self, _green: &GreenNode<ElmLanguage>, _source: &SourceText) -> Result<ElmRoot, oak_core::OakError> {
        // Simplified AST construction logic
        Ok(ElmRoot { items: vec![] })
    }
}

impl<'config> Builder<ElmLanguage> for ElmBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<ElmLanguage>) -> oak_core::builder::BuildOutput<ElmLanguage> {
        let parser = ElmParser::new(self.config);
        let mut cache = oak_core::parser::ParseSession::<ElmLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut cache);

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
