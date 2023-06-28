use crate::{CssParser, ast::CssRoot, language::CssLanguage};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, Parser, RedNode, SourceText, TextEdit, builder::BuildOutput, source::Source};

/// A builder for CSS documents that coordinates parsing and AST construction.
#[derive(Clone)]
pub struct CssBuilder {
    config: CssLanguage,
}

impl CssBuilder {
    /// Creates a new `CssBuilder` with the specified language configuration.
    pub fn new(config: CssLanguage) -> Self {
        Self { config }
    }
}

impl Builder<CssLanguage> for CssBuilder {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<CssLanguage>) -> BuildOutput<CssLanguage> {
        let parser = CssParser::new(&self.config);

        let mut parse_cache = oak_core::parser::session::ParseSession::<CssLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut parse_cache);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
                let ast_root = self.build_root(green_tree, &source_text);
                OakDiagnostics { result: Ok(ast_root), diagnostics: parse_result.diagnostics }
            }
            Err(parse_error) => OakDiagnostics { result: Err(parse_error), diagnostics: parse_result.diagnostics },
        }
    }
}

impl CssBuilder {
    pub(crate) fn build_root<'a>(&self, green_tree: &'a GreenNode<'a, CssLanguage>, _source: &SourceText) -> CssRoot {
        let _red_root = RedNode::new(green_tree, 0);
        // Simplified AST building for now
        CssRoot { nodes: Vec::new() }
    }
}
