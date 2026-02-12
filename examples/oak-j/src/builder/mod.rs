#![doc = include_str!("readme.md")]
use crate::{ast::*, language::JLanguage, parser::JParser};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, Parser, SourceText, TextEdit, source::Source};

/// AST builder for the J language.
#[derive(Clone)]
pub struct JBuilder<'config> {
    /// Language configuration.
    config: &'config JLanguage,
}

impl<'config> JBuilder<'config> {
    /// Creates a new J builder.
    pub fn new(config: &'config JLanguage) -> Self {
        Self { config }
    }

    /// Builds the AST root node from the syntax tree.
    pub fn build_root(&self, _green: &GreenNode<JLanguage>, _source: &SourceText) -> Result<JRoot, oak_core::OakError> {
        // Simplified AST building logic.
        Ok(JRoot::new(vec![]))
    }
}

impl<'config> Builder<JLanguage> for JBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<JLanguage>) -> oak_core::builder::BuildOutput<JLanguage> {
        let parser = JParser::new(self.config);
        let mut cache = oak_core::parser::ParseSession::<JLanguage>::default();
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
