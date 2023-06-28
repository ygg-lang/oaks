use crate::{LessParser, ast::LessRoot, language::LessLanguage};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, Parser, RedNode, SourceText, TextEdit, builder::BuildOutput, source::Source};

/// A builder for Less documents that coordinates parsing and AST construction.
#[derive(Clone)]
pub struct LessBuilder {
    config: LessLanguage,
}

impl LessBuilder {
    /// Creates a new `LessBuilder` with the specified language configuration.
    pub fn new(config: LessLanguage) -> Self {
        Self { config }
    }
}

impl Builder<LessLanguage> for LessBuilder {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<LessLanguage>) -> BuildOutput<LessLanguage> {
        let parser = LessParser::new(&self.config);

        let mut parse_cache = oak_core::parser::session::ParseSession::<LessLanguage>::default();
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

impl LessBuilder {
    pub(crate) fn build_root<'a>(&self, green_tree: &'a GreenNode<'a, LessLanguage>, _source: &SourceText) -> LessRoot {
        let _red_root = RedNode::new(green_tree, 0);
        // Simplified AST building for now
        LessRoot { nodes: Vec::new() }
    }
}
