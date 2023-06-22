use crate::{HtmlParser, ast::HtmlDocument, language::HtmlLanguage};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, Parser, RedNode, SourceText, TextEdit, builder::BuildOutput, source::Source};

/// HTML Builder
#[derive(Clone)]
pub struct HtmlBuilder<'config> {
    config: &'config HtmlLanguage,
}

impl<'config> HtmlBuilder<'config> {
    pub fn new(config: &'config HtmlLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<HtmlLanguage> for HtmlBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<HtmlLanguage>) -> BuildOutput<HtmlLanguage> {
        let parser = HtmlParser::new(self.config);

        let mut parse_cache = oak_core::parser::session::ParseSession::<HtmlLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut parse_cache);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()));
                let ast_root = self.build_root(green_tree, &source_text);
                OakDiagnostics { result: Ok(ast_root), diagnostics: parse_result.diagnostics }
            }
            Err(parse_error) => OakDiagnostics { result: Err(parse_error), diagnostics: parse_result.diagnostics },
        }
    }
}

impl<'config> HtmlBuilder<'config> {
    pub(crate) fn build_root<'a>(&self, green_tree: &'a GreenNode<'a, HtmlLanguage>, _source: &SourceText) -> HtmlDocument {
        let _red_root = RedNode::new(green_tree, 0);
        // Simplified AST building for now
        HtmlDocument { nodes: Vec::new() }
    }
}
