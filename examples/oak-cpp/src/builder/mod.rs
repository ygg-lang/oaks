#![doc = include_str!("readme.md")]
use crate::{ast::*, language::CppLanguage, parser::CppParser};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, Parser, SourceText, TextEdit, source::Source};

/// AST builder for the C++ language
#[derive(Clone)]
pub struct CppBuilder<'config> {
    /// Language configuration
    config: &'config CppLanguage,
}

impl<'config> CppBuilder<'config> {
    /// Creates a new C++ builder
    pub fn new(config: &'config CppLanguage) -> Self {
        Self { config }
    }

    /// Builds the AST root node from the green tree
    pub fn build_root(&self, _green: &GreenNode<CppLanguage>, _source: &SourceText) -> Result<CppRoot, oak_core::OakError> {
        // Simplified AST building logic, currently serving as a framework
        Ok(CppRoot { translation_unit: TranslationUnit { external_declarations: vec![], span: (0.._source.length()).into() } })
    }
}

impl<'config> Builder<CppLanguage> for CppBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<CppLanguage>) -> oak_core::builder::BuildOutput<CppLanguage> {
        let parser = CppParser::new(self.config);
        let mut cache = oak_core::parser::ParseSession::<CppLanguage>::default();
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
