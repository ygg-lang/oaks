#![doc = include_str!("readme.md")]

use crate::{
    ast::{Expression, Provide, Require, RhombusRoot},
    language::RhombusLanguage,
    parser::RhombusParser,
};
use oak_core::{
    Builder, BuilderCache, Lexer, OakDiagnostics, Parser, TextEdit,
    source::{Source, SourceText},
};

/// Builder for the Rhombus language.
#[derive(Clone)]
pub struct RhombusBuilder<'config> {
    config: &'config RhombusLanguage,
}

impl<'config> RhombusBuilder<'config> {
    /// Creates a new `RhombusBuilder` with the given configuration.
    pub fn new(config: &'config RhombusLanguage) -> Self {
        Self { config }
    }

    /// Builds a RhombusRoot AST node from a green tree and source text.
    pub fn build_root(&self, green_tree: &oak_core::GreenNode<RhombusLanguage>, source_text: &oak_core::SourceText) -> Result<crate::ast::RhombusRoot, oak_core::OakError> {
        Ok(crate::ast::RhombusRoot { expressions: vec![] })
    }
}

impl<'config> Builder<RhombusLanguage> for RhombusBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<RhombusLanguage>) -> OakDiagnostics<RhombusRoot> {
        let parser = RhombusParser::new(self.config);
        let lexer = crate::lexer::RhombusLexer::new(&self.config);

        let mut parse_cache = oak_core::parser::session::ParseSession::<RhombusLanguage>::default();
        lexer.lex(source, edits, &mut parse_cache);
        let parse_result = parser.parse(source, edits, &mut parse_cache);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
                match self.build_root(&green_tree, &source_text) {
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
