//! D language AST builder
use crate::language::DLanguage;
use oak_core::{Builder, BuilderCache, GreenNode, OakError, Parser, RedNode, SourceText, TextEdit, source::Source};

/// D language AST builder
#[derive(Clone)]
pub struct DBuilder<'config> {
    config: &'config DLanguage,
}

impl<'config> DBuilder<'config> {
    /// Create a new D builder
    pub fn new(config: &'config DLanguage) -> Self {
        Self { config }
    }

    fn build_root(&self, green_tree: GreenNode<DLanguage>, _source: &SourceText) -> Result<crate::ast::DRoot, OakError> {
        let _red_root = RedNode::new(&green_tree, 0);
        // Basic implementation, can be expanded later
        Ok(crate::ast::DRoot { items: Vec::new() })
    }
}

impl<'config> Builder<DLanguage> for DBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<DLanguage>) -> oak_core::builder::BuildOutput<DLanguage> {
        let parser = crate::parser::DParser::new(self.config);
        let mut parse_cache = oak_core::parser::session::ParseSession::<DLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut parse_cache);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()));
                match self.build_root(green_tree.clone(), &source_text) {
                    Ok(ast_root) => oak_core::builder::BuildOutput::<DLanguage> { result: Ok(ast_root), diagnostics: parse_result.diagnostics },
                    Err(build_error) => {
                        let mut diagnostics = parse_result.diagnostics;
                        diagnostics.push(build_error.clone());
                        oak_core::builder::BuildOutput::<DLanguage> { result: Err(build_error), diagnostics }
                    }
                }
            }
            Err(parse_error) => oak_core::builder::BuildOutput::<DLanguage> { result: Err(parse_error), diagnostics: parse_result.diagnostics },
        }
    }
}
